use anyhow::Result;
use opentelemetry::trace::{Tracer, Span};
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{trace as sdktrace, Resource};
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
    // Verbose debug: set TELEMETRY_DEBUG=1 to show all steps
    let debug = std::env::var("TELEMETRY_DEBUG").is_ok();
    if debug {
        eprintln!("TEST DEBUG ENABLED");
        eprintln!("OTLP_ENDPOINT={} JAEGER_QUERY_URL={}", otlp_endpoint, jaeger_query);
    }

    // Choose exporter: gRPC (default) or HTTP (if OTLP_USE_HTTP=1)
    let use_http = std::env::var("OTLP_USE_HTTP").is_ok();
    if debug { eprintln!("OTLP_USE_HTTP={}", use_http); }

    use opentelemetry_sdk::trace as sdktrace;
    use opentelemetry_sdk::Resource;

    let tracer_provider = if use_http {
        if debug {
            eprintln!(
                "Using OTLP HTTP exporter -> {}/v1/traces",
                otlp_endpoint.trim_end_matches('/'),
            );
        }
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .http()
                    .with_endpoint(format!("{}/v1/traces", otlp_endpoint.trim_end_matches('/'))),
            )
            .with_trace_config(
                sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "telemetry-e2e",
                )])),
            )
            .install_simple()?
    } else {
        if debug {
            eprintln!("Using OTLP gRPC exporter -> {}", otlp_endpoint);
        }
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(otlp_endpoint.clone())
                    .with_tls(false),
            )
            .with_trace_config(
                sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "telemetry-e2e",
                )])),
            )
            .install_simple()?
    };
    let tracer = global::tracer("telemetry-e2e");

    // Diagnostic: check OTLP endpoint TCP reachability before sending spans
    eprintln!("OTLP_ENDPOINT={}", otlp_endpoint);
    let mut otlp_reachable = false;
    let connect_attempts: u32 = std::env::var("OTLP_CONNECT_ATTEMPTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);
    if let Some(stripped) = otlp_endpoint.strip_prefix("http://") {
        if let Some((host, port)) = stripped.split_once(":") {
            for try_i in 0..connect_attempts {
                match std::net::TcpStream::connect_timeout(
                    &format!("{}:{}", host, port).parse().unwrap(),
                    std::time::Duration::from_secs(1),
                ) {
                    Ok(_) => {
                        eprintln!("OTLP endpoint {}:{} reachable (try {})", host, port, try_i);
                        otlp_reachable = true;
                        break;
                    }
                    Err(e) => {
                        eprintln!("OTLP connection attempt {} failed: {}", try_i, e);
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    }
                }
            }
        }
    }
    if !otlp_reachable {
        eprintln!("Warning: OTLP endpoint does not appear reachable after {} attempts; trying HTTP port 4318 as a fallback check...", connect_attempts);
        // Try HTTP port 4318 as a fallback readiness check for the collector
        if let Some(stripped) = otlp_endpoint.strip_prefix("http://") {
            if let Some((host, _)) = stripped.split_once(":") {
                let http_url = format!("http://{}:4318/", host);
                eprintln!("Checking HTTP collector endpoint {}", http_url);
                let client = Client::new();
                for j in 0..10 {
                    match client.get(&http_url).send().await {
                        Ok(resp) => {
                            eprintln!("HTTP check status: {}", resp.status());
                            otlp_reachable = true;
                            break;
                        }
                        Err(e) => {
                            eprintln!("HTTP probe {} failed: {}", j, e);
                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        }
                    }
                }
            }
        }
        if !otlp_reachable {
            eprintln!("OTLP not reachable via gRPC or HTTP fallback; continuing but expect failures");
        } else {
            eprintln!("Collector HTTP endpoint reachable via fallback; proceeding")
        }
    }

    // Create a span with our test tag and attempt a couple times to ensure exporter receives it
    if debug { eprintln!("Sending spans (3 attempts)"); }
    for i in 0..3 {
        if debug { eprintln!("creating span attempt {}", i); }
        let mut span = tracer.start("test-span");
        span.set_attribute(KeyValue::new("test_id", test_id.clone()));
        drop(span);
        if debug { eprintln!("span created attempt {}", i); }
        // small ramp between attempts
        tokio::time::sleep(std::time::Duration::from_millis(200 * (i + 1))).await;
    }

    // Ensure exporter flushes
    if debug { eprintln!("Calling shutdown_tracer_provider..."); }
    global::shutdown_tracer_provider();
    if debug { eprintln!("shutdown_tracer_provider done"); }

    // Poll Jaeger query API for the trace containing our test_id with exponential backoff + jitter
    let client = Client::new();
    // Tunable defaults: shorter for local runs to avoid long hangs
    let attempts: u32 = std::env::var("OTLP_POLL_ATTEMPTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let initial_backoff_ms: u64 = std::env::var("OTLP_BACKOFF_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(500);
    let max_backoff_ms: u64 = std::env::var("OTLP_MAX_BACKOFF_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);

    let start = std::time::Instant::now();
    // Overall timeout to avoid infinite runs (configurable via OTLP_TOTAL_TIMEOUT_MS)
    let total_timeout_ms: u64 = std::env::var("OTLP_TOTAL_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30_000);

    eprintln!("Starting poll: attempts={}, initial_backoff_ms={}, max_backoff_ms={}, total_timeout_ms={}", attempts, initial_backoff_ms, max_backoff_ms, total_timeout_ms);

    let poll_future = async {
        let mut found_local = false;
        let mut last_body_local: Option<String> = None;
        let mut attempts_used_local: Option<u32> = None;
        let mut unreachable_streak: u32 = 0u32;

        for attempt in 0..attempts {
            eprintln!("poll attempt {}", attempt);
            // Small readiness check for Jaeger
            if let Ok(resp) = client
                .get(format!("{}/api/services", jaeger_query))
                .send()
                .await
            {
                if !resp.status().is_success() {
                    eprintln!("Jaeger /api/services returned status {}", resp.status());
                    unreachable_streak += 1;
                } else {
                    unreachable_streak = 0;
                }
            } else {
                eprintln!("Failed to reach Jaeger /api/services on attempt {}", attempt);
                unreachable_streak += 1;
            }

            // if Jaeger is unreachable for several attempts, abort early and record metrics
            if unreachable_streak >= 10 {
                eprintln!("Jaeger appears unreachable after {} attempts, aborting test early", unreachable_streak);
                return (false, Some(attempt + 1), Some("Jaeger unreachable".to_string()), start.elapsed());
            }

            // Try service-specific query first, then fallback to all traces
            let urls = vec![
                format!("{}/api/traces?service={}&limit=20", jaeger_query, "telemetry-e2e"),
                format!("{}/api/traces?limit=50", jaeger_query),
            ];

            for url in urls {
                if debug { eprintln!("poll attempt {}: querying {}", attempt, url); }
                if let Ok(resp) = client.get(&url).send().await {
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    if debug { eprintln!("response status {} length {}", status, text.len()); }
                    if status.is_success() {
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            if let Some(arr) = json.get("data").and_then(|d| d.as_array()) {
                                'outer: for trace in arr {
                                    if let Some(spans) = trace.get("spans").and_then(|s| s.as_array()) {
                                        for span in spans {
                                            if let Some(tags) = span.get("tags").and_then(|t| t.as_array()) {
                                                for tag in tags {
                                                    if tag.get("key").and_then(|k| k.as_str())
                                                        == Some("test_id")
                                                    {
                                                        if tag.get("value").and_then(|v| v.as_str())
                                                            == Some(&test_id)
                                                        {
                                                            if debug { eprintln!("found trace in response on attempt {}", attempt); }
                                                            found_local = true;
                                                            attempts_used_local = Some(attempt + 1);
                                                            break 'outer;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if debug { eprintln!("no data array present in response") }
                            }
                        } else {
                            last_body_local = Some(text);
                        }
                    } else {
                        last_body_local = Some(text);
                    }
                } else {
                    eprintln!("Failed to query Jaeger trace API on attempt {} (url={})", attempt, url);
                }

                if found_local {
                    break;
                }
            }

            if found_local {
                break;
            }

            // exponential backoff with simple jitter
            let exp = 1u64 << std::cmp::min(attempt, 10);
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

        (found_local, attempts_used_local, last_body_local, start.elapsed())
    };

    let (found, attempts_used, last_body, duration) = match tokio::time::timeout(
        std::time::Duration::from_millis(total_timeout_ms),
        poll_future,
    )
    .await
    {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Telemetry poll timed out after {} ms", total_timeout_ms);
            (false, Some(attempts), None, std::time::Duration::from_millis(total_timeout_ms))
        }
    };

    let duration_ms = duration.as_millis();
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
