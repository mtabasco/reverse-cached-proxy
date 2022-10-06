use bytes::Bytes;
use endorphin::policy::LazyFixedTTLPolicy;
use endorphin::HashMap;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use std::time::Duration;
use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::Mutex;

async fn handle(
    data: Request<Body>,
    cache: Arc<Mutex<HashMap<String, Bytes, LazyFixedTTLPolicy>>>,
) -> Result<Response<Body>, Infallible> {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    // adquire lock
    let mut x = cache.lock().await;

    // check if the request exsists in the cache
    let cached_request = x.get(&data.uri().to_string());

    let res_data: Bytes;

    let uri_string = data.uri().to_string();

    // no request cached found for this uri
    if cached_request.is_none() {
        // request origin server
        match client.request(data).await {
            // get body's content
            Ok(resp) => match hyper::body::to_bytes(resp).await {
                Ok(body) => {
                    res_data = body;
                    // insert into cache 
                    x.insert(uri_string, res_data.clone(), ());
                }
                Err(e) => {
                    eprintln!("Error parsing request's body ---- {:?}", e);
                    return Ok(Response::new(Body::from("Error parsing request's body")));
                }
            },
            Err(err) => {
                eprintln!("Error forwarding the request: -- {:?}", err);
                return Ok(Response::new(Body::from("Error forwarding the request to origin server")));
            }
        }
    } else {
        // request is cached
        res_data = cached_request.unwrap().to_owned();
    }

    let split = res_data.to_vec();

    Ok(Response::new(Body::from(split)))
}

#[tokio::main]
async fn main() {
    let cache: Arc<Mutex<HashMap<String, Bytes, LazyFixedTTLPolicy>>> = Arc::new(Mutex::new(
        HashMap::new(LazyFixedTTLPolicy::new(Duration::from_secs(10))),
    ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(move |_conn| {
        let cache = cache.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle(req, cache.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
