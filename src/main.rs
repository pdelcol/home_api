use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::error;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/ts", get(health_check));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("App is running");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn health_check() -> (StatusCode, Json<Option<Timestamp>>) {
    let start = SystemTime::now();
    let since_the_epoch = match start.duration_since(UNIX_EPOCH) {
        Ok(time) => time,
        Err(_) => {
            error!("time went backwards");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    let timestamp = Timestamp {
        ts: since_the_epoch.as_secs(),
    };

    return (StatusCode::OK, Json(Some(timestamp)));
}

#[derive(Serialize)]
struct Timestamp {
    ts: u64,
}
