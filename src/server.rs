use crate::app_config::HttpConfig;
use crate::http::{AppState, router};
use std::io::Error;
use tracing::info;

pub async fn init_server(config: &HttpConfig, state: AppState) -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind(config.url()).await?;

    let routes = router(state);

    info!("Starting server on {}", config.url());

    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}
