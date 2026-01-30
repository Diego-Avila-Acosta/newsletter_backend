use actix_web_metrics::{ActixWebMetrics, ActixWebMetricsBuilder};
use metrics_exporter_prometheus::PrometheusBuilder;

pub fn get_metrics_middleware(namespace: String) -> ActixWebMetrics {
    ActixWebMetricsBuilder::new().namespace(namespace).build()
}

pub fn init_prometheus_exporter() {
    PrometheusBuilder::new()
        .install()
        .expect("Failed to install Prometheus exporter")
}
