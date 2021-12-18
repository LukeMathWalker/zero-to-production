use crate::session_state::TypedSession;
use crate::utils::e500;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::HttpResponse;

pub async fn change_password_form(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Change Password</title>
</head>
<body>
    <form action="/admin/password" method="post">
        <label>Old password
            <input
                type="password"
                placeholder="Enter old password"
                name="old_password"
            />
        </label>
        <br />
        <label>New password
            <input
                type="password"
                placeholder="Enter new password"
                name="new_password"
            />
        </label>
        <br />
        <label>Confirm new password
            <input
                type="password"
                placeholder="Type the new password again"
                name="new_password_check"
            />
        </label>
        <br />
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
    ))
}
