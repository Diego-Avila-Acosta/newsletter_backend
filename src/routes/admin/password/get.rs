use std::fmt::Write;

use actix_web::{HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;

#[tracing::instrument(
    name = "Get change password form"
    skip(flash_messages) // TODO: May be better to record flash_messages to span
)]
pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="e">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Change Password</title>
</head>
<body>
    {msg_html}
    <form action="/admin/password" method="post">
        <label>Current password
            <input type="password" placeholder="Enter current password" name="current_password">
        </label>

        <br>

        <label>New password
            <input type="password" placeholder="Enter new password" name="new_password">
        </label>

        <br>

        <label>Confirm new password
            <input type="password" placeholder="Confirm new password" name="new_password_check">
        </label>

    </form>

    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>


        "#,
        )))
}
