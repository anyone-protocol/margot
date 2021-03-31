#![feature(drain_filter)]

mod commands;
mod opts;

use crate::commands::Runnable;

use anyhow::Result;
use std::sync::Arc;
use structopt::StructOpt;
#[macro_use]
extern crate prettytable;

use tor_client;
use tor_dirmgr;

async fn handle_command(opts: &opts::Opts) -> Result<()> {
    let mut builder = tor_dirmgr::NetDirConfigBuilder::new();
    builder.use_default_cache_path()?;
    let config: tor_dirmgr::NetDirConfig = builder.finalize()?;

    let tor_client = Arc::new(tor_client::TorClient::bootstrap(config).await?);

    Ok(opts.subcommand.run(&tor_client).await?)
}

fn main() -> Result<()> {
    let opts = opts::Opts::from_args();

    tor_rtcompat::task::block_on(async { handle_command(&opts).await })
}
