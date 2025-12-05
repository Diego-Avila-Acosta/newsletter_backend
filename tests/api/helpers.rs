use newsletter_backend::configuration::{DatabaseSettings, get_configuration};
use newsletter_backend::startup::{Application, get_connection_pool};
use newsletter_backend::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::{MockServer, Request};

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    http_client: reqwest::Client,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_subscription(&self, body: String) -> reqwest::Response {
        self.http_client
            .post(format!("{}/subscriptions", self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub fn get_confirmation_links(&self, email_request: &Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();

            assert_eq!(links.len(), 1);

            let mut confirmation_link = reqwest::Url::parse(links[0].as_str()).unwrap();
            confirmation_link.set_port(Some(self.port)).unwrap();

            confirmation_link
        };

        let html_link = get_link(&body["HtmlBody"].as_str().unwrap());
        let plain_text_link = get_link(&body["TextBody"].as_str().unwrap());

        ConfirmationLinks {
            html: html_link,
            plain_text: plain_text_link,
        }
    }

    pub async fn post_newsletters(&self, body: serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/newsletters", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
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

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let mut configuration = {
        let mut c = get_configuration().expect("Failed to load configuration");

        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;

        c.email_client.base_url = email_server.uri();

        c
    };

    configure_database(&mut configuration.database).await;

    let server = Application::build(configuration.clone()).expect("Failed to build the app");
    let port = server.port();
    let address = format!("http://127.0.0.1:{}", port);

    let _ = tokio::spawn(server.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(configuration.database.connection_string()),
        http_client: reqwest::Client::new(),
        email_server,
        port,
    }
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
