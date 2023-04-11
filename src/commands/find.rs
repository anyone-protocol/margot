use anyhow::Result;
use async_trait::async_trait;
use ipnetwork::IpNetwork;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use structopt::StructOpt;

use crate::commands::err::Error;
use crate::commands::util;
use crate::commands::RunnableOffline;

use tor_netdoc::doc::netstatus;
use tor_netdoc::types::policy::PortPolicy;

#[derive(Debug, Clone)]
pub enum Filter {
    /// Address or Network
    Address(IpNetwork),
    /// Relay fingerprint
    Fingerprint(util::RelayFingerprint),
    /// Relay flags,
    Flags(netstatus::RelayFlags),
    /// Relay nickname
    Nickname(String),
    /// ORPort
    Port(u16),
    /// Relay version
    Version(String),
    /// Port policy
    PortPolicyFilter(PortPolicy),
}

#[derive(Debug, Clone)]
pub struct FindFilter {
    exclude: bool,
    filter: Filter,
}

impl FindFilter {
    pub fn new(n: bool, f: Filter) -> Self {
        Self {
            exclude: n,
            filter: f,
        }
    }

    pub fn match_relay(&self, relay: &tor_netdir::Relay) -> bool {
        let mut ret = match &self.filter {
            Filter::Address(a) => {
                relay.rs().orport_addrs().any(|addr| a.contains(addr.ip()))
            }
            Filter::Nickname(n) => relay.rs().nickname().contains(n),
            Filter::Fingerprint(fp) => fp.match_relay(relay),
            Filter::Flags(f) => relay.rs().flags().contains(*f),
            Filter::Port(p) => {
                relay.rs().orport_addrs().any(|addr| addr.port() == *p)
            }
            Filter::Version(v) => relay
                .rs()
                .version()
                .as_ref()
                .expect("version error")
                .to_string()
                .contains(v),
            Filter::PortPolicyFilter(pp) => &**relay.md().ipv4_policy() == pp,
            // ^ this is `&Arc<PortPolicy>`, 1st dereference `Arc`,
            // then `&`, then add `&` to match `&PortPolicy`
        };
        ret ^= self.exclude;
        ret
    }
}

#[derive(StructOpt)]
pub struct FindCommand {
    #[structopt(short = "l", long = "oneline")]
    oneline: bool,
    /// The filters of the find command.
    filters: Vec<FindFilter>,
}

impl FindCommand {
    pub fn new(filters: &[FindFilter]) -> Self {
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

    pub fn filter<'a>(
        &self,
        netdir: &'a tor_netdir::NetDir,
    ) -> Vec<tor_netdir::Relay<'a>> {
        netdir.relays().filter(|r| self.match_relay(r)).collect()
    }

    pub fn count(&self, netdir: &tor_netdir::NetDir) -> usize {
        netdir.relays().filter(|r| self.match_relay(r)).count()
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
        let exclude = s.contains("-:");
        if let Some(kv) = s.to_string().replace("-:", "").split_once(':') {
            let filter = match kv.0 {
                "a" | "addr" => Filter::Address(kv.1.parse().unwrap()),
                "fl" | "flag" => Filter::Flags(util::parse_routerflag(kv.1)),
                "f" | "fp" => Filter::Fingerprint(
                    kv.1.parse::<util::RelayFingerprint>()?,
                ),
                "n" | "nick" => Filter::Nickname(String::from(kv.1)),
                "p" | "port" => Filter::Port(kv.1.parse().unwrap()),
                "v" | "version" => Filter::Version(String::from(kv.1)),
                // Because a [PortPolicy] already contains `accept` or
                // `reject`, there is no need to use the `exclude` argument
                // (`-`). If used, it will still negate the policy, ie.
                // `pp-:"accept 25"` has the same effect as `pp:"reject 25".`
                "pp" | "portpolicyfilter" => {
                    Filter::PortPolicyFilter(kv.1.parse::<PortPolicy>()?)
                }
                // It takes the port policy from a file and expect the ports or
                // port ranges to be separated by spaces or commas.
                // Example: `pf:policy_accept.txt pf:policy_reject.txt`
                "pf" | "portpolicyfile" => Filter::PortPolicyFilter(
                    util::portpolicyfile2portpolicy(Path::new(kv.1))?,
                ),
                _ => return Err(Error::UnrecognizedFilter(kv.0.to_string())),
            };
            return Ok(FindFilter::new(exclude, filter));
        }
        Err(Error::InvalidFilter(s.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn port_policy_filter_from_str() {
        let filter_str = "pp:accept 20-23,43,53,79-81,88,110,143,194,220";
        let find_filter = FindFilter::from_str(filter_str).unwrap();
        let port_policy_filter = find_filter.filter;
        matches!(port_policy_filter, Filter::PortPolicyFilter(_));
    }

    #[test]
    #[should_panic(expected = "WrongPolicy(InvalidPolicy)")]
    // Ports are not in order, what happens when `-` is removed from `20-23`.
    fn port_policy_filter_from_str_invalid_policy() {
        let filter_str = "pp:accept 2023,43,53,79-81,88,110,143,194,220";
        let find_filter = FindFilter::from_str(filter_str).unwrap();
        let port_policy_filter = find_filter.filter;
        matches!(port_policy_filter, Filter::PortPolicyFilter(_));
    }
}
