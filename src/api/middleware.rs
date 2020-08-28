use std::pin::Pin;
use std::task::{Context, Poll};

use crate::api::tokens::Claims;
use actix_service::{Service, Transform};
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub struct BearerAuthentication;

impl<S, B> Transform<S> for BearerAuthentication
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = BearerAuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BearerAuthenticationMiddleware { service })
    }
}

pub struct BearerAuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for BearerAuthenticationMiddleware<S>
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut authenticated: bool;
        let headers = req.headers();

        if skip_authentication(req.path()) {
            authenticated = true;
        } else {
            authenticated = false;

            if let Some(auth_header) = headers.get("AUTHORIZATION") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("bearer")
                        || auth_str.starts_with("Bearer")
                    {
                        let token = auth_str[6..auth_str.len()].trim();

                        let secret_key = std::env::var("SECRET_KEY")
                            .expect("Unable to find a SECRET_KEY");

                        let token_message = decode::<Claims>(
                            &token,
                            &DecodingKey::from_secret(secret_key.as_ref()),
                            &Validation::new(Algorithm::HS256),
                        );

                        debug!("{:#?}", token_message);
                        authenticated = true;
                    }
                }
            }
        }

        if authenticated {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized().json("").into_body(),
                ))
            })
        }
    }
}

fn skip_authentication(path: &str) -> bool {
    for ignore_route in IGNORE_ROUTES.iter() {
        if path.starts_with(ignore_route) {
            return true;
        }
    }

    false
}

pub const IGNORE_ROUTES: [&str; 1] = ["/api/v1/authenticate"];
