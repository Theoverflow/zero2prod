use crate::startup::HmacSecret;
use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_flash_messages::IncomingFlashMessages;
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use std::fmt::Write;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();

        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

pub async fn login_form(
    query: Option<web::Query<QueryParams>>,
    secret: web::Data<HmacSecret>,
) -> HttpResponse {
    let error_html = match query {
        None => "".into(),
        Some(query) => match query.into_inner().verify(&secret) {
            Ok(err) => {
                format!("<p><i>{}</i></p>", htmlescape::encode_minimal(&err))
            }
            Err(e) => {
                tracing::warn!(
                error.message = %e,
                error.cause_chain = ?e,
                "Failed to verify query parameters using the HMAC tag"
                );
                "".into()
            }
        },
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Login</title>
</head>
<body>
{error_html}
<form action="/login" method="post">
<label>Username
<input
type="text"
placeholder="Enter Username"
name="username"
>
</label>
<label>Password
<input
type="password"
placeholder="Enter Password"
name="password"
>
</label>
<button type="submit">Login</button>
</form>
</body>
</html>"#,
        ))
}

// pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
//     let mut error_html = String::new();
//     for m in flash_messages.iter() {
//         writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
//     }
//     HttpResponse::Ok()
//         .content_type(ContentType::html())
//         .body(format!(
//             r#"<!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta http-equiv="content-type" content="text/html; charset=utf-8">
//     <title>Login</title>
// </head>
// <body>
//     {error_html}
//     <form action="/login" method="post">
//         <label>Username
//             <input
//                 type="text"
//                 placeholder="Enter Username"
//                 name="username"
//             >
//         </label>
//         <label>Password
//             <input
//                 type="password"
//                 placeholder="Enter Password"
//                 name="password"
//             >
//         </label>
//         <button type="submit">Login</button>
//     </form>
// </body>
// </html>"#,
//         ))
// }
