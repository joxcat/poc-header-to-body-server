use std::net::SocketAddr;
use std::env;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use hyper::{Server, Response, Request, Body};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if let Some(content) = req.headers().get("X-Content") {
        Ok(Response::new(Body::from(
            content.to_str().expect("Cannot convert X-Content to string").to_owned()
        )))
    } else {
        Ok(Response::new(Body::empty()))
    }
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>().expect("Cannot convert `PORT` env variable to integer");

    let addr = SocketAddr::from(([127,0,0,1], port));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("Server error {}", e);
    }
}
