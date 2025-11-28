use std::net::TcpListener;

use newsletter_backend::run;
use reqwest;

struct App {
    port: u16,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().expect("Failed to spawn our app.");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{}/health_check", app.port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    let body = "name=diego&email=diego20@gmail.com";
    let response = client
        .post(format!("http://127.0.0.1:{}/subscriptions", app.port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=diego", "missing the email"),
        ("email=diego20@gmail.com", "missing the name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("http://127.0.0.1:{}/subscriptions", app.port))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

fn spawn_app() -> Result<App, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Error Trying to bind address");

    let port = listener.local_addr().unwrap().port();

    let server = newsletter_backend::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    Ok(App { port })
}
