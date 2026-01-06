use anyhow::Result;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_exporter_prometheus::PrometheusHandle;

/// Initialize tracing and metrics. Returns a PrometheusHandle you can use to render metrics
pub fn init_observability() -> Result<PrometheusHandle> {
    // Tracing: try to initialize a default subscriber (may already be set by caller)
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    // Metrics: install a Prometheus recorder and return the handle
    // Use install_recorder if available which returns a PrometheusHandle directly
    let handle = PrometheusBuilder::new()
        .install_recorder()?;

    Ok(handle)
}

/// Render metrics text for Prometheus to scrape
pub fn render_metrics(handle: &PrometheusHandle) -> String {
    handle.render()
}
