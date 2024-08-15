mod database;
pub use database::connect_to_database;
pub use database::fetch_users;
pub use database::retrieve_user_by_email;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CurrentUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}
