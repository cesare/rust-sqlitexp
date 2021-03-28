use std::str::FromStr;

use anyhow::Result;
use sqlx::{ConnectOptions, Executor};
use sqlx::sqlite::SqliteConnectOptions;
use structopt::StructOpt;
use tokio;

#[derive(Debug, StructOpt)]
#[structopt(name = "create_database")]
struct Options {
    #[structopt(short, long, default_value = "sqlite:database.sqlite3")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    let mut connection = SqliteConnectOptions::from_str(&options.url)?
        .create_if_missing(true)
        .connect().await?;
    let query = sqlx::query("
        create table servants (
            id integer primary key,
            name text not null,
            class_name text not null
        );
    ");
    let _results = connection.execute(query).await?;
    Ok(())
}
