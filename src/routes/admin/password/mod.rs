//! src/routes/admin/password/mod.rs
mod get;
pub use get::change_password_form;
mod post;
pub use post::change_password;