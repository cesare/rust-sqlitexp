use std::str::FromStr;

use anyhow::Result;
use sqlx::{Connection, ConnectOptions, Executor};
use sqlx::sqlite::SqliteConnectOptions;
use structopt::StructOpt;
use tokio;

#[derive(Debug, StructOpt)]
#[structopt(name = "list_servants")]
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
    let query = sqlx::query("select id, name, class_name from servants");
    let results = connection.execute(query).await?;
    println!("{:?}", results);
    connection.close().await?;

    Ok(())
}
