use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
mod database;
use serde::Serialize;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio_postgres::Client;
use tracing::error;
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
        .route("/users", get(check_database_call))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> (StatusCode, Json<Option<Timestamp>>) {
    let start = SystemTime::now();
    let since_the_epoch = match start.duration_since(UNIX_EPOCH) {
        Ok(time) => time,
        Err(_) => {
            error!("Time went backwards");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    let timestamp = Timestamp {
        ts: since_the_epoch.as_secs(),
    };

    return (StatusCode::OK, Json(Some(timestamp)));
}

async fn check_database_call(
    State(state): State<AppState>,
) -> (StatusCode, Json<Option<Vec<String>>>) {
    let user_list = match database::fetch_users(Arc::clone(&state.database)).await {
        Ok(users) => users,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    return (StatusCode::OK, Json(Some(user_list)));
}

#[derive(Serialize)]
struct Timestamp {
    ts: u64,
}
