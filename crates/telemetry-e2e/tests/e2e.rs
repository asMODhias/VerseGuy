use anyhow::Result;
use opentelemetry::trace::Tracer;
use opentelemetry::{global, KeyValue, sdk::trace as sdktrace, Resource};
use opentelemetry_otlp::WithExportConfig;
use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

#[tokio::test]
async fn otlp_trace_reaches_jaeger() -> Result<()> {
    let otlp_endpoint =
        std::env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:4317".into());
    let jaeger_query =
        std::env::var("JAEGER_QUERY_URL").unwrap_or_else(|_| "http://127.0.0.1:16686".into());

    // Unique id to search for in Jaeger
    let test_id = Uuid::new_v4().to_string();

    // Build OTLP exporter -> Collector
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otlp_endpoint.clone());

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "telemetry-e2e",
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;
    let tracer = global::tracer("telemetry-e2e");

    // Create a span with our test tag and attempt a couple times to ensure exporter receives it
    for i in 0..3 {
        let mut span = tracer.start("test-span");
        span.set_attribute(KeyValue::new("test_id", test_id.clone()));
        drop(span);
        // small ramp between attempts
        tokio::time::sleep(std::time::Duration::from_millis(200 * (i + 1))).await;
    }

    // Ensure exporter flushes
    global::shutdown_tracer_provider();

    // Poll Jaeger query API for the trace containing our test_id with exponential backoff + jitter
    let client = Client::new();
    let attempts: u32 = std::env::var("OTLP_POLL_ATTEMPTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);
    let initial_backoff_ms: u64 = std::env::var("OTLP_BACKOFF_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    let max_backoff_ms: u64 = std::env::var("OTLP_MAX_BACKOFF_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5000);

    let start = std::time::Instant::now();
    let mut found = false;
    let mut last_body: Option<String> = None;
    let mut attempts_used: Option<u32> = None;
    for attempt in 0..attempts {
        // Small readiness check for Jaeger
        if let Ok(resp) = client.get(format!("{}/api/services", jaeger_query)).send().await {
            if !resp.status().is_success() {
                eprintln!("Jaeger /api/services returned status {}", resp.status());
            }
        } else {
            eprintln!("Failed to reach Jaeger /api/services on attempt {}", attempt);
        }

        let url = format!("{}/api/traces?service={}&limit=20", jaeger_query, "telemetry-e2e");
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<Value>().await {
                    if let Some(arr) = json.get("data").and_then(|d| d.as_array()) {
                        'outer: for trace in arr {
                            if let Some(spans) = trace.get("spans").and_then(|s| s.as_array()) {
                                for span in spans {
                                    if let Some(tags) = span.get("tags").and_then(|t| t.as_array()) {
                                        for tag in tags {
                                            if tag.get("key").and_then(|k| k.as_str()) == Some("test_id") {
                                                if tag.get("value").and_then(|v| v.as_str()) == Some(&test_id) {
                                                    found = true;
                                                    attempts_used = Some(attempt + 1);
                                                    break 'outer;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Ok(text) = resp.text().await {
                    last_body = Some(text);
                }
            } else if let Ok(text) = resp.text().await {
                last_body = Some(text);
            }
        } else {
            eprintln!("Failed to query Jaeger trace API on attempt {}", attempt);
        }

        if found {
            break;
        }

        // exponential backoff with simple jitter
        let exp = 1u64.saturating_shl(std::cmp::min(attempt, 10) as u32);
        let mut sleep_ms = initial_backoff_ms.saturating_mul(exp);
        if sleep_ms > max_backoff_ms {
            sleep_ms = max_backoff_ms;
        }
        let jitter = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() % 1000) as u64;
        let sleep_ms = sleep_ms + jitter;
        tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;
    }

    let duration_ms = start.elapsed().as_millis();
    let metrics = serde_json::json!({
        "test_id": test_id,
        "found": found,
        "attempts": attempts_used.unwrap_or(attempts),
        "duration_ms": duration_ms,
        "last_body": last_body,
        "timestamp_unix": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    });
    let metrics_path = std::env::var("TELEMETRY_METRICS_FILE").unwrap_or_else(|_| "telemetry_e2e_metrics.json".into());
    if let Err(e) = std::fs::write(&metrics_path, serde_json::to_string_pretty(&metrics).unwrap()) {
        eprintln!("Failed to write metrics file {}: {}", metrics_path, e);
    } else {
        eprintln!("Wrote metrics to {}", metrics_path);
    }

    assert!(found, "Trace with test_id {} not found in Jaeger, last body: {:?}", test_id, last_body);
    Ok(())
}
