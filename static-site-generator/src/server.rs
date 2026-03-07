use anyhow::Result;
use axum::{
    Router,
    routing::{get, get_service},
    response::IntoResponse,
    http::StatusCode,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;

const RELOAD_SCRIPT: &str = r#"
<script>
(function() {
    let lastCheck = Date.now();

    async function checkForUpdates() {
        try {
            const response = await fetch('/__reload_check');
            const data = await response.json();

            if (data.should_reload && data.timestamp > lastCheck) {
                console.log('Reloading page due to changes...');
                location.reload();
            }
            lastCheck = Date.now();
        } catch (e) {
            console.error('Reload check failed:', e);
        }
    }

    setInterval(checkForUpdates, 1000);
})();
</script>
"#;

pub struct DevServer {
    output_dir: PathBuf,
    port: u16,
    last_build: std::time::SystemTime,
}

impl DevServer {
    pub fn new(output_dir: PathBuf, port: u16) -> Self {
        Self {
            output_dir,
            port,
            last_build: std::time::SystemTime::now(),
        }
    }

    pub fn update_build_time(&mut self) {
        self.last_build = std::time::SystemTime::now();
    }

    pub async fn run(self) -> Result<()> {
        let app = Router::new()
            .route("/__reload_check", get(Self::reload_check))
            .fallback_service(
                get_service(ServeDir::new(&self.output_dir))
                    .handle_error(|error| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled error: {}", error),
                        )
                    }),
            );

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        println!("Dev server running at http://{}", addr);
        println!("Press Ctrl+C to stop");

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    async fn reload_check() -> impl IntoResponse {
        // This is a simple implementation. In a real scenario, you'd track build times
        let response = serde_json::json!({
            "should_reload": false,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        });

        (StatusCode::OK, axum::Json(response))
    }
}

pub fn inject_reload_script(html: &str) -> String {
    if let Some(pos) = html.rfind("</body>") {
        let mut result = html.to_string();
        result.insert_str(pos, RELOAD_SCRIPT);
        result
    } else {
        html.to_string()
    }
}
