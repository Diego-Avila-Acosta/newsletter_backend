use std::net::TcpListener;

use newsletter_backend::configuration::get_configuration;
use newsletter_backend::run;
use sqlx::{Connection, PgPool};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Error trying to bind address");
    run(listener, connection)?.await
}
