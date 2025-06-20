//! src/routes/login/post.rs
use crate::authentication::{AuthError, Credentials, validate_credentials};
use crate::routes::error_chain_fmt;
use crate::startup::HmacSecret;
use crate::session_state::TypedSession;
use actix_web::HttpResponse;
use actix_web::cookie::Cookie;
use actix_web::error::InternalError;
use actix_web::http::{StatusCode, header::LOCATION};
use actix_web::{ResponseError, web};
use actix_web_flash_messages::FlashMessage;
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: SecretString,
}

#[tracing::instrument(
skip(form, pool, secret, session),
fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
// We are now injecting `PgPool` to retrieve stored credentials from the database
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    secret: web::Data<HmacSecret>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };

            // let query_string = format!("error={}", urlencoding::Encoded::new(e.to_string()));

            // let hmac_tag = {
            //     let mut mac =
            //         Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes())
            //             .unwrap();
            //     mac.update(query_string.as_bytes());
            //     mac.finalize().into_bytes()
            // };

            // let response = HttpResponse::SeeOther()
            //     .insert_header((
            //         LOCATION,
            //         format!("/login?{}&tag={:x}", query_string, hmac_tag),
            //     ))
            //     .finish();
            // Err(InternalError::from_response(e, response))
            FlashMessage::error(e.to_string()).send();
            let response = HttpResponse::SeeOther()
                // No cookies here now!
                .insert_header((LOCATION, "/login"))
                .finish();
            Err(InternalError::from_response(e, response))
        }
    }
}

// Redirect to the login page with an error message.
fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// impl ResponseError for LoginError {
//     fn error_response(&self) -> HttpResponse {
//         let query_string = urlencoding::Encoded::new(self.to_string());
//         let secret: &[u8] = todo!();
//         let hmac_tag = {
//             let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).unwrap();
//             mac.update(query_string.as_bytes());
//             mac.finalize().into_bytes()
//         };

//         HttpResponse::build(self.status_code())
//             .insert_header((LOCATION, format!("/login?{query_string}&tag={hmac_tag:x}")))
//             .finish()
//     }
//     fn status_code(&self) -> StatusCode {
//         StatusCode::SEE_OTHER
//     }
// }
