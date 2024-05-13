use tracing::{info, warn};
use anyhow::Result;
use axum::{
    Router, routing,
    extract::{State, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tower_http::services::ServeDir;
// use tracing_subscriber::fmt::time::ChronoLocal;

use crate::{cli::http::HttpServeOpts, CmdExector};

use std::path::PathBuf;
// use std::path::Path as stdPath;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    // let local_timer = ChronoLocal::new(self.time_format.to_string());
    // tracing_subscriber::fmt::layer()
    //             .with_writer(std::io::stdout.with_max_level(self.level))
    //             .with_file(true)
    //             .with_line_number(true)
    //             .with_target(false)
    //             .with_timer(local_timer);

    info!("Serving {:?} on port {}", path, port);

    let state = HttpServeState { path: path.clone() };
    // let dir_service = ServeDir::new(path)
    //     .append_index_html_on_directories(true)
    //     .precompressed_gzip()
    //     .precompressed_br()
    //     .precompressed_deflate()
    //     .precompressed_zstd();

    let router = Router::new()
        .route("/*path", routing::get(file_handler))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0,0,0,0], port));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            Html("File not found: {}".into())
        )
    } else if p.is_dir() {
        match get_filelist(p.clone()).await {
            Ok(list) => (
                StatusCode::OK,
                Html(list)
            ),
            Err(e) => {
                warn!("Error reading directory: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string()))
            }
        }

    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, Html(content))
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string()))
            }
        }
    }
}

async fn get_filelist(dir: impl AsRef<std::path::Path>) -> Result<String> {
    let mut html = String::new();
    html.push_str("<html><body><ul>");

    for entry in std::fs::read_dir(dir)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>", 
                name, name));
        }
    }

    html.push_str("</ul></body></html>");

    Ok(html)
}

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> Result<()> {
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState { path: PathBuf::from(".") });
        // let path = PathBuf::from("Cargo.toml");

        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        // assert_eq!(content, include_str!("../../Cargo.toml"));
        assert!(content.trim().starts_with("[package]"));
    }
}

