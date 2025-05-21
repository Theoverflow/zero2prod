//! src/domain/new_suscriber.rs

use crate::domain::suscriber_name::SubscriberName;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
