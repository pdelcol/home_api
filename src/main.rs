use axum::{http::StatusCode, middleware, routing::get, routing::post, Json, Router};
mod auth;
mod database;
mod user;
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;
use tokio_postgres::Client;
use tracing_subscriber;

#[derive(Clone)]
struct AppState {
    database: Arc<Client>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Arc::new(
        database::connect_to_database()
            .await
            .expect("problem connecting"),
    );

    let state = AppState { database: db };

    let app = Router::new()
        .route("/ts", get(health_check))
        .route("/signin", post(auth::sign_in))
        .route(
            "/user",
            get(user::get_users).layer(middleware::from_fn_with_state(
                state.clone(),
                auth::authorize,
            )),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> (StatusCode, Json<Option<Timestamp>>) {
    let timestamp = Timestamp {
        ts: Utc::now().timestamp(),
    };

    return (StatusCode::OK, Json(Some(timestamp)));
}

#[derive(Serialize)]
struct Timestamp {
    ts: i64,
}
