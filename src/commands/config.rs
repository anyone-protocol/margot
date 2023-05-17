use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use structopt::StructOpt;

use crate::commands::err::Error;
use crate::commands::find;
use crate::commands::RunnableOffline;

static GITLAB_BUG_URL: &str =
    "https://gitlab.torproject.org/tpo/network-health/bad-relay-reports/-/issues";

static REJECT_TOKENS: (&str, &str) = ("AuthDirReject", "!reject");
static BADEXIT_TOKENS: (&str, &str) = ("", "!badexit");
static MIDDLEONLY_TOKENS: (&str, &str) = ("", "!middleonly");

static APPROVED_ROUTERS_PATH: &str =
    "approved-routers.d/approved-routers.conf";
static BAD_PATH: &str = "torrc.d/bad.conf";

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
    #[structopt(name = "middleonly", about = "Generate middleonly rule(s)")]
    MiddleOnly(BadCommand),
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
    [rules.join("\n"), "\n".to_string()].concat()
}

fn fmt_fp_rule(prefix: &str, relay: &tor_netdir::Relay<'_>) -> String {
    format!(
        "{} {}\n",
        prefix,
        relay.rsa_id().to_string().replace('$', "").to_uppercase()
    )
}

impl fmt::Display for ConfigCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.subcommand)
    }
}

impl BadCommand {
    fn comment(&self) -> String {
        // Add new line at the start of the comment to visually separated from
        // previous rules
        format!("\n# Ticket: {}/{}\n", GITLAB_BUG_URL, self.ticket)
    }

    fn print_header(&self, fname: &str, file: &mut File) -> Result<()> {
        println!("[+] Rules for {}:", fname);
        println!();
        println!("-----");
        print!("{}", self.comment());
        file.write_all(self.comment().as_bytes())?;
        Ok(())
    }

    fn print_footer(&self) {
        println!("-----");
        println!();
    }

    fn open_file(&self, fname: &str) -> Result<File, Error> {
        let path = Path::new(fname);
        let parent = path
            .parent()
            .ok_or_else(|| Error::WrongParent(fname.to_string()))?;
        // Create the directory if does not exists
        fs::create_dir_all(parent)?;
        // Create the file if it does not exists
        let file = OpenOptions::new().create(true).append(true).open(fname)?;
        Ok(file)
    }

    fn print_rules<F>(
        &self,
        prefix: &str,
        fname: &str,
        fmt_fn: F,
        relays: &[tor_netdir::Relay<'_>],
    ) -> Result<(), anyhow::Error>
    where
        F: Fn(&str, &tor_netdir::Relay<'_>) -> String,
    {
        let mut file = self.open_file(fname)?;
        self.print_header(fname, &mut file)?;
        for relay in relays {
            let rule = fmt_fn(prefix, relay);
            print!("{}", rule);
            file.write_all(rule.as_bytes())?;
        }
        self.print_footer();
        Ok(())
    }

    fn generate(
        &self,
        netdir: &tor_netdir::NetDir,
        tokens: &'static (&str, &str),
    ) -> Result<()> {
        let relays = find::FindCommand::new(&self.filters).filter(netdir);

        // Do not create bad.conf config when there is not token for it, as it
        // is the case for `middleonly` argument.
        if !tokens.0.is_empty() {
            self.print_rules(tokens.0, BAD_PATH, fmt_addr_rule, &relays)?;
        }
        self.print_rules(
            tokens.1,
            APPROVED_ROUTERS_PATH,
            fmt_fp_rule,
            &relays,
        )?;

        println!("[+] Found {} relays: {:?}", relays.len(), self.filters);
        Ok(())
    }
}

#[async_trait]
impl RunnableOffline for ConfigCommand {
    fn run(&self, netdir: &tor_netdir::NetDir) -> Result<()> {
        match &self.subcommand {
            ConfigSubCommand::BadExit(r) => {
                r.generate(netdir, &BADEXIT_TOKENS)
            }
            ConfigSubCommand::Reject(r) => r.generate(netdir, &REJECT_TOKENS),
            ConfigSubCommand::MiddleOnly(r) => {
                r.generate(netdir, &MIDDLEONLY_TOKENS)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn open_file() {
        let binding = temp_dir().join("approved-routers.conf");
        let fname = binding.to_str().unwrap();
        let bad_command = BadCommand {
            ticket: 1,
            filters: Vec::from([find::FindFilter::new(
                false,
                find::Filter::Nickname("moria1".to_string()),
            )]),
        };
        let file = bad_command.open_file(fname);
        assert!(file.is_ok());

        let fname = "/root";
        let file = bad_command.open_file(fname);
        assert!(file.is_err());
    }
    #[test]

    fn print_header_ok() {
        let binding = temp_dir().join("approved-routers.conf");
        let fname = binding.to_str().unwrap();
        let bad_command = BadCommand {
            ticket: 1,
            filters: Vec::from([find::FindFilter::new(
                false,
                find::Filter::Nickname("moria1".to_string()),
            )]),
        };
        let mut file = bad_command.open_file(fname).unwrap();

        let result = bad_command.print_header(fname, &mut file);
        assert!(result.is_ok());
    }
}
