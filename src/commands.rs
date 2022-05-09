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
use tor_rtcompat::Runtime;

#[async_trait]
pub trait Runnable<R: Runtime> {
    async fn run(&self, arti_client: &arti_client::TorClient<R>) -> Result<()>;
}

pub trait RunnableOffline {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()>;
}

#[async_trait]
impl<T: RunnableOffline + Send + Sync, R: Runtime> Runnable<R> for T {
    async fn run(&self, arti_client: &arti_client::TorClient<R>) -> Result<()> {
        let netdir = arti_client.dirmgr().timely_netdir()?;
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
    fn cmd<R: Runtime>(&self) -> &(dyn Runnable<R> + Send + Sync) {
        match self {
            SubCommand::Config(c) => c,
            SubCommand::Count(c) => c,
            SubCommand::Find(c) => c,
            SubCommand::Like(c) => c,
            SubCommand::Sybil(c) => c,
            SubCommand::Test(c) => c,
        }
    }
}

#[async_trait]
impl<R: Runtime> Runnable<R> for SubCommand {
    async fn run(&self, arti_client: &arti_client::TorClient<R>) -> Result<()> {
        self.cmd().run(arti_client).await
    }
}
