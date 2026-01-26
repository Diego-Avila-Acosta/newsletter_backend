use crate::{
    domain::SubscriberEmail,
    email_client::EmailClient,
    utils::{e500, see_other},
};
use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use sqlx::postgres::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    // TODO: Validation of Form data
    title: String,
    text_content: String,
    html_content: String,
}

pub async fn send_issue(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, actix_web::Error> {
    let subscribers = get_confirmed_subscribers(&pool).await.map_err(e500)?;

    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => email_client
                .send_email(
                    &subscriber.email,
                    &form.0.title,
                    &form.0.html_content,
                    &form.0.text_content,
                )
                .await
                .with_context(|| {
                    format!("Failed to send a newsletter issue to {}", subscriber.email)
                })
                .map_err(e500)?,
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    error.message = %error,
                    "Skipping a confirmed subscriber. Their stored contact details are invalid"
                );
            }
        }
    }

    FlashMessage::info("The newsletter issue has been published!").send();
    Ok(see_other("/admin/newsletters"))
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers = sqlx::query!(
        r#"SELECT email FROM subscriptions
        WHERE status = 'confirmed';
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| match SubscriberEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();

    Ok(confirmed_subscribers)
}
