mod routing;
mod store;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use crate::store::Store;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let store = Arc::new(Store::new());

    let make_svc = make_service_fn(|_conn| {
        // service_fn converts our function into a `Service`
        let store = store.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |f| {
                let store = store.clone();

                routing::router(f, store)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}