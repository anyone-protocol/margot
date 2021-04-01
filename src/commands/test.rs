use anyhow::Result;
use async_trait::async_trait;
use rand::prelude::*;
use std::fmt;
use structopt::StructOpt;

use crate::commands::err::Error;
use crate::commands::util;
use crate::commands::Runnable;

use tor_circmgr;

#[derive(Debug, Clone, StructOpt)]
pub struct ExtendCommand {
    fingerprint: util::RelayFingerprint,
}

impl ExtendCommand {
    async fn extend(&self, tor_client: &tor_client::TorClient) -> Result<()> {
        let netdir = tor_client.dirmgr().netdir().await;
        let relay = netdir
            .relays()
            .find(|r| self.fingerprint.match_relay(&r))
            .ok_or(Error::RelayNotFound(self.fingerprint.to_string()))?;

        let path = tor_circmgr::path::TorPath::OneHop(relay);

        let dirinfo: tor_circmgr::DirInfo = netdir.as_ref().into();
        let mut rng = StdRng::from_rng(rand::thread_rng()).expect("Unable to build RNG");
        let circ = tor_client
            .circmgr()
            .build_path(&mut rng, dirinfo, &path)
            .await;
        match circ {
            Err(e) => println!("[-] Unable to extend: {}", e),
            Ok(_) => println!("[+] Successful one hop to: {}", self.fingerprint),
        };
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub enum TestSubCommand {
    #[structopt(name = "extend", about = "Extend to a relay")]
    Extend(ExtendCommand),
}

#[derive(StructOpt)]
pub struct TestCommand {
    #[structopt(subcommand)]
    pub subcommand: TestSubCommand,
}

impl fmt::Display for TestCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.subcommand)
    }
}

#[async_trait]
impl Runnable for TestCommand {
    async fn run(&self, tor_client: &tor_client::TorClient) -> Result<()> {
        Ok(match &self.subcommand {
            TestSubCommand::Extend(c) => c.extend(tor_client).await?,
        })
    }
}
