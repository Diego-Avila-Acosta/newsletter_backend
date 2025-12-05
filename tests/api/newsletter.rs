use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::{ConfirmationLinks, TestApp, spawn_app};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as HTML</p>"
        }
    });

    let response = app.post_newsletters(newsletter_request_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as HTML</p>"
        }
    });

    let response = app.post_newsletters(newsletter_request_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=diego&email=diego20@gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscription(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    app.get_confirmation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirmation_link.plain_text)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
#[tokio::test]
async fn newsletters_return_400_for_invalid_data() {
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;

    let test_cases = [
        (
            serde_json::json!({
                "title": "Newsletter title",
            }),
            "missing content",
        ),
        (
            serde_json::json!({
            "content": {
                "text": "Newsletter body as plain text",
                "html": "<p>Newsletter body as HTML</p>"
            }
            }),
            "missing title",
        ),
    ];

    for (invalid_json, message) in test_cases {
        let response = app.post_newsletters(invalid_json).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when te payload was {}",
            message
        );
    }
}
