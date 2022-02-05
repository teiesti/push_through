use {
    crate::config::Deployment,
    anyhow::{bail, Error},
    hyper::{service::Service, Body, Request, Response},
    std::{
        collections::HashMap,
        future::Future,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
};

pub(crate) struct PushThrough {
    deployments: Arc<HashMap<String, Deployment>>,
}

impl PushThrough {
    pub(crate) fn for_deployments(deployments: Arc<HashMap<String, Deployment>>) -> Self {
        PushThrough { deployments }
    }
}

impl Service<Request<Body>> for PushThrough {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        // TODO
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        // TODO
        let path = request.uri().path();
        let response = match self.deployments.get(path) {
            Some(deployment) => Response::builder().body(Body::from(format!("{:?}", deployment))),
            None => Response::builder()
                .status(404)
                .body(Body::from("404 Not Found")),
        };
        Box::pin(async { Ok(response.unwrap()) })
    }
}
