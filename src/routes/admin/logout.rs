use std::ops::Deref;

use actix_web::HttpResponse;
use actix_web::web::ReqData;
use actix_web_flash_messages::FlashMessage;

use crate::authentication::UserId;
use crate::session_state::TypedSession;
use crate::utils::see_other;

#[tracing::instrument(
    name = "Admin user logs out",
    skip(session, user_id),
    fields(user_id = %user_id.deref()))
]
pub async fn log_out(
    session: TypedSession,
    user_id: ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    session.log_out();
    FlashMessage::info("You have successfully logged out.").send();
    Ok(see_other("/login"))
}
