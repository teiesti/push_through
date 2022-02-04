use {
    hyper::{service::Service, Body, Request, Response},
    std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    },
};

pub(crate) struct PushThrough {
    // TODO
}

impl Service<Request<Body>> for PushThrough {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        // TODO
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        // TODO
        let response = Ok(Response::builder()
            .body(Body::from(format!("{:#?}", request)))
            .unwrap());
        Box::pin(async { response })
    }
}
