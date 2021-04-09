use anyhow::Result;
use async_trait::async_trait;
use levenshtein::levenshtein;
use std::collections::HashMap;
use std::fmt;
use structopt::StructOpt;

use crate::commands::RunnableOffline;

#[derive(StructOpt)]
pub struct LikeCommand {
    name: String,
}

impl fmt::Display for LikeCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[async_trait]
impl RunnableOffline for LikeCommand {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()> {
        println!("[+] Getting relays from consensus");
        let relays = netdir.relays();
        let mut distances = HashMap::new();
        println!("[+] Computing nickname distances...");
        let mut count = 0;
        for relay in relays {
            count += 1;
            if (count % 1000) == 0 {
                println!(" :: {} relays processed", count);
            }
            distances.insert(relay.rsa_id().to_string(), levenshtein(&self.name.as_str(), relay.rs().nickname().as_str()));
        }
        println!("[+] Top 5 closest nicknames to: {}", self.name);
        let mut d_vec: Vec<(&String, &usize)> = distances.iter().collect();
        d_vec.sort_by(|a, b| a.1.cmp(b.1));
        for item in d_vec.iter().take(5) {
            println!(" -> {}: {}", item.0, item.1);
        }
        Ok(())
    }
}
