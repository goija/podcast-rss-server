use axum::{
<<<<<<< HEAD
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
=======
    routing::get,
    response::IntoResponse,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/rss.xml", get(rss_handler))
        .nest_service("/podcasts", ServeDir::new("./podcasts"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🎙️ VORTEX RSS Podcast Server gestart op http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn rss_handler() -> impl IntoResponse {
    let podcast_dir = "./podcasts";
    let mut items_xml = String::new();

    if let Ok(entries) = std::fs::read_dir(podcast_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("mp3") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let mut title = filename.clone();

                if let Ok(tag) = id3::Tag::read_from_path(&path) {
                    if let Some(t) = tag.title() {
                        title = t.to_string();
                    }
                }

                let metadata = std::fs::metadata(&path).unwrap();
                let length = metadata.len();

                items_xml.push_str(&format!(
                    r#"
                    <item>
                        <title>{}</title>
                        <enclosure url="http://localhost:8080/podcasts/{}" length="{}" type="audio/mpeg"/>
                        <guid>{}</guid>
                    </item>
                    "#,
                    title, filename, length, filename
                ));
            }
        }
    }

    let rss_feed = format!(
        r#"<?xml version="1.0" encoding="UTF-8" ?>
        <rss version="2.0">
            <channel>
                <title>VORTEX Sovereign Podcast Feed</title>
                <link>http://localhost:8080/rss.xml</link>
                <description>Automatisch gegenereerde lokale podcast feed</description>
                <language>nl-NL</language>
                {}
            </channel>
        </rss>"#,
        items_xml
    );

    ([(axum::http::header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")], rss_feed)
}
>>>>>>> a5374f3 (feat: robuuste bestandsnaam-naar-titel parser en axum rss server)
