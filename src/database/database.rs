use crate::database::CurrentUser;
use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls};
use tracing::error;

pub async fn connect_to_database() -> Result<Client, Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=127.0.0.1 user=postgres dbname=test_db", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn fetch_users(client: Arc<Client>) -> Result<Vec<String>, Error> {
    let result = client.query("SELECT * FROM test_schema.users", &[]).await?;

    let mut users = Vec::<String>::new();

    for row in result {
        match row.try_get::<&str, String>("first_name") {
            Ok(name) => users.push(name),
            Err(e) => {
                error!("Error fetching user! {}", e);
                continue;
            }
        }
    }

    return Ok(users);
}

pub async fn retrieve_user_by_email(email: &str, client: Arc<Client>) -> Option<CurrentUser> {
    let result = client
        .query_one("SELECT * FROM test_schema.users WHERE email=$1", &[&email])
        .await;

    let user: CurrentUser = match result {
        Ok(row) => CurrentUser {
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            password_hash: row.get("password_hash"),
        },
        Err(_) => {
            error!("Error finding user with email {}", email);
            return None;
        }
    };

    Some(user)
}
