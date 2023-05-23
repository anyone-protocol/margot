use anyhow::Result;
use async_trait::async_trait;
use levenshtein::levenshtein;
use std::fmt;
use structopt::StructOpt;
use tor_netdir::NetDir;

use crate::commands::util;
use crate::commands::RunnableOffline;

#[derive(StructOpt)]
pub struct SybilHunterCommand {
    fingerprint: String,
}

impl fmt::Display for SybilHunterCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fingerprint)
    }
}

#[async_trait]
impl RunnableOffline for SybilHunterCommand {
    fn run(&self, netdir: &NetDir) -> Result<()> {
        let reference = util::id2relay(netdir, &self.fingerprint)?;
        let reference_str = util::relay2string(&reference);
        println!("Reference string: {}", reference_str);
        println!("[+] Computing distances...");
        let mut distances: Vec<_> = netdir
            .relays()
            .map(|relay| {
                (
                    levenshtein(&reference_str, &util::relay2string(&relay)),
                    relay,
                )
            })
            .collect();
        distances.sort_by(|a, b| a.0.cmp(&b.0));

        println!("[+] Top 20 closest relays to: {}", self.fingerprint);
        util::print_distances(distances);
        Ok(())
    }
}
