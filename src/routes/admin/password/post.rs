use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::authentication::{self, AuthError, Credentials, validate_credentials};
use crate::domain::AdminPassword;
use crate::routes::admin::dashboard::get_username;
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(e500)?;

    if user_id.is_none() {
        return Ok(see_other("/login"));
    }

    let user_id = user_id.unwrap();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();

        return Ok(see_other("/admin/password"));
    }

    let username = get_username(user_id, &pool).await.map_err(e500)?;

    let user_credentials = Credentials {
        username,
        password: form.0.current_password,
    };

    if let Err(e) = validate_credentials(user_credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(e) => Err(e500(e).into()),
        };
    }

    let new_password = AdminPassword::new(form.0.new_password.expose_secret().clone());

    if let Err(_) = new_password {
        FlashMessage::error(
            "The new password must be longer than 12 characters and shorter than 129 characters.",
        )
        .send();

        return Ok(see_other("/admin/password"));
    }

    let new_password = new_password.unwrap();

    authentication::change_password(user_id, new_password, &pool)
        .await
        .map_err(e500)?;

    FlashMessage::success("Your password has been changed.").send();

    Ok(see_other("/admin/password"))
}
