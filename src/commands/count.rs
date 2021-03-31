use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use structopt::StructOpt;

use crate::commands::find;
use crate::commands::util;
use crate::commands::RunnableOffline;

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
impl RunnableOffline for CountCommand {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()> {
        // We'll go filter by filter and then do a final count of all filters.
        for filter in &self.filters {
            let count = netdir.relays().filter(|r| filter.match_relay(r)).count();
            println!("[+] {} relays match: {:?}", count, filter);
            if self.list {
                let find = find::FindCommand::new(&vec![filter.clone()]);
                let relays = find.filter(netdir);
                util::describe_relays(&relays, true, 4);
            }
        }

        // Count relays with matching all filters together.
        let find = find::FindCommand::new(&self.filters);
        println!("[+] {} relays matched all", find.count(netdir));

        Ok(())
    }
}
