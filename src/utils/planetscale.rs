use crate::utils::env_utils::get_env_var;
use anyhow::Error;
use planetscale_driver::{query, PSConnection};
use serde_json::Value;

async fn ps_client() -> Result<PSConnection, Error> {
    let host = get_env_var("DATABASE_HOST")?;
    let username = get_env_var("DATABASE_USERNAME")?;
    let password = get_env_var("DATABASE_PASSWORD")?;

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    Ok(conn)
}
