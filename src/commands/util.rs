use prettytable::format;
use prettytable::Table;
use std::fmt;
use std::str::FromStr;

use crate::commands::err::Error;

use tor_netdoc::doc::netstatus;

#[derive(Debug, Clone)]
pub enum RelayFingerprint {
    /// Rsa identity fingerprint
    Rsa(String),
    /// ed25519 identity fingerprint
    Ed(String),
}

impl FromStr for RelayFingerprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fingerprint = s.to_string().replace('$', "");
        let fp = match fingerprint.len() {
            40 => {
                if hex::decode(&fingerprint).is_err() {
                    return Err(Error::UndecodableFingerprint(s.to_string()));
                }
                RelayFingerprint::Rsa(fingerprint)
            }
            43 => RelayFingerprint::Ed(fingerprint),
            _ => return Err(Error::WrongFingerprintLength(s.to_string())),
        };
        Ok(fp)
    }
}

impl fmt::Display for RelayFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RelayFingerprint::Rsa(rsa) => rsa,
            RelayFingerprint::Ed(ed) => ed,
        };
        write!(f, "{}", s)
    }
}

impl RelayFingerprint {
    pub fn match_relay(&self, relay: &tor_netdir::Relay) -> bool {
        match self {
            RelayFingerprint::Rsa(rsa) => relay
                .rsa_id()
                .to_string()
                .contains(rsa.to_lowercase().as_str()),
            RelayFingerprint::Ed(ed) => *ed == relay.id().to_string(),
        }
    }
}

fn get_version(r: &tor_netdir::Relay) -> String {
    // Can't `unwrap_or` cause can't create `Version` data type
    r.rs()
        .version()
        .as_ref()
        .expect("version error")
        .to_string()
}

fn get_orports(r: &tor_netdir::Relay) -> String {
    r.rs()
        .orport_addrs()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}

fn describe_relay(r: &tor_netdir::Relay) {
    println!("[+] Nickname: {}", r.rs().nickname());
    println!(
        "  > Fingerprint: Rsa: {}, Ed: {}",
        r.rsa_id().to_string().to_uppercase(),
        r.md().ed25519_id().to_string()
    );
    println!("  > Flags: {:?}", r.rs().flags());
    println!("  > Weight: {:?}", r.rs().weight());
    println!("  > Version: {}", get_version(r));
    println!("  > ORPort(s): {}", get_orports(r));
    println!("  > IPv4 Policy: {}", r.ipv4_policy());
    println!("  > IPv6 Policy: {}", r.ipv6_policy());
    println!(
        "  > Family: {}",
        r.md()
            .family()
            .members()
            .map(|f| f.to_string().to_uppercase().replace('$', ""))
            .collect::<Vec<String>>()
            .join(" ")
    );
}

pub fn describe_relays(relays: &[tor_netdir::Relay], oneline: bool, indent: usize) {
    let tfmt = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[
                format::LinePosition::Top,
                format::LinePosition::Intern,
                format::LinePosition::Title,
                format::LinePosition::Bottom,
            ],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .indent(indent)
        .build();
    let mut table = Table::new();
    table.set_format(tfmt);
    table.set_titles(row!["Nickname", "Rsa", "Ed", "Version", "ORPorts"]);
    for r in relays {
        if oneline {
            table.add_row(row![
                r.rs().nickname(),
                r.rsa_id().to_string().to_uppercase(),
                r.md().ed25519_id().to_string(),
                get_version(r),
                get_orports(r),
            ]);
        } else {
            describe_relay(r)
        }
    }
    if oneline {
        table.printstd();
    }
}

/// Unfortunately, the arti RelayFlags string parsing follows the torspec case
/// sensitiveness and thus we have to remap them ourselves.
pub fn parse_routerflag(f: &str) -> netstatus::RelayFlags {
    match f.to_lowercase().as_str() {
        "authority" => netstatus::RelayFlags::AUTHORITY,
        "badexit" => netstatus::RelayFlags::BAD_EXIT,
        "exit" => netstatus::RelayFlags::EXIT,
        "fast" => netstatus::RelayFlags::FAST,
        "guard" => netstatus::RelayFlags::GUARD,
        "hsdir" => netstatus::RelayFlags::HSDIR,
        "noedconsensus" => netstatus::RelayFlags::NO_ED_CONSENSUS,
        "running" => netstatus::RelayFlags::RUNNING,
        "stable" => netstatus::RelayFlags::STABLE,
        "staledesc" => netstatus::RelayFlags::STALE_DESC,
        "v2dir" => netstatus::RelayFlags::V2DIR,
        "valid" => netstatus::RelayFlags::VALID,
        _ => netstatus::RelayFlags::empty(),
    }
}
