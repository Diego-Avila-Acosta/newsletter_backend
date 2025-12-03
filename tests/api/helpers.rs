use newsletter_backend::configuration::{DatabaseSettings, get_configuration};
use newsletter_backend::email_client::EmailClient;
use newsletter_backend::startup::run;
use newsletter_backend::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct App {
    pub port: u16,
    pub db_pool: PgPool,
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

pub async fn spawn_app() -> Result<App, std::io::Error> {
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
