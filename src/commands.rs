mod err;
mod count;
mod find;
mod util;

use anyhow::Result;
use async_trait::async_trait;
use structopt::StructOpt;

use tor_client::TorClient;

#[async_trait]
pub trait Runnable {
    async fn run(&self, tor_client: &TorClient) -> Result<()>;
}

#[derive(StructOpt)]
pub enum SubCommand {
    #[structopt(name = "count", about = "Count relay(s) in the consensus")]
    Count(count::CountCommand),
    #[structopt(name = "find", about = "Find relay(s) in the consensus")]
    Find(find::FindCommand),
}

#[async_trait]
impl Runnable for SubCommand {
    async fn run(&self, tor_client: &TorClient) -> Result<()> {
        match self {
            SubCommand::Count(c) => Ok(c.run(tor_client).await?),
            SubCommand::Find(c) => Ok(c.run(tor_client).await?),
        }
    }
}
