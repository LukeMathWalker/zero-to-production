use crate::helpers::spawn_app;
use uuid::Uuid;

#[actix_rt::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_change_password().await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

#[actix_rt::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}

#[actix_rt::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/password"
    );

    // Act - Part 3 - Follow the redirect
    let response = app.get_change_password().await;
    let html_page = response.text().await.unwrap();
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - \
        the field values must match.</i></p>"
    ));
}

#[actix_rt::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/password"
    );

    // Act - Part 3 - Follow the redirect
    let response = app.get_change_password().await;
    let html_page = response.text().await.unwrap();
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"));
}

#[actix_rt::test]
async fn changing_password_works() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act - Part 1 - Login
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/dashboard"
    );

    // Act - Part 2 - Change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/dashboard"
    );

    // Act - Part 3 - Follow the redirect
    let response = app.get_change_password().await;
    let html_page = response.text().await.unwrap();
    assert!(!html_page.contains(r#"<p><i>Your password has been changed!</i></p>"#));

    // Act - Part 4 - Login using the new password
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &new_password
    });
    let response = app.post_login(&login_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/admin/dashboard"
    );
}
