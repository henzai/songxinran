use ed25519_compact::{Error, PublicKey, Signature};
use hyper::body::{Bytes, HttpBody};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
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
async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            let (pt, mut bd) = req.into_parts();
            let bybd = bd.data().await.unwrap().unwrap();
            match validate(&bybd, &pt.headers).await {
                Ok(_) => handler(&bybd).await,
                Err(_) => unauthorized(),
            }
        }
        _ => unauthorized(),
    }
}

async fn validate(body: &Bytes, hd: &HeaderMap) -> Result<(), Error> {
    let (decoded_header, ts) = signature(hd);
    let content = ts
        .iter()
        .chain(body.iter().clone())
        .cloned()
        .collect::<Vec<u8>>();

    let pk =
        PublicKey::from_slice(&hex::decode(env::var("DISCORD_PUBLIC_KEY").expect("")).expect(""))
            .expect("");

    let sig = Signature::from_slice(&decoded_header).expect("");
    pk.verify(content, &sig)
}

fn signature(hd: &HeaderMap) -> (Vec<u8>, Vec<u8>) {
    let sig = hd.get("X-Signature-Ed25519").expect("");
    let ts = hd.get("X-Signature-Timestamp").unwrap();
    let sss: Vec<u8> = ts.to_str().unwrap().as_bytes().to_vec();

    let sig = hex::decode(sig).expect("");
    (sig, sss)
}

async fn handler(body: &Bytes) -> Result<Response<Body>, hyper::Error> {
    let i: Interaction = serde_json::from_slice(body.as_ref()).unwrap();

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
