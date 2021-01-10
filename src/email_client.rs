use crate::domain::SubscriberEmail;

pub struct EmailClient;

impl EmailClient {
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
