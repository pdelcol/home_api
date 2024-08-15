use crate::database::{fetch_users, CurrentUser};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use std::sync::Arc;

pub async fn get_users(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> impl IntoResponse {
    let user_list = match fetch_users(Arc::clone(&state.database)).await {
        Ok(users) => users,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    return (StatusCode::OK, Json(Some(user_list)));
}
