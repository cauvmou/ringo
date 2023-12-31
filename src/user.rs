use crate::queries;
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    code: String,
    name: String,
}

impl User {
    pub fn new(code: String, name: String) -> Self {
        Self { code, name }
    }
}

impl Into<String> for User {
    fn into(self) -> String {
        let claims = RegisteredClaims {
            issuer: Some("ringo".into()),
            subject: Some(
                rocket::serde::json::serde_json::to_string(&self)
                    .expect("JSON SERIALIZATION FAILED!"),
            ),
            ..Default::default()
        };
        let key = queries::get_signing_key();
        claims
            .sign_with_key(&key)
            .expect("Failed to sign with key!")
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(header) = request.headers().get_one("Authorization") {
            let string_token = header.replace("Bearer ", "");
            let key = queries::get_signing_key();
            let claims: RegisteredClaims = string_token
                .verify_with_key(&key)
                .expect("Failed to verify token!");
            let claims = claims.subject.expect("No claims in token!");
            let user: User = rocket::serde::json::serde_json::from_str(claims.as_str())
                .expect("Failed to deserialize user!");
            Outcome::Success(user)
        } else {
            Outcome::Error((Status::Unauthorized, "Not authenticated.".to_owned()))
        }
    }
}
