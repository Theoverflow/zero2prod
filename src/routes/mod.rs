//! src/routes/mod.rs
mod health_check;
mod home;
mod login;
mod newsletter;
mod subscription_confirms;
mod subscriptions;

pub use health_check::*;
pub use home::*;
pub use login::*;
pub use newsletter::*;
pub use subscription_confirms::*;
pub use subscriptions::*;
