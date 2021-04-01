use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use structopt::StructOpt;

use crate::commands::find;
use crate::commands::RunnableOffline;

static GITLAB_BUG_URL: &str =
    "https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues";

static REJECT_TOKENS: (&str, &str) = ("AuthDirReject", "!reject");
static BADEXIT_TOKENS: (&str, &str) = ("AuthBadExit", "!badexit");

#[derive(Debug, Clone, StructOpt)]
pub struct BadCommand {
    ticket: u32,
    /// The filters of this command.
    filters: Vec<find::FindFilter>,
}

#[derive(StructOpt, Debug)]
pub enum ConfigSubCommand {
    #[structopt(name = "badexit", about = "Generate bad exit rule(s)")]
    BadExit(BadCommand),
    #[structopt(name = "reject", about = "Generate reject rule(s)")]
    Reject(BadCommand),
}

#[derive(StructOpt)]
pub struct ConfigCommand {
    #[structopt(subcommand)]
    pub subcommand: ConfigSubCommand,
}

fn fmt_addr_rule(prefix: &str, relay: &tor_netdir::Relay<'_>) -> String {
    let rules: Vec<_> = relay
        .rs()
        .orport_addrs()
        .map(|a| format!("{} {}", prefix, a.ip()))
        .collect();
    rules.join("\n")
}

fn fmt_fp_rule(prefix: &str, relay: &tor_netdir::Relay<'_>) -> String {
    format!(
        "{} {}",
        prefix,
        relay.rsa_id().to_string().replace("$", "").to_uppercase()
    )
}

impl fmt::Display for ConfigCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.subcommand)
    }
}

impl BadCommand {
    fn comment(&self) -> String {
        format!("# Ticket: {}/{}", GITLAB_BUG_URL, self.ticket)
    }

    fn print_header(&self, fname: &str) {
        println!("[+] Rules for {}:", fname);
        println!("");
        println!("-----");
        println!("{}", self.comment());
    }

    fn print_footer(&self) {
        println!("-----");
        println!("");
    }

    fn print_rules<F>(
        &self,
        prefix: &str,
        fname: &str,
        fmt_fn: F,
        relays: &Vec<tor_netdir::Relay<'_>>,
    ) where
        F: Fn(&str, &tor_netdir::Relay<'_>) -> String,
    {
        self.print_header(fname);
        for relay in relays {
            println!("{}", fmt_fn(prefix, relay));
        }
        self.print_footer();
    }

    fn generate(&self, netdir: &tor_netdir::NetDir, tokens: &'static (&str, &str)) -> Result<()> {
        let relays = find::FindCommand::new(&self.filters).filter(netdir);

        self.print_rules(tokens.0, "bad.conf", fmt_addr_rule, &relays);
        self.print_rules(tokens.1, "approved-routers.conf", fmt_fp_rule, &relays);

        println!("[+] Found {} relays: {:?}", relays.len(), self.filters);
        Ok(())
    }
}

#[async_trait]
impl RunnableOffline for ConfigCommand {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()> {
        match &self.subcommand {
            ConfigSubCommand::BadExit(r) => r.generate(netdir, &BADEXIT_TOKENS),
            ConfigSubCommand::Reject(r) => r.generate(netdir, &REJECT_TOKENS),
        }
    }
}
