use crate::domain::subscriber_name::SubscriberName;
use crate::domain::subscriber_email::SubscriberEmail;

pub struct NewSubscriber {
    // We are not using `String` anymore!
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
