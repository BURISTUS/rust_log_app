use crate::startup::AppState;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};

pub struct BeforeRequest;

impl<S, B> Transform<S, ServiceRequest> for BeforeRequest
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = BeforeRequestMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BeforeRequestMiddleware { service })
    }
}

pub struct BeforeRequestMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for BeforeRequestMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let app_state = req
            .app_data::<Data<AppState>>()
            .expect("Can't get app state from request");
        let app_state_clone = app_state.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            {
                let mut server_stats = app_state_clone
                    .server_stats
                    .lock()
                    .expect("Problem during lock");
                server_stats.total_clients_unserviced += 1;
            }
            let res = fut.await?;
            Ok(res)
        })
    }
}
