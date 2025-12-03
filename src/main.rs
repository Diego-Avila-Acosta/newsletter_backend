use newsletter_backend::startup::Application;
use newsletter_backend::telemetry::init_subscriber;
use newsletter_backend::{configuration::get_configuration, telemetry::get_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter_backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Error reading initial configuration");

    let server = Application::build(configuration)?;
    server.run_until_stopped().await
}
