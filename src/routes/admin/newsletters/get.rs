use std::fmt::Write;

use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn send_issue_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let html_page = format!(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Send new issue</title>
    </head>
    <body>
        <p>Send new issue</p>
        {msg_html}
        <form action="/admin/newsletters" method="post">
            <label>Title
                <input
                    type="text"
                    placeholder="Enter title"
                    name="subject"
                >
            </label>

            <label>Text Content
                <textarea
                    placeholder="Enter the content in plaint text"
                    name="text_content"
                    rows="20"
                    cols="50"
                ></textarea>
            </label>

            <label>HTML Content
                <textarea
                    placeholder="Enter the content in HTML format"
                    name="html_content"
                    rows="20"
                    cols="50"
                ></textarea>
            </label>

            <button type="submit">Publish</button>
        </form>

        <p><a href="/admin/dashboard">&lt;- Back</a></p>
    </body>
</html>
"#
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_page)
}
