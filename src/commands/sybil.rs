use anyhow::Result;
use async_trait::async_trait;
use multimap::MultiMap;
use std::fmt;
use structopt::StructOpt;

use crate::commands::util;
use crate::commands::RunnableOffline;

use tor_netdoc::types::policy::PortPolicy;

/// From <https://gitlab.torproject.org/legacy/trac/-/wikis/doc/ReducedExitPolicy>:
/// The Reduced Exit Policy is an alternative to the default exit policy.
/// It allows as many Internet services as possible while still blocking
/// the majority of TCP ports.
static REDUCED_EXIT_POLICY_DEFAULT: [u16; 102] = [
    20, 21, 22, 23, 43, 53, 79, 80, 81, 88, 110, 143, 194, 220, 389, 443, 464, 465, 531, 543, 544,
    554, 563, 587, 636, 706, 749, 853, 873, 902, 903, 904, 981, 989, 990, 991, 992, 993, 994, 995,
    1194, 1220, 1293, 1500, 1533, 1677, 1723, 1755, 1863, 2082, 2083, 2086, 2087, 2095, 2096, 2102,
    2103, 2104, 3128, 3389, 3690, 4321, 4643, 5050, 5190, 5222, 5223, 5228, 5900, 6660, 6661, 6662,
    6663, 6664, 6665, 6666, 6667, 6668, 6669, 6679, 6697, 8000, 8008, 8074, 8080, 8082, 8087, 8088,
    8232, 8233, 8332, 8333, 8443, 8888, 9418, 9999, 10000, 11371, 19294, 19638, 50002, 64738,
];
/*
static REDUCED_EXIT_POLICY_ABUSE: [u16; 76] = [
    20, 21, 43, 53, 79, 80, 81, 88, 110, 143, 220, 389, 443, 464, 531, 543, 544, 554, 636, 706,
    749, 873, 902, 903, 904, 981, 989, 990, 991, 992, 993, 995, 1194, 1220, 1293, 1500, 1533, 1677,
    1723, 1755, 1863, 2082, 2083, 2086, 2087, 2095, 2096, 2102, 2103, 2104, 3690, 4321, 4643, 5050,
    5190, 5222, 5223, 5228, 8008, 8074, 8082, 8087, 8088, 8232, 8233, 8332, 8333, 8443, 8888, 9418,
    10000, 11371, 19294, 19638, 50002, 64738,
];

static REDUCED_EXIT_POLICY_PORT_LIGHTWEIGHT: [u16; 26] = [
    20, 21, 43, 53, 80, 110, 143, 220, 443, 873, 989, 990, 991, 992, 993, 995, 1194, 1293, 3690,
    4321, 5222, 5223, 5228, 9418, 11371, 64738,
];

static REDUCED_EXIT_POLICY_PORT_BASIC: [u16; 3] = [53, 80, 443];

static REDUCED_EXIT_POLICY_PORT_IOT: [u16; 17] = [
    81, 83, 85, 86, 90, 1043, 1103, 1113, 1883, 4070, 5004, 5287, 5675, 6880, 8502, 8601, 8602,
];
*/

#[derive(Debug, Clone, StructOpt)]
pub struct TestExitPolicy {}

#[derive(StructOpt, Debug)]
pub enum SybilSubCommand {
    #[structopt(name = "exitpolicy", about = "Inspect Exit Policies")]
    Exitpolicy(TestExitPolicy),
}

#[derive(StructOpt)]
pub struct SybilCommand {
    #[structopt(subcommand)]
    pub subcommand: SybilSubCommand,
}

impl fmt::Display for SybilCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.subcommand)
    }
}

impl SybilCommand {
    /// If the relay's policy allows the same number of ports as the reduced
    /// policy, return true, otherwise return false.
    fn match_policy(&self, policy: &PortPolicy, reduced: &'static [u16]) -> bool {
        if reduced.iter().filter(|p| policy.allows_port(**p)).count() == reduced.len() {
            return true;
        }
        false
    }

    /// If the relay's policy does not allow any of the reduced policy ports,
    /// return true, otherwise return false.
    fn match_policy_empty(&self, policy: &PortPolicy, reduced: &'static [u16]) -> bool {
        if reduced.iter().filter(|p| policy.allows_port(**p)).count() == 0 {
            return true;
        }
        false
    }

    /// If the relay's policy allows the same number of ports as the reduced
    /// policy and the total number of allowed ports is greater than the
    /// reduced policy, return true, otherwise return false.
    fn match_policy_and_more(&self, policy: &PortPolicy, reduced: &'static [u16]) -> bool {
        // Yeah ugly but we don't have a way to know how many ports are
        // allowed for a PortPolicy object.
        let num_allowed_port = (0..u16::MAX).collect::<Vec<u16>>()  // 65535
            .iter()
            .filter(|p| policy.allows_port(**p))
            .count();

        if self.match_policy(policy, reduced) && num_allowed_port > reduced.len() {
            return true;
        }
        false
    }
}

#[async_trait]
impl RunnableOffline for SybilCommand {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()> {
        let mut policies = MultiMap::new();
        for relay in netdir.relays() {
            policies.insert(relay.ipv4_policy().clone(), relay);
        }

        for (policy, values) in policies.iter_all() {
            // Only unique policies we want.
            if values.len() != 1 {
                continue;
            }
            if self.match_policy_and_more(&policy, &REDUCED_EXIT_POLICY_DEFAULT) {
                println!("[+] Matching Reduced Exit Policy and More: '{}'", policy);
                util::describe_relays(&values, true, 4);
            } else if self.match_policy_empty(&policy, &REDUCED_EXIT_POLICY_DEFAULT) {
                println!("[+] Not matching Reduced Exit Policy: '{}'", policy);
                util::describe_relays(&values, true, 4);
            }
        }

        Ok(())
    }
}
