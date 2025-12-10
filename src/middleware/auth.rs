use axum::body::Body;
use axum::http::{Request, Response, HeaderMap};
use axum::response::IntoResponse;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};
use chrono::Utc;
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tower_http::auth::AsyncAuthorizeRequest;
use futures_core::future::BoxFuture;

use crate::error::{UniAxNftErr, UniAxNftResult};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub user_id: u32,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone)]
struct UserInfo {
    pub user_id: u32,
    pub email: String,
}

struct JwtKeypair {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtKeypair {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
static JWT_KEYPAIR: LazyLock<JwtKeypair> = LazyLock::new(|| {
    let secret = std::env::var("SERVER_JWT_SECRET").expect("SERVER_JWT_SECRET must be set");
    JwtKeypair::new(secret.as_bytes())
});

#[derive(Clone, Copy)]
pub struct Authorizer;

impl Authorizer {
    pub fn init_jwt_keypair() {
        let _placeholder = &JWT_KEYPAIR;
    }

    pub fn authorize(user_info: UserInfo) -> UniAxNftResult<String> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            user_id: user_info.user_id,
            email: user_info.email,
            iat: now,
            exp: now + 3600,
        };

        let token = encode(&Header::default(), &claims, &JWT_KEYPAIR.encoding)
            .map_err(|e| UniAxNftErr::AuthErr(e.to_string()))?;
        Ok(token.to_string())
    }

    fn verify_jwt(headers: &HeaderMap) -> UniAxNftResult<UserInfo> {
        let bearer = headers
            .typed_get::<Authorization<Bearer>>()
            .ok_or("missing Bearer in header")
            .map_err(|e| UniAxNftErr::InvalidToken(e.to_string()))?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &JWT_KEYPAIR.decoding,
            &Validation::default()
        ).map_err(|e| UniAxNftErr::InvalidToken(e.to_string()))?;

        Ok(UserInfo {
            user_id: token_data.claims.user_id,
            email: token_data.claims.email
        })
    }
}


impl AsyncAuthorizeRequest<Body> for Authorizer {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<Body>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        Box::pin(async {
            match Authorizer::verify_jwt(request.headers()) {
                Ok(user_info) => {
                    request.extensions_mut().insert(user_info);
                    Ok(request)
                },
                Err(e) => Err(e.into_response()),
            }
        })
    }
}


