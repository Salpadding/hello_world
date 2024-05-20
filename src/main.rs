mod config;
mod err;

use config::Config;

use axum::{extract::State, routing::get, Router};
use tokio::net::tcp::ReuniteError;
use std::sync::Arc;

async fn clash(State(config): State<Arc<Config>>) -> Result<String, err::AppError> {
    // clash 订阅
    let config = config.as_ref();
    let sub: &str = &config.clash.subscribe;
    let resp = reqwest::get(sub).await?;
    let bytes = resp.bytes().await?;
    let mut value: serde_yml::Value = serde_yml::from_slice(&bytes)?;

    config.clash.patch_rules(&mut value)?;
    Ok(serde_yml::to_string(&value)?)
}

#[tokio::main]
async fn main() {
    let cfg_path = std::env::var("APP_CONFIG")
        .unwrap_or("config/config.yaml".to_string());
    
    let cfg: config::Config = match std::fs::File::open(&cfg_path) {
        Ok(file) => serde_yml::from_reader(file).unwrap(),
        Err(_) => Config::default(),
    };

    let cfg = Arc::new(cfg);

    let app = Router::new()
        .route(&cfg.clash.path, get(clash))
        .with_state(Arc::clone(&cfg));

    let listener = tokio::net::TcpListener::bind(&cfg.server.bind)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
