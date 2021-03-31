use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use structopt::StructOpt;

use crate::commands::Runnable;

use tor_client::TorClient;

use crate::commands::find;
use crate::commands::util;

#[derive(StructOpt)]
pub struct CountCommand {
    #[structopt(short = "l", long = "list")]
    list: bool,
    /// The filters of this command.
    filters: Vec<find::FindFilter>,
}

impl fmt::Display for CountCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.filters)
    }
}

#[async_trait]
impl Runnable for CountCommand {
    async fn run(&self, tor_client: &TorClient) -> Result<()> {
        let netdir = tor_client.dirmgr().netdir().await;

        // We'll go filter by filter and then do a final count of all filters.
        for filter in &self.filters {
            // XXX: Need to do this for every filter since we can't copy the
            // object inside the Vec (tor_netdir::Relay) and we filter it in
            // place using the find command.
            let mut relays: Vec<_> = netdir.relays().collect();
            let mut find = find::FindCommand::new();
            find.add(filter);
            find.filter(&mut relays);
            println!("[+] {} relays match: {:?}", relays.len(), filter);
            if self.list {
                util::describe_relays(&relays, true, 4);
            }
        }

        // Count relays with matching all filters together.
        let mut relays: Vec<_> = netdir.relays().collect();
        let mut find = find::FindCommand::new();
        find.add_many(&self.filters);
        find.filter(&mut relays);
        println!("[+] {} relays matched all", relays.len());

        Ok(())
    }
}
