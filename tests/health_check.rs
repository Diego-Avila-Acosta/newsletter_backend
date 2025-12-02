use std::net::TcpListener;

use newsletter_backend::configuration::{DatabaseSettings, get_configuration};
use newsletter_backend::email_client::EmailClient;
use newsletter_backend::startup::run;
use newsletter_backend::telemetry::*;
use once_cell::sync::Lazy;
use reqwest;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

struct App {
    port: u16,
    db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_name = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_name, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_name, std::io::sink);
        init_subscriber(subscriber);
    }
});

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    let body = "name=diego&email=diego20@gmail.com";
    let response = client
        .post(format!("http://127.0.0.1:{}/subscriptions", app.port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let query = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(query.email, "diego20@gmail.com");
    assert_eq!(query.name, "diego");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=diego", "missing the email"),
        ("email=diego20@gmail.com", "missing the name"),
        ("name=diego&email=invalid-email", "invalid email"),
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

async fn spawn_app() -> Result<App, std::io::Error> {
    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to load configuration");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Error Trying to bind address");
    let port = listener.local_addr().unwrap().port();

    let db_pool = configure_database(&mut configuration.database).await;

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let timeout = configuration.email_client.timeout();

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let server = run(listener, db_pool.clone(), email_client).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    Ok(App { port, db_pool })
}

async fn configure_database(configuration: &mut DatabaseSettings) -> PgPool {
    configuration.database_name = Uuid::new_v4().to_string();

    let mut connection = PgConnection::connect(&configuration.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, configuration.database_name).as_str())
        .await
        .expect("Failed to create database");

    let pool = PgPool::connect(&configuration.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}
