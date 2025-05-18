use actix_service::{Service, Transform};
use actix_web::{
    Error, HttpMessage,
    dev::{ServiceRequest, ServiceResponse},
};
use futures_util::future::{Future, Ready, ok};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use radix_trie::Trie;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::structures::{auth_structures::login_structures::Claims, static_vars::JWT_SECRET};

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Arc::new(service),
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let mut trie = Trie::new();
            trie.insert("/auth", true);
            trie.insert("/test", true);
            trie.insert("/swagger-ui", true);
            trie.insert("/api-docs", true);
            trie.insert("/fetch", true);

            // Step 2: Check if the request path has a matching stored prefix
            if trie.get_ancestor(req.path()).is_some() {
                return service.call(req).await;
            }

            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = auth_str.trim_start_matches("Bearer ").to_string();

                        match decode::<Claims>(
                            &token,
                            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
                            &Validation::new(Algorithm::HS256),
                        ) {
                            Ok(decoded) => {
                                req.extensions_mut().insert(decoded.claims);
                            }
                            Err(_) => {
                                return Err(actix_web::error::ErrorUnauthorized("Invalid JWT"));
                            }
                        }

                        req.extensions_mut().insert(token);
                        return service.call(req).await;
                    }
                }
            }

            Err(actix_web::error::ErrorUnauthorized("No JWT"))
        })
    }
}
