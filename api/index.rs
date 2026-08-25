
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Dit start de asynchrone Vercel runtime
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // Hier bouw je het antwoord (HTTP 200 OK)
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::Text(
            r#"{ "status": "succes", "bericht": "Rust endpoint op Vercel is actief!" }"#.into()
        ))?)
}
