use std::ops::Deref;

use actix_web::web;
use actix_web::{HttpResponse, http::header::ContentType};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::authentication::UserId;
use crate::utils::e500;

#[tracing::instrument(
    name = "Get admin dashboard html page"
    skip(pool, user_id)
    fields(user_id = %user_id.deref())
)]
pub async fn admin_dashboard(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let username = get_username(*user_id, &pool).await.map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html>
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Admin dashboard</title>
    </head>
    <body>
        <p>Welcome {username}</p>
        <p><a href="/admin/password">Change password</a></p>

        <form name="logoutForm" action="/admin/logout" method="post">
            <input type="submit" value="Logout">
        </form>
    </body>
</html>"#
        )))
}

#[tracing::instrument(name = "Get username by user_id from DB", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username")?;

    Ok(row.username)
}
