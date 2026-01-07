use anyhow::Result;
use opentelemetry::trace::{Span, Tracer};
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use reqwest::Client;
use serde_json::Value;
use std::net::ToSocketAddrs;
use uuid::Uuid;

#[tokio::test]
#[allow(
    clippy::unwrap_used,
    clippy::disallowed_methods,
    clippy::collapsible_if,
    unused_variables
)]
async fn otlp_trace_reaches_jaeger() -> Result<()> {
    // Overall test-level timeout (ms). Default 60s to avoid endless tests; configurable via TELEMETRY_TEST_MAX_MS
    let max_runtime_ms: u64 = std::env::var("TELEMETRY_TEST_MAX_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60_000);

    let inner = async {
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
            eprintln!(
                "OTLP_ENDPOINT={} JAEGER_QUERY_URL={}",
                otlp_endpoint, jaeger_query
            );
        }

        // Choose exporter: gRPC (default) or HTTP (if OTLP_USE_HTTP=1)
        let use_http = std::env::var("OTLP_USE_HTTP").is_ok();
        if debug {
            eprintln!("OTLP_USE_HTTP={}", use_http);
        }

        use opentelemetry_sdk::trace as sdktrace;
        use opentelemetry_sdk::Resource;

        let tracer_provider =
            if use_http {
                // If user provided the default gRPC endpoint, prefer the HTTP port 4318 for OTLP/HTTP
                let http_endpoint = if otlp_endpoint.ends_with(":4317") {
                    otlp_endpoint.replacen(":4317", ":4318", 1)
                } else if otlp_endpoint.ends_with(':') {
                    format!("{}4318", otlp_endpoint)
                } else {
                    otlp_endpoint.clone()
                };
                if debug {
                    eprintln!(
                        "Using OTLP HTTP exporter -> {}/v1/traces (derived from {})",
                        http_endpoint.trim_end_matches('/'),
                        otlp_endpoint
                    );
                }
                opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(
                        opentelemetry_otlp::new_exporter()
                            .http()
                            .with_endpoint(http_endpoint.clone())
                            .with_http_client(reqwest::Client::new()),
                    )
                    .with_trace_config(sdktrace::Config::default().with_resource(Resource::new(
                        vec![KeyValue::new("service.name", "telemetry-e2e")],
                    )))
                    .install_batch(opentelemetry_sdk::runtime::Tokio)?
            } else {
                if debug {
                    eprintln!("Using OTLP gRPC exporter -> {}", otlp_endpoint);
                }
                opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(
                        opentelemetry_otlp::new_exporter()
                            .tonic()
                            .with_endpoint(otlp_endpoint.clone()),
                    )
                    .with_trace_config(sdktrace::Config::default().with_resource(Resource::new(
                        vec![KeyValue::new("service.name", "telemetry-e2e")],
                    )))
                    .install_batch(opentelemetry_sdk::runtime::Tokio)?
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
                    let addr_str = format!("{}:{}", host, port);
                    match addr_str.as_str().to_socket_addrs() {
                        Ok(mut addrs) => {
                            if let Some(sock) = addrs.next() {
                                match std::net::TcpStream::connect_timeout(
                                    &sock,
                                    std::time::Duration::from_secs(1),
                                ) {
                                    Ok(_) => {
                                        eprintln!(
                                            "OTLP endpoint {}:{} reachable (try {})",
                                            host, port, try_i
                                        );
                                        otlp_reachable = true;
                                        break;
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "OTLP connection attempt {} failed: {}",
                                            try_i, e
                                        );
                                        tokio::time::sleep(std::time::Duration::from_millis(500))
                                            .await;
                                    }
                                }
                            } else {
                                eprintln!("Could not resolve {} to a socket address", addr_str);
                                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to resolve {}: {}", addr_str, e);
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
                eprintln!(
                    "OTLP not reachable via gRPC or HTTP fallback; continuing but expect failures"
                );
            } else {
                eprintln!("Collector HTTP endpoint reachable via fallback; proceeding")
            }
        }

        // Create a span with our test tag and attempt a couple times to ensure exporter receives it
        if debug {
            eprintln!("Sending spans (3 attempts)");
        }
        for i in 0..3 {
            if debug {
                eprintln!("creating span attempt {}", i);
            }
            let mut span = tracer.start("test-span");
            span.set_attribute(KeyValue::new("test_id", test_id.clone()));
            drop(span);
            if debug {
                eprintln!("span created attempt {}", i);
            }
            // small ramp between attempts
            tokio::time::sleep(std::time::Duration::from_millis(200 * (i + 1))).await;
        }

        // Ensure exporter flushes, but do shutdown in a blocking task and cap wait time so we don't block the test thread
        if debug {
            eprintln!("Calling shutdown_tracer_provider (spawn_blocking + timeout)...");
        }
        let shutdown_wait_ms: u64 = std::env::var("OTLP_SHUTDOWN_WAIT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2000);
        let shutdown_handle = tokio::task::spawn_blocking(|| {
            global::shutdown_tracer_provider();
        });
        match tokio::time::timeout(
            std::time::Duration::from_millis(shutdown_wait_ms),
            shutdown_handle,
        )
        .await
        {
            Ok(Ok(_)) => {
                if debug {
                    eprintln!("shutdown_tracer_provider completed");
                }
            }
            Ok(Err(e)) => {
                eprintln!("shutdown task panicked: {:?}", e);
            }
            Err(_) => {
                eprintln!(
                    "shutdown_tracer_provider timed out after {} ms",
                    shutdown_wait_ms
                );
            }
        }

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

        eprintln!(
            "Starting poll: attempts={}, initial_backoff_ms={}, max_backoff_ms={}, total_timeout_ms={}",
            attempts, initial_backoff_ms, max_backoff_ms, total_timeout_ms
        );

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
                    eprintln!(
                        "Failed to reach Jaeger /api/services on attempt {}",
                        attempt
                    );
                    unreachable_streak += 1;
                }

                // if Jaeger is unreachable for several attempts, abort early and record metrics
                if unreachable_streak >= 10 {
                    eprintln!(
                        "Jaeger appears unreachable after {} attempts, aborting test early",
                        unreachable_streak
                    );
                    return (
                        false,
                        Some(attempt + 1),
                        Some("Jaeger unreachable".to_string()),
                        start.elapsed(),
                    );
                }

                // Try service-specific query first, then fallback to all traces
                let urls = vec![
                    format!(
                        "{}/api/traces?service={}&limit=20",
                        jaeger_query, "telemetry-e2e"
                    ),
                    format!("{}/api/traces?limit=50", jaeger_query),
                ];

                for url in urls {
                    if debug {
                        eprintln!("poll attempt {}: querying {}", attempt, url);
                    }

                    if let Ok(resp) = client.get(&url).send().await {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        if debug {
                            eprintln!("response status {} length {}", status, text.len());
                        }
                        if status.is_success() {
                            if let Ok(json) = serde_json::from_str::<Value>(&text) {
                                if let Some(arr) = json.get("data").and_then(|d| d.as_array()) {
                                    'outer: for trace in arr {
                                        if let Some(spans) =
                                            trace.get("spans").and_then(|s| s.as_array())
                                        {
                                            for span in spans {
                                                if let Some(tags) =
                                                    span.get("tags").and_then(|t| t.as_array())
                                                {
                                                    for tag in tags {
                                                        if tag.get("key").and_then(|k| k.as_str())
                                                            == Some("test_id")
                                                        {
                                                            if tag
                                                                .get("value")
                                                                .and_then(|v| v.as_str())
                                                                == Some(&test_id)
                                                            {
                                                                if debug {
                                                                    eprintln!("found trace in Jaeger response on attempt {}", attempt);
                                                                }
                                                                found_local = true;
                                                                attempts_used_local =
                                                                    Some(attempt + 1);
                                                                break 'outer;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if debug {
                                    eprintln!("no data array present in Jaeger response")
                                }
                            } else {
                                last_body_local = Some(text);
                            }
                        } else {
                            last_body_local = Some(text);
                        }
                    } else {
                        eprintln!(
                            "Failed to query Jaeger trace API on attempt {} (url={})",
                            attempt, url
                        );
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
                let jitter =
                    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                        Ok(d) => (d.subsec_nanos() % 1000) as u64,
                        Err(_) => 0u64,
                    };
                let sleep_ms = sleep_ms + jitter;
                tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;
            }

            (
                found_local,
                attempts_used_local,
                last_body_local,
                start.elapsed(),
            )
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
                (
                    false,
                    Some(attempts),
                    None,
                    std::time::Duration::from_millis(total_timeout_ms),
                )
            }
        };

        let duration_ms = duration.as_millis();
        let timestamp_unix =
            match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Ok(d) => d.as_secs(),
                Err(_) => 0u64,
            };
        let metrics = serde_json::json!({
            "test_id": test_id,
            "found": found,
            "attempts": attempts_used.unwrap_or(attempts),
            "duration_ms": duration_ms,
            "last_body": last_body,
            "timestamp_unix": timestamp_unix,
        });
        let metrics_path = std::env::var("TELEMETRY_METRICS_FILE")
            .unwrap_or_else(|_| "telemetry_e2e_metrics.json".into());
        let body = serde_json::to_string_pretty(&metrics)?;
        match std::fs::write(&metrics_path, &body) {
            Ok(_) => eprintln!("Wrote metrics to {}", metrics_path),
            Err(e) => eprintln!("Failed to write metrics file {}: {}", metrics_path, e),
        }

        if !found {
            eprintln!(
                "Trace with test_id {} not found, collecting diagnostics...",
                test_id
            );
            let diag_path = collect_diagnostics(&test_id, &otlp_endpoint, &jaeger_query, use_http)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Failed to collect diagnostics: {}", e);
                    "<diag-failed>".to_string()
                });
            panic!(
                "Trace with test_id {} not found in Jaeger, last body: {:?}. Diagnostics: {}",
                test_id, last_body, diag_path
            );
        }

        Ok(())
    };

    match tokio::time::timeout(std::time::Duration::from_millis(max_runtime_ms), inner).await {
        Ok(Ok(r)) => Ok(r),
        Ok(Err(e)) => Err(e),
        Err(_) => {
            eprintln!(
                "Test exceeded max runtime ({} ms). Collecting diagnostics...",
                max_runtime_ms
            );
            // attempt to collect diagnostics quickly
            let diag_path = collect_diagnostics(
                "timed-out",
                &std::env::var("OTLP_ENDPOINT").unwrap_or_default(),
                &std::env::var("JAEGER_QUERY_URL").unwrap_or_default(),
                std::env::var("OTLP_USE_HTTP").is_ok(),
            )
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to collect diagnostics: {}", e);
                "<failed>".to_string()
            });
            return Err(anyhow::anyhow!(
                "Test timed out after {} ms; diagnostics written to {}",
                max_runtime_ms,
                diag_path
            ));
        }
    }
}

async fn collect_diagnostics(
    test_id: &str,
    otlp_endpoint: &str,
    jaeger_query: &str,
    use_http: bool,
) -> Result<String> {
    use serde_json::Value as JVal;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()?;
    let mut diag = serde_json::Map::new();
    diag.insert(
        "timestamp".into(),
        serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
    );
    diag.insert("test_id".into(), serde_json::Value::String(test_id.into()));
    diag.insert(
        "otlp_endpoint".into(),
        serde_json::Value::String(otlp_endpoint.into()),
    );
    diag.insert(
        "jaeger_query".into(),
        serde_json::Value::String(jaeger_query.into()),
    );
    diag.insert("use_http".into(), serde_json::Value::Bool(use_http));

    // quick TCP check to OTLP host:port (if possible)
    if let Some(stripped) = otlp_endpoint.strip_prefix("http://") {
        if let Some((host, port)) = stripped.split_once(":") {
            let addr = format!("{}:{}", host, port);
            let tcp_ok = match addr.as_str().to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(sock) = addrs.next() {
                        std::net::TcpStream::connect_timeout(
                            &sock,
                            std::time::Duration::from_millis(500),
                        )
                        .is_ok()
                    } else {
                        false
                    }
                }
                Err(_) => false,
            };
            diag.insert("otlp_tcp_connect".into(), serde_json::Value::Bool(tcp_ok));
        }
    }

    // try simple HTTP GETs and capture short body (truncated)
    async fn try_get(url: &str, client: &reqwest::Client) -> Result<(u16, String), String> {
        match client.get(url).send().await {
            Ok(r) => {
                let status = r.status().as_u16();
                let text = r.text().await.unwrap_or_default();
                let truncated: String = text.chars().take(2048).collect();
                Ok((status, truncated))
            }
            Err(e) => Err(format!("{}", e)),
        }
    }

    // OTLP HTTP probe
    if otlp_endpoint.starts_with("http") {
        match try_get(otlp_endpoint, &client).await {
            Ok((s, b)) => {
                diag.insert(
                    "otlp_http_status".into(),
                    serde_json::Value::Number(serde_json::Number::from(s)),
                );
                diag.insert("otlp_http_body".into(), serde_json::Value::String(b));
            }
            Err(e) => {
                diag.insert("otlp_http_error".into(), serde_json::Value::String(e));
            }
        }
    }

    // Jaeger /api/services
    let jaeger_services_url = format!("{}/api/services", jaeger_query.trim_end_matches('/'));
    match try_get(jaeger_services_url.as_str(), &client).await {
        Ok((s, b)) => {
            diag.insert(
                "jaeger_services_status".into(),
                serde_json::Value::Number(serde_json::Number::from(s)),
            );
            diag.insert("jaeger_services_body".into(), serde_json::Value::String(b));
        }
        Err(e) => {
            diag.insert("jaeger_services_error".into(), serde_json::Value::String(e));
        }
    };

    // Attempt to fetch capture dump if present
    let capture_dump_url = "http://127.0.0.1:4318/dump".to_string();
    match try_get(capture_dump_url.as_str(), &client).await {
        Ok((s, b)) => {
            diag.insert(
                "capture_dump_status".into(),
                serde_json::Value::Number(serde_json::Number::from(s)),
            );
            diag.insert("capture_dump_body".into(), serde_json::Value::String(b));
        }
        Err(e) => {
            diag.insert("capture_dump_error".into(), serde_json::Value::String(e));
        }
    };

    // write diagnostic file
    let path = "target/telemetry_e2e_timeout_diag.json";
    let _ = std::fs::create_dir_all("target");
    let _ = std::fs::write(path, serde_json::to_string_pretty(&JVal::Object(diag))?);
    Ok(path.into())
}
