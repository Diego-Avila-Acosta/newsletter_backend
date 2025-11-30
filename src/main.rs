use std::net::TcpListener;

use newsletter_backend::startup::run;
use newsletter_backend::telemetry::init_subscriber;
use newsletter_backend::{configuration::get_configuration, telemetry::get_subscriber};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter_backend".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Error trying to bind address");
    run(listener, connection)?.await
}
