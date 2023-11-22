use axum::{
    response::{Response, IntoResponse},
    body::Body,
    http::{Request, StatusCode},
};
use futures_util::future::BoxFuture;
use tower::{Service, Layer};
use std::task::{Context, Poll};

use crate::core::{consts, state};
use crate::util::{jwt, resp};

#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
    where
        S: Service<Request<Body>, Response=Response> + Send + 'static,
        S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        tracing::info!("auth layer call");
        let mut user_id = None;
        let mut is_exp = false;

        let mut authenticate_pass: bool = false;
        // if setting::UNAUTH_ROUTERS.contains(&req.path()) {
        //     authenticate_pass = true;
        // }
        for ignore_route in consts::UNAUTH_ROUTERS.iter() {
            if request.uri().path().starts_with(ignore_route) {
                authenticate_pass = true;
                break;
            }
        }
        if !authenticate_pass {
            match request.headers().get("Authorization") {
                None => authenticate_pass = false,
                Some(auth_header) => match auth_header.to_str() {
                    Err(_) => authenticate_pass = false,
                    Ok(auth_str) => {
                        if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                            let token = auth_str[6..auth_str.len()].trim();
                            if let Ok(result) = jwt::decode_token(token.into()) {
                                authenticate_pass = true;
                                user_id = Some(result.id);
                                is_exp = result.is_exp;
                            }
                        }
                    }
                },
            }
        }

        if !authenticate_pass {
            let rsp = resp::ErrorResponse {
                code: 403,
                msg: "Forbidden".to_string(),
                data: "".to_string(),
            };
            return Box::pin(async { Ok((StatusCode::FORBIDDEN, axum::Json(rsp)).into_response()) });
        }

        request.extensions_mut()
            .insert(state::ReqContext { user_id: user_id.clone().unwrap() });


        let future = self.inner.call(request);
        Box::pin(async move {
            let mut response: Response = future.await?;
            if is_exp {
                let new_token = jwt::encode_token(user_id.unwrap()).unwrap();
                response.headers_mut().append("SET-TOKEN", new_token.parse().unwrap());
            }
            Ok(response)
        })
    }
}