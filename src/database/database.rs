use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls};
use tracing::error;

pub async fn connect_to_database() -> Result<Client, Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=127.0.0.1 user=postgres dbname=test_db", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
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
        match row.try_get::<&str, String>("name") {
            Ok(name) => users.push(name),
            Err(e) => {
                error!("Error fetching user! {}", e);
                return Err(e);
            }
        }
    }

    return Ok(users);
}
