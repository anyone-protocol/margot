#![feature(drain_filter)]

mod commands;
mod opts;

use crate::commands::Runnable;

use tor_rtcompat::{Runtime,SpawnBlocking};

use anyhow::Result;
use std::sync::Arc;
use structopt::StructOpt;
#[macro_use]
extern crate prettytable;

async fn handle_command<R: Runtime>(runtime: &R, opts: &opts::Opts) -> Result<()> {
    let mut builder = tor_dirmgr::DirMgrConfigBuilder::new();
    builder.use_default_cache_path()?;
    let config: tor_dirmgr::DirMgrConfig = builder.build()?;

    let tor_client = Arc::new(tor_client::TorClient::bootstrap(runtime.clone(), config).await?);

    Ok(opts.subcommand.run(&tor_client).await?)
}

fn main() -> Result<()> {
    let opts = opts::Opts::from_args();

    let runtime = tor_rtcompat::create_runtime()?;
    let rt_copy = runtime.clone();

    rt_copy.block_on(async { handle_command(&runtime, &opts).await })
}
