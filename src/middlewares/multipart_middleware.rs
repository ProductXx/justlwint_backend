use actix_service::{Service, Transform};
use actix_web::{
    Error, Result,
    dev::{ServiceRequest, ServiceResponse},
};
use futures_util::future::{Future, Ready, ok};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

// use super::multipart_handlers::{
//     car_form_multipart::car_form_multipath, driver_form_multipart::driver_form_multipath,
//     id_verify_form_multipart::id_verify_form_multipath,
//     package_post_multipart::package_post_multipath,
// };

pub struct MultipartMiddleware;

impl<S, B> Transform<S, ServiceRequest> for MultipartMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MultipartMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MultipartMiddlewareService {
            service: Arc::new(service),
        })
    }
}

pub struct MultipartMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for MultipartMiddlewareService<S>
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

    #[allow(clippy::await_holding_refcell_ref)]
    // fn call(&self, mut req: ServiceRequest) -> Self::Future {
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            if !req.path().starts_with("/forms") && !req.path().starts_with("/post") {
                return service.call(req).await;
            }

            // if req.path().ends_with("/driver_form") {
            //     driver_form_multipath(&mut req).await?;
            // } else if req.path().ends_with("/id_verify_form") {
            //     id_verify_form_multipath(&mut req).await?;
            // } else if req.path().ends_with("/car_form") {
            //     car_form_multipath(&mut req).await?;
            // } else if req.path().ends_with("/package_post") {
            //     package_post_multipath(&mut req).await?;
            // }

            service.call(req).await
        })
    }
}
