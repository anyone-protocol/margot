use anyhow::Result;
use async_trait::async_trait;
use ipnetwork::IpNetwork;
use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

use crate::commands::err::Error;
use crate::commands::util;
use crate::commands::RunnableOffline;

use tor_netdir;
use tor_netdoc::doc::netstatus;

#[derive(Debug, Clone)]
pub enum FindFilter {
    /// Address or Network
    Address(IpNetwork),
    /// Relay fingerprint
    Fingerprint(util::RelayFingerprint),
    /// Relay flags,
    Flags(netstatus::RouterFlags),
    /// Relay nickname
    Nickname(String),
    /// ORPort
    Port(u16),
    /// Relay version
    Version(String),
}

#[derive(StructOpt)]
pub struct FindCommand {
    #[structopt(short = "l", long = "oneline")]
    oneline: bool,
    /// The filters of the find command.
    filters: Vec<FindFilter>,
}

impl FindCommand {
    pub fn new(filters: &Vec<FindFilter>) -> Self {
        FindCommand {
            oneline: false,
            filters: filters.to_vec(),
        }
    }

    pub fn match_relay(&self, relay: &tor_netdir::Relay) -> bool {
        for filter in &self.filters {
            if !filter.match_relay(relay) {
                return false;
            }
        }
        true
    }

    pub fn filter<'a>(&self, netdir: &'a tor_netdir::NetDir) -> Vec<tor_netdir::Relay<'a>> {
        netdir.relays().filter(|r| self.match_relay(r)).collect()
    }

    pub fn count(&self, netdir: &tor_netdir::NetDir) -> usize {
        netdir.relays().filter(|r| self.match_relay(r)).count()
    }
}

impl FindFilter {
    pub fn match_relay(&self, relay: &tor_netdir::Relay) -> bool {
        match self {
            FindFilter::Address(a) => relay
                .rs()
                .orport_addrs()
                .find(|addr| a.contains(addr.ip()))
                .is_some(),
            FindFilter::Nickname(n) => relay.rs().nickname().contains(n),
            FindFilter::Fingerprint(fp) => fp.match_relay(relay),
            FindFilter::Flags(f) => relay.rs().flags().contains(*f),
            FindFilter::Port(p) => relay
                .rs()
                .orport_addrs()
                .find(|addr| addr.port() == *p)
                .is_some(),
            FindFilter::Version(v) => relay
                .rs()
                .version()
                .as_ref()
                .unwrap_or(&"".to_string())
                .contains(v),
        }
    }
}

impl fmt::Display for FindCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.filters)
    }
}

impl FromStr for FindFilter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(kv) = s.split_once(':') {
            let filter = match kv.0 {
                "a" | "addr" => Ok(FindFilter::Address(kv.1.parse().unwrap())),
                "fl" | "flag" => Ok(FindFilter::Flags(util::parse_routerflag(kv.1))),
                "f" | "fp" => Ok(FindFilter::Fingerprint(
                    kv.1.parse::<util::RelayFingerprint>()?,
                )),
                "n" | "nick" => Ok(FindFilter::Nickname(String::from(kv.1))),
                "p" | "port" => Ok(FindFilter::Port(kv.1.parse().unwrap())),
                "v" | "version" => Ok(FindFilter::Version(String::from(kv.1))),
                _ => Err(Error::UnrecognizedFilter(kv.0.to_string())),
            };
            return filter;
        }
        return Err(Error::InvalidFilter(s.to_string()));
    }
}

#[async_trait]
impl RunnableOffline for FindCommand {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()> {
        let relays = self.filter(netdir);

        if relays.is_empty() {
            println!("[-] No relays found");
            return Ok(());
        }
        util::describe_relays(&relays, self.oneline, 0);

        Ok(())
    }
}
