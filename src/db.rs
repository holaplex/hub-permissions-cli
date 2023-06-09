use serde::Deserialize;
use tokio_postgres::{Config, NoTls};

use crate::prelude::error;

#[derive(Debug, Deserialize)]
pub struct Instance {
    ip: String,
    port: u16,
    pub database: String,
    username: String,
    password: String,
}

impl Instance {
    pub async fn connect(config: &Self) -> tokio_postgres::Client {
        let mut pg_config = Config::new();
        pg_config
            .user(&config.username)
            .password(&config.password)
            .host(&config.ip)
            .port(config.port)
            .dbname(&config.database);

        let (client, connection) = pg_config.connect(NoTls).await.unwrap_or_else(|_| {
            error!("Unable to connect to DB");
            std::process::exit(1)
        });

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("Database connection error: {}", e);
            }
        });

        client
    }
}
