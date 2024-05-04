use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use handlebars::Handlebars;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path).append_index_html_on_directories(true);

    let router = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Serving file {:?}", p);

    match p.exists() {
        true => match p.is_file() {
            true => match tokio::fs::read_to_string(p).await {
                Ok(content) => {
                    info!("Read {} bytes", content.len());
                    (StatusCode::OK, content)
                }
                Err(e) => {
                    warn!("Error reading file: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                }
            },
            false => {
                let dir = match std::fs::read_dir(p.clone()) {
                    Ok(d) => d,
                    Err(e) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to read directory: {}", e),
                        )
                    }
                };

                let mut directories = Vec::new();
                let mut files = Vec::new();

                // 遍历目录和文件
                for entry in dir {
                    let entry = match entry {
                        Ok(e) => e,
                        Err(e) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to read directory entry: {}", e),
                            )
                        }
                    };
                    let path = entry.path();
                    if path.is_dir() {
                        directories.push(path.display().to_string());
                    } else {
                        files.push(path.display().to_string());
                    }
                }

                // 使用 handlebars 渲染
                let mut handlebars = Handlebars::new();
                handlebars
                    .register_template_string(
                        "template",
                        include_str!("../../fixtures/template.html"),
                    )
                    .unwrap();

                let mut data = HashMap::new();
                data.insert("current_path", vec![p.display().to_string()]);
                data.insert("directories", directories);
                data.insert("files", files);

                let rendered = handlebars.render("template", &data).unwrap();

                // let _ = std::fs::write(p.join("index.html"), &rendered);
                info!("Read {} bytes", rendered.len());
                (StatusCode::OK, rendered)
            }
        },
        false => (
            StatusCode::NOT_FOUND,
            format!("File {} note found", p.display()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
