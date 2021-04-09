mod config;
mod count;
mod err;
mod find;
mod like;
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
        let netdir = tor_client.dirmgr().netdir();
        self.run(&netdir)
    }
}

#[derive(StructOpt)]
pub enum SubCommand {
    #[structopt(name = "config", about = "Create configuration entries")]
    Config(config::ConfigCommand),
    #[structopt(name = "count", about = "Count relay(s) in the consensus")]
    Count(count::CountCommand),
    #[structopt(name = "find", about = "Find relay(s) in the consensus")]
    Find(find::FindCommand),
    #[structopt(name = "like", about = "Match alike relay(s) in the consensus")]
    Like(like::LikeCommand),
    #[structopt(name = "sybil", about = "Sybil testing")]
    Sybil(sybil::SybilCommand),
    #[structopt(name = "test", about = "Run test(s) on one or many relay(s)")]
    Test(test::TestCommand),
}

impl SubCommand {
    fn cmd(&self) -> Box<&(dyn Runnable + Send + Sync)> {
        Box::new(match self {
            SubCommand::Config(c) => c,
            SubCommand::Count(c) => c,
            SubCommand::Find(c) => c,
            SubCommand::Like(c) => c,
            SubCommand::Sybil(c) => c,
            SubCommand::Test(c) => c,
        })
    }
}

#[async_trait]
impl Runnable for SubCommand {
    async fn run(&self, tor_client: &tor_client::TorClient) -> Result<()> {
        self.cmd().run(tor_client).await
    }
}
