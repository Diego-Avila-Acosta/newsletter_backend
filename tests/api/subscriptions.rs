use crate::helpers::spawn_app;

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
