use crate::helpers::spawn_app;

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_password() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_admin_dashboard().await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");
}
