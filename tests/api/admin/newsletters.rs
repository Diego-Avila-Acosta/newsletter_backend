use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn must_be_logged_in_to_get_send_issue_form() {
    let app = spawn_app().await;

    let response = app.get_send_issue().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logged_user_can_get_send_issuer_form() {
    let app = spawn_app().await;

    let html_page = app.get_change_password_html().await;

    assert!(html_page.contains("<p>Send new issue</p>"))
}
