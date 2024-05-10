use tracing::info;
use std::path::Path;

pub fn process_http_serve(path: &Path, port: u16) {
    info!("Serving {:?} on port {}", path, port);
}