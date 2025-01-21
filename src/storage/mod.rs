use sqlx::migrate::Migrator;
use sqlx::{Connection, SqliteConnection};

mod types;

pub use types::*;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn connect() -> SqliteConnection {
    let mut conn = SqliteConnection::connect("output.sqlite?mode=rwc")
        .await
        .unwrap();
    MIGRATOR.run(&mut conn).await.unwrap();
    conn
}
