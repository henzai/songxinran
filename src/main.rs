use ed25519_compact::{Error, PublicKey, Signature};
use hyper::body::HttpBody;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use model::{Interaction, InteractionResponse, InteractionResponseType, InteractionType};
use std::env;

mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 8080).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(route)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn route(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => match validate(&mut req).await {
            Ok(_) => handler(req).await,
            Err(_) => unauthorized(),
        },
        _ => unauthorized(),
    }
}

async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let b = hyper::body::to_bytes(req).await?;
    let i: Interaction = serde_json::from_slice(b.as_ref()).unwrap();

    let oo: Result<hyper::Response<hyper::Body>, hyper::Error> = Ok(InteractionResponse {
        interaction_response_type: InteractionResponseType::Pong,
        data: None,
    }
    .into_response());

    match i.interaction_type {
        InteractionType::Ping => oo,
        InteractionType::ApplicationCommand => oo,
    }
}

fn unauthorized() -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::UNAUTHORIZED;
    Ok(not_found)
}

async fn validate(req: &mut Request<Body>) -> Result<(), Error> {
    let (decoded_header, ts) = signature(req);
    let sss = req.body_mut().data().await;
    let aaa = sss.unwrap().unwrap();
    let content = ts.iter().chain(aaa.iter()).cloned().collect::<Vec<u8>>();

    let pk =
        PublicKey::from_slice(&hex::decode(env::var("DISCORD_PUBLIC_KEY").expect("")).expect(""))
            .expect("");

    let sig = Signature::from_slice(&decoded_header).expect("");

    pk.verify(content, &sig)
}

fn signature(req: &Request<Body>) -> (Vec<u8>, Vec<u8>) {
    let headers = req.headers();
    let sig = headers.get("X-Signature-Ed25519").expect("");
    let ts = headers.get("X-Signature-Timestamp").unwrap();
    let sss: Vec<u8> = ts.to_str().unwrap().as_bytes().to_vec();

    let sig = hex::decode(sig).expect("");
    (sig, sss)
}
