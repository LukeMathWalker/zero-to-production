use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn login_form() -> HttpResponse {
    let error_html: &str = todo!();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {}
    <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
            />
        </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
            />
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#,
            error_html
        ))
}
