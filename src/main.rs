mod commands;
mod opts;

use anyhow::Result;
use std::sync::Arc;
use structopt::StructOpt;

use crate::commands::Runnable;
use arti_client::{TorClient, TorClientConfig};
use tokio_crate as tokio;
// use tor_rtcompat::{Runtime};

#[macro_use]
extern crate prettytable;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = opts::Opts::from_args();
    let config = TorClientConfig::default();
    let arti_client = Arc::new(TorClient::create_bootstrapped(config).await?);
    opts.subcommand.run(&arti_client).await
}
