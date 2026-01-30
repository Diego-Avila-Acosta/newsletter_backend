use newsletter_backend::configuration::get_configuration;
use newsletter_backend::metrics::init_prometheus_exporter;
use newsletter_backend::startup::Application;
use newsletter_backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("newsletter_backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    init_prometheus_exporter();

    let configuration = get_configuration().expect("Error reading initial configuration");

    let server = Application::build(configuration).await?;
    server.run_until_stopped().await
}
