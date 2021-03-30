use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use serde_derive::Deserialize;
use sqlx::{Connection, ConnectOptions};
use sqlx::sqlite::SqliteConnectOptions;
use structopt::StructOpt;
use tokio::{self, fs::File, io::AsyncReadExt};

#[derive(Debug, StructOpt)]
#[structopt(name = "list_servants")]
struct Options {
    #[structopt(short, long, default_value = "sqlite:database.sqlite3")]
    url: String,

    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    servants: Vec<Servant>
}

#[derive(Debug, Deserialize)]
struct Servant {
    name: String,
    class_name: String,
}

async fn read_config(options: &Options) -> Result<Vec<Servant>> {
    let mut file = File::open(&options.input).await?;
    let mut source = String::new();
    file.read_to_string(&mut source).await?;

    let config: Config = toml::from_str(&source)?;
    Ok(config.servants)
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    let servants = read_config(&options).await?;

    let mut connection = SqliteConnectOptions::from_str(&options.url)?
        .connect().await?;
    for servant in servants {
        println!("Loading {:?}", servant);

        let query = sqlx::query("insert into servants (name, class_name) values (?, ?)")
            .bind(&servant.name)
            .bind(&servant.class_name);
        let results = query.execute(&mut connection).await?;
        println!("{:?}", results);
    }
    connection.close().await?;

    Ok(())
}
