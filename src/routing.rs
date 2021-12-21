use std::sync::Arc;
use hyper::{Body, Request, Response, Uri};
use hyper::{Method, StatusCode};
use url::Url;
use crate::store::Store;

pub fn parse_query_value(uri: &Uri, key_name: &str) -> Option<String> {
    let request_url = Url::parse(
        &(format!("{}{}", "http://domain.com", uri.to_string()))
    ).unwrap();
    let params = request_url.query_pairs();

    let mut keys = params.filter(|x| x.0.eq(key_name));

    keys.find(|_f| true).map(|f| f.1.to_string())
}

pub async fn router(_req: Request<Body>, store: Arc<Store>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = _req.into_body();
        }
        (&Method::POST, "/cache") => {
            if let Some(key) = parse_query_value(_req.uri(), "key") {
                let value = hyper::body::to_bytes(_req.into_body()).await?.to_vec();
                store.add(key.to_string(), value);
            }
        }
        (&Method::GET, "/cache") => {
            if let Some(key) = parse_query_value(_req.uri(), "key") {
                *response.body_mut() = match store.get(&key.to_string()) {
                    Some(value) => value.into(),
                    None => "Not Found".into()
                }
            }
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}