use axum::{
    body::Body,
    http::{header, HeaderValue, Response, StatusCode, Uri},
    response::IntoResponse,
    Router,
};
use rust_embed::Embed;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Embed)]
#[folder = "../web/dist/"]
struct Assets;

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let raw_path = uri.path().trim_start_matches('/');
    let path = if raw_path.is_empty() {
        "index.html"
    } else {
        raw_path
    };

    // 1. Try exact file match
    if let Some(content) = Assets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        let cache_control = if path.starts_with("assets/") {
            "public, max-age=31536000, immutable"
        } else {
            "no-cache"
        };

        return Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime.as_ref())
                    .unwrap_or(HeaderValue::from_static("application/octet-stream")),
            )
            .header(header::CACHE_CONTROL, HeaderValue::from_static(cache_control))
            .body(Body::from(content.data))
            .unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap()
            });
    }

    // 2. Check if this is a request for a static asset with a file extension
    // If an asset is missing, return 404 rather than serving index.html
    let has_asset_extension = path
        .rsplit_once('.')
        .map(|(_, ext)| !ext.eq_ignore_ascii_case("html"))
        .unwrap_or(false);

    if has_asset_extension {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .body(Body::from(format!("404 Not Found: /{}", path)))
            .unwrap();
    }

    // 3. SPA Fallback: Serve index.html for client-side routing
    if let Some(index) = Assets::get("index.html") {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )
            .header(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"))
            .body(Body::from(index.data))
            .unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap()
            })
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .body(Body::from("404 Not Found: Embedded frontend assets missing"))
            .unwrap()
    }
}

async fn bind_listener() -> std::io::Result<(TcpListener, SocketAddr)> {
    // Check if custom port specified via environment
    if let Ok(port_str) = std::env::var("PORT").or_else(|_| std::env::var("HOMECALC_PORT")) {
        if let Ok(port) = port_str.parse::<u16>() {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            if let Ok(listener) = TcpListener::bind(addr).await {
                let local_addr = listener.local_addr()?;
                return Ok((listener, local_addr));
            }
        }
    }

    // Attempt preferred ports in order
    let candidate_ports = [8080, 8081, 8082, 3000, 5173, 0];
    for &port in &candidate_ports {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if let Ok(listener) = TcpListener::bind(addr).await {
            let local_addr = listener.local_addr()?;
            return Ok((listener, local_addr));
        }
    }

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let local_addr = listener.local_addr()?;
    Ok((listener, local_addr))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    println!("\nGracefully shutting down HomeCalc server...");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (listener, addr) = bind_listener().await?;
    let url = format!("http://127.0.0.1:{}", addr.port());

    println!("====================================================");
    println!("  HomeCalc Standalone Launcher");
    println!("  Server running at: {}", url);
    println!("  Press Ctrl+C to stop.");
    println!("====================================================");

    // Open the default browser
    let open_url = url.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        println!("Opening default browser at {} ...", open_url);
        if let Err(e) = open::that(&open_url) {
            eprintln!("Note: Could not open browser automatically ({e}).");
            eprintln!("Please visit {} directly in your browser.", open_url);
        }
    });

    let app = Router::new().fallback(static_handler);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
