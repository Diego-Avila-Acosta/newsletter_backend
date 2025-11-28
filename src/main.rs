use std::net::TcpListener;

use newsletter_backend::configuration::get_configuration;
use newsletter_backend::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Error trying to bind address");
    run(listener)?.await
}
