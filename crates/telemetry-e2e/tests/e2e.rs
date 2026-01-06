use anyhow::Result;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace as sdktrace, Resource};
use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

#[tokio::test]
async fn otlp_trace_reaches_jaeger() -> Result<()> {
    let otlp_endpoint = std::env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:4317".into());
    let jaeger_query = std::env::var("JAEGER_QUERY_URL").unwrap_or_else(|_| "http://127.0.0.1:16686".into());

    // Unique id to search for in Jaeger
    let test_id = Uuid::new_v4().to_string();

    // Build OTLP exporter -> Collector
    let exporter = opentelemetry_otlp::new_exporter().with_endpoint(otlp_endpoint.clone());

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing(
            sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new("service.name", "telemetry-e2e")]))
        )
        .with_exporter(exporter)
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let tracer = global::tracer("telemetry-e2e");

    // Create a span with our test tag
    let mut span = tracer.start("test-span");
    span.set_attribute(KeyValue::new("test_id", test_id.clone()));
    drop(span);

    // Ensure exporter flushes
    global::shutdown_tracer_provider();

    // Poll Jaeger query API for the trace containing our test_id
    let client = Client::new();
    let mut found = false;
    for _ in 0..30 {
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
                                                    break 'outer;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if found { break; }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    assert!(found, "Trace with test_id {} not found in Jaeger", test_id);
    Ok(())
}
