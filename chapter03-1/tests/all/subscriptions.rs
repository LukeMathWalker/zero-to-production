use crate::helpers::spawn_app;
use serde_json::json;

#[actix_rt::test]
async fn subscribe_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let email = "myemail@mydomain.com";
    let name = "my name";
    let payload = json!({
        "email": email,
        "name": name
    });

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, email);
    assert_eq!(saved.name, name);
}
