//! src/domain/new_suscriber.rs

use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
