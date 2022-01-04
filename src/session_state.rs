use actix_session::{Session, SessionExt};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::Ready;
use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), serde_json::Error> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, serde_json::Error> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        std::future::ready(Ok(TypedSession(req.get_session())))
    }
}
