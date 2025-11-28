use std::net::TcpListener;

use newsletter_backend::run;
use reqwest;

struct App {
    port: u16,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await.expect("Failed to spawn our app.");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{}/health_check", app.port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<App, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Error Trying to bind address");

    let port = listener.local_addr().unwrap().port();

    let server = newsletter_backend::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    Ok(App { port })
}
