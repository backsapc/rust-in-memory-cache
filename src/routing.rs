use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;

pub async fn router(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = _req.into_body();
        },
        // Yet another route inside our match block...
        (&Method::POST, "/echo/uppercase") => {
            // This is actually a new `futures::Stream`...
            let mapping = _req
                .into_body()
                .map_ok(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });

            // Use `Body::wrap_stream` to convert it to a `Body`...
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        (&Method::POST, "/echo/reverse") => {
            let full_body = hyper::body::to_bytes(_req.into_body()).await?;

            let string_body = String::from_utf8(full_body.to_vec())
                .map(|x| x.chars()
                    .rev()
                    .collect::<String>());

            *response.body_mut() = string_body.unwrap().as_bytes().to_vec().into();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}