mod count;
mod err;
mod find;
mod sybil;
mod test;
mod util;

use anyhow::Result;
use async_trait::async_trait;
use structopt::StructOpt;

use tor_client;
use tor_netdir;

#[async_trait]
pub trait Runnable {
    async fn run(&self, tor_client: &tor_client::TorClient) -> Result<()>;
}

pub trait RunnableOffline {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()>;
}

#[async_trait]
impl<T: RunnableOffline + Send + Sync> Runnable for T {
    async fn run(&self, tor_client: &tor_client::TorClient) -> Result<()> {
        let netdir = tor_client.dirmgr().netdir().await;
        self.run(&netdir)
    }
}

#[derive(StructOpt)]
pub enum SubCommand {
    #[structopt(name = "count", about = "Count relay(s) in the consensus")]
    Count(count::CountCommand),
    #[structopt(name = "find", about = "Find relay(s) in the consensus")]
    Find(find::FindCommand),
    #[structopt(name = "sybil", about = "Sybil testing")]
    Sybil(sybil::SybilCommand),
    #[structopt(name = "test", about = "Run test(s) on one or many relay(s)")]
    Test(test::TestCommand),
}

impl SubCommand {
    fn cmd(&self) -> Box<&(dyn Runnable + Send + Sync)> {
        match self {
            SubCommand::Count(c) => Box::new(c),
            SubCommand::Find(c) => Box::new(c),
            SubCommand::Sybil(c) => Box::new(c),
            SubCommand::Test(c) => Box::new(c),
        }
    }
}

#[async_trait]
impl Runnable for SubCommand {
    async fn run(&self, tor_client: &tor_client::TorClient) -> Result<()> {
        self.cmd().run(tor_client).await
    }
}
