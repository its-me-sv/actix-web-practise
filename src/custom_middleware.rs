use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, Error, Result,
};
use actix_web_lab::middleware::Next;
use std::{
    future::{self, ready, Ready},
    pin::Pin,
};

pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(token) = req.headers().get("Authorization") {
            let token = token
                .to_str()
                .map_err(|_| error::ErrorUnauthorized("Invalid token"))
                .unwrap()
                .split("Bearer")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap_or(&"")
                .trim();
            if token != "123456" {
                return Box::pin(async { Err(error::ErrorUnauthorized("Invalid token")) });
            }
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async { Err(error::ErrorUnauthorized("Token not found")) })
        }
    }
}

pub async fn my_authenticator(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>> {
    if let Some(token) = req.headers().get("Authorization") {
        let token = token
            .to_str()
            .map_err(|_| error::ErrorUnauthorized("Invalid token"))
            .unwrap()
            .split("Bearer")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap_or(&"")
            .trim();
        if token != "123456" {
            return Err(error::ErrorUnauthorized("Invalid token"));
        }
        next.call(req).await
    } else {
        Err(error::ErrorUnauthorized("Token not found"))
    }
}
