use {
    rocket::{
        http::{ContentType, Method, Status},
        route::{Handler, Outcome},
        Data, Request, Response, Route,
    },
    serde::Deserialize,
    std::{io::Cursor, path::PathBuf},
};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Deployment {
    hook: String,
    repo: String,
    key: Option<PathBuf>,
    path: PathBuf,
}

impl Deployment {
    pub(crate) fn into_route(self) -> Route {
        let hook = self.hook.clone();
        Route::new(Method::Get, hook.as_ref(), self)
    }
}

#[rocket::async_trait]
impl Handler for Deployment {
    async fn handle<'r>(&self, _req: &'r Request<'_>, _data: Data<'r>) -> Outcome<'r> {
        // TODO: Deploy the project
        let body = format!("{:?}", &self);
        let res = Response::build()
            .status(Status::Ok)
            .header(ContentType::Plain)
            .sized_body(body.len(), Cursor::new(body))
            .finalize();
        Outcome::Success(res)
    }
}
