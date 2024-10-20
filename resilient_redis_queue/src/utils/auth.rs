use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage};
use futures::future::{ok, Ready};
use std::future::{ready, Future};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct ApiKeyMiddleware {
    api_key: Rc<String>,
}

impl ApiKeyMiddleware {
    pub fn new(api_key: String) -> Self {
        ApiKeyMiddleware {
            api_key: Rc::new(api_key),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyMiddlewareService {
            service,
            api_key: self.api_key.clone(),
        })
    }
}

pub struct ApiKeyMiddlewareService<S> {
    service: S,
    api_key: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let api_key = self.api_key.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            if let Some(key) = res.request().headers().get("X-API-Key") {
                if key.to_str().map(|s| s == api_key.as_str()).unwrap_or(false) {
                    Ok(res)
                } else {
                    Err(ErrorUnauthorized("Invalid API Key"))
                }
            } else {
                Err(ErrorUnauthorized("API Key not provided"))
            }
        })
    }
}
