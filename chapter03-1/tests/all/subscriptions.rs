use crate::helpers::spawn_app;
use serde_json::json;

#[actix_rt::test]
async fn subscribe_saves_subscriber_data() {
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
        .form(&payload)
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

#[actix_rt::test]
async fn subscribe_accepts_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let invalid_bodies = vec![
        // Missing email
        "name=le%20guin",
        // Missing name
        "email=ursula_le_guin%40gmail.com",
        // Missing both name and email
        ""
    ];

    for invalid_body in invalid_bodies {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when '{}' was passed as input.",
            invalid_body
        );
    }
}
