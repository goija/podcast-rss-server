use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

const RSS_URL: &str = "https://www.omnycontent.com/d/playlist/61ee9ca4-a1b2-4660-9651-b2b70035edf5/d643a93a-f161-486c-b8cc-b31501095860/969874ed-2240-43b1-bf81-b3150109586e/podcast.rss";

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let app = Router::new()
        // Koppel de hoofdpagina aan onze HTML
        .route("/", get(serve_html))
        // Koppel de API aan de feed
        .route("/api/feed", get(fetch_feed))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server is gestart!");
    println!("👉 Open je browser en ga naar: http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

// Handler om de HTML-pagina te serveren. 
// include_str! zoekt één niveau omhoog (..) naar de root-map waar index.html staat.
async fn serve_html() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

// Handler om de RSS feed veilig op te halen
async fn fetch_feed() -> Result<Response, StatusCode> {
    match reqwest::get(RSS_URL).await {
        Ok(resp) => {
            if resp.status().is_success() {
                let xml_text = resp.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                let response = (
                    [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
                    xml_text,
                ).into_response();
                
                Ok(response)
            } else {
                Err(StatusCode::BAD_GATEWAY)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}