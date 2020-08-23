use crate::helpers::spawn_app;
use serde_json::json;

#[actix_rt::test]
async fn subscribe_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let payload = json!({
        "email": "myemail@mydomain.com",
        "name": "my name"
    });

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/subscriptions", &app.address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}
