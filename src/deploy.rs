use {
    rocket::{
        http::{ContentType, Status},
        route::{Handler, Outcome},
        Data, Request, Response,
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

#[rocket::async_trait]
impl Handler for Deployment {
    async fn handle<'r>(&self, _req: &'r Request<'_>, _data: Data<'r>) -> Outcome<'r> {
        let body = format!("{:?}", &self);
        let res = Response::build()
            .status(Status::Ok)
            .header(ContentType::Plain)
            .sized_body(body.len(), Cursor::new(body))
            .finalize();
        Outcome::Success(res)
    }
}
