//! tests/api/login.rs
use crate::helpers::{assert_is_redirect_to, spawn_app};
use reqwest::header::HeaderValue;
use std::collections::HashSet;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;
    // Act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_is_redirect_to(&response, "/login");

    // let cookies: HashSet<_> = response
    //     .headers()
    //     .get_all("Set-Cookie")
    //     .into_iter()
    //     .collect();
    // assert!(cookies.contains(&HeaderValue::from_str("_flash=Authentication failed").unwrap()));

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    let html_page = app.get_login_html().await;
    assert!(!html_page.contains("<p><i>Authentication failed</i></p>"));
}

// #[tokio::test]
// async fn redirect_to_admin_dashboard_after_login_success() {
//     // Arrange
//     let app = spawn_app().await;
//     // Act - Part 1 - Login
//     let login_body = serde_json::json!({
//     "username": &app.test_user.username,
//     "password": &app.test_user.password
//     });
//     let response = app.post_login(&login_body).await;
//     assert_is_redirect_to(&response, "/admin/dashboard");
//     // Act - Part 2 - Follow the redirect
//     let html_page = app.get_admin_dashboard().await;
//     assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
// }