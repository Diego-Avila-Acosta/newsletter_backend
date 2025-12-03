use newsletter_backend::configuration::{DatabaseSettings, get_configuration};
use newsletter_backend::startup::{Application, get_connection_pool};
use newsletter_backend::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
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

pub async fn spawn_app() -> Result<TestApp, std::io::Error> {
    Lazy::force(&TRACING);

    let mut configuration = {
        let mut c = get_configuration().expect("Failed to load configuration");

        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;

        c
    };

    configure_database(&mut configuration.database).await;

    let server = Application::build(configuration.clone()).expect("Failed to build the app");
    let address = format!("http://127.0.0.1:{}", server.port());

    let _ = tokio::spawn(server.run_until_stopped());

    Ok(TestApp {
        address,
        db_pool: get_connection_pool(configuration.database.connection_string()),
    })
}

async fn configure_database(configuration: &mut DatabaseSettings) {
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
}
