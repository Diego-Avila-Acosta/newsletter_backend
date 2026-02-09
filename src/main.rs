use newsletter_backend::configuration::get_configuration;
use newsletter_backend::metrics::init_prometheus_exporter;
use newsletter_backend::startup::Application;
use newsletter_backend::telemetry::{get_opentelemetry_parts, get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Error reading initial configuration");

    let (tracer, provider) = get_opentelemetry_parts(
        &configuration.tracer.export_endpoint,
        configuration.tracer.sampling_ratio,
    );

    let subscriber = get_subscriber(tracer, "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    init_prometheus_exporter();

    let configuration = get_configuration().expect("Error reading initial configuration");

    let server = Application::build(configuration).await?;
    server.run_until_stopped().await?;

    provider
        .shutdown()
        .expect("Failed to close tracer provider");

    Ok(())
}
