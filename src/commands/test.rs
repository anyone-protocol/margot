use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use structopt::StructOpt;

use tor_chanmgr::ChannelUsage;
use tor_proto::circuit::CircParameters;
use tor_rtcompat::Runtime;

use crate::commands::find;
use crate::commands::Runnable;

#[derive(Debug, Clone, StructOpt)]
pub struct ExtendCommand {
    /// The filters of this command.
    filters: Vec<find::FindFilter>,
}

impl ExtendCommand {
    async fn extend<R: Runtime>(
        &self,
        arti_client: &arti_client::TorClient<R>,
    ) -> Result<()> {
        let mut found: bool = false;
        let find = find::FindCommand::new(&self.filters);
        let netdir = arti_client.dirmgr().timely_netdir().unwrap();
        let relays_iter = netdir.relays().filter(|r| find.match_relay(r));

        for relay in relays_iter {
            found = true;
            // We take a copy of the fingerprint and nickname for later
            // printing because we loose ownership of the relay object once it
            // is in the TorPath.
            let fp =
                relay.rsa_id().to_string().replace('$', "").to_uppercase();
            let nickname = relay.rs().nickname().to_string();
            let path = tor_circmgr::path::TorPath::new_one_hop(relay);
            let params = CircParameters::default();
            let usage = ChannelUsage::UselessCircuit;
            let circ = arti_client
                .circmgr()
                .builder()
                .build(&path, &params, usage)
                .await;

            match circ {
                Err(e) => println!("[-] Unable to extend: {}", e),
                Ok(_) => println!(
                    "[+] Successful one hop to: {} - {}",
                    nickname, fp
                ),
            };
        }
        if !found {
            println!("[-] No relays matching filters: {:?}", self.filters);
        }
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
impl<R: Runtime> Runnable<R> for TestCommand {
    async fn run(
        &self,
        arti_client: &arti_client::TorClient<R>,
    ) -> Result<()> {
        match &self.subcommand {
            TestSubCommand::Extend(c) => c.extend(arti_client).await?,
        };
        Ok(())
    }
}
