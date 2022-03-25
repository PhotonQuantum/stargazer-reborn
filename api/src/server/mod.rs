use std::sync::Arc;

use axum::{extract::Extension, routing::post, Json, Router};
use color_eyre::Result;
use http::Method;

use crate::rpc::models::Requests;

#[cfg(test)]
mod test;

mod_use::mod_use![db, config, handler, jwt, context];

pub async fn serve_with_config(config: Config) -> Result<()> {
    let config = Arc::new(config);
    tracing::debug!(config = ?config);
    let db = DB::new(&config).await?;
    let jwt = Arc::new(JWTContext::new(&config));

    let config = Arc::new(config);
    let cors_layer = tower_http::cors::CorsLayer::new()
        // Allow `POST` when accessing the resource
        .allow_methods(vec![Method::POST])
        // Credentials should be passed in as parameter of the request(rpc) body
        .allow_credentials(false)
        // Allow requests from any origin
        .allow_origin(tower_http::cors::Any);

    let trace_layer = tower_http::trace::TraceLayer::new_for_http();

    let app = Router::new()
        .route(
            "/v1",
            post(|Json(req): Json<Requests>, Extension(ctx): Extension<Context>| req.handle(ctx)),
        )
        .layer(Extension(Context { db, jwt, config }))
        .layer(cors_layer)
        .layer(trace_layer)
        .into_make_service();

    tracing::info!("Server starting");

    server.serve(app).await?;

    tracing::info!("Server stopped");

    Ok(())
}

pub async fn serve() -> Result<()> {
    serve_with_config(Config::from_env()?).await
}
