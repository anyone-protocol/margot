use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path;
use structopt::StructOpt;

use crate::commands::err::Error;
use crate::commands::find;
use crate::commands::util;
use crate::commands::RunnableOffline;

static GITHUB_BUG_URL: &str =
    "https://github.com/anyone-protocol/ator-protocol/issues";

static REJECT_TOKENS: (&str, &str) = ("", "!reject");
static REJECTBAD_TOKENS: (&str, &str) = ("AuthDirReject", "!reject");
static BADEXIT_TOKENS: (&str, &str) = ("", "!badexit");
static MIDDLEONLY_TOKENS: (&str, &str) = ("", "!middleonly");

static APPROVED_ROUTERS_PATH: &str =
    "data/approved-routers.d/approved-routers.conf";
static BAD_PATH: &str = "data/anonrc.d/bad.conf";

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
    #[structopt(
        name = "rejectbad",
        about = "Generate reject rule(s), writing to bad.conf too"
    )]
    RejectBad(BadCommand),
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
        // This returns all the ORPort IPs (v4 and v6)
        .orport_addrs()
        .map(|a| {
            if a.is_ipv4() {
                format!("{} {}", prefix, a.ip())
            } else {
                // Enclose v6 IPs in brackets
                format!("{} [{}]", prefix, a.ip())
            }
        })
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
        format!("\n# Ticket: {}/{}\n", GITHUB_BUG_URL, self.ticket)
    }

    fn print_header(&self, fname: &str, file: &mut File) -> Result<()> {
        println!("[+] Rules for {}:", fname);
        println!();
        println!("-----");
        print!("{}", self.comment());
        file.write_all(self.comment().as_bytes())?;
        Ok(())
    }

    fn print_fp_comment(&self, file: &mut File) -> Result<()> {
        let fp_text = "# Fingerprints:\n";
        print!("{}", fp_text);
        file.write_all(fp_text.as_bytes())?;
        Ok(())
    }

    fn print_footer(&self) {
        println!("-----");
        println!();
    }

    fn open_file(&self, fname: &str) -> Result<File, Error> {
        let path = path::Path::new(fname);
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
        file: &mut File,
        fmt_fn: F,
        relays: &[tor_netdir::Relay<'_>],
    ) -> Result<(), anyhow::Error>
    where
        F: Fn(&str, &tor_netdir::Relay<'_>) -> String,
    {
        for relay in relays {
            let rule = fmt_fn(prefix, relay);
            print!("{}", rule);
            file.write_all(rule.as_bytes())?;
        }
        Ok(())
    }

    fn print_missing_fps(
        &self,
        prefix: &str,
        file: &mut File,
        fps: Vec<String>,
    ) -> Result<(), anyhow::Error> {
        for fp in fps {
            let rule = format!("{} {}\n", prefix, fp);
            print!("{}", rule);
            file.write_all(rule.as_bytes())?;
        }
        Ok(())
    }

    fn generate(
        &self,
        netdir: &tor_netdir::NetDir,
        tokens: &'static (&str, &str),
    ) -> Result<()> {
        let relays = find::FindCommand::new(&self.filters).filter(netdir);

        // Do not create bad.conf config when there is not token for it, as it
        // is the case for `middleonly` argument or when no relays were found.
        if !tokens.0.is_empty() && !relays.is_empty() {
            // When token.0 is present, write also into BAD_PATH
            let fname = BAD_PATH;
            let mut file = self.open_file(fname)?;
            self.print_header(fname, &mut file)?;

            // Print also the fingeprints in a comment
            self.print_fp_comment(&mut file)?;
            self.print_rules(
                "#              ",
                &mut file,
                fmt_fp_rule,
                &relays,
            )?;
            // Print the addresses when there's the `AuthDirReject` token
            self.print_rules(tokens.0, &mut file, fmt_addr_rule, &relays)?;
            self.print_footer();
        }
        // If the filter is `FpsFileFilter`, print all the parsed fingerprints
        // from the file even if the weren't found in the consensus.
        let found_fps = util::relays2fps(&relays);
        let missing_fps = self.missing_fps(found_fps);
        // Write into APPROVED_ROUTERS_PATH if there're found relays or
        // missing ones.
        if !relays.is_empty() || !missing_fps.is_empty() {
            let fname = APPROVED_ROUTERS_PATH;
            let mut file = self.open_file(fname)?;
            self.print_header(fname, &mut file)?;

            self.print_rules(tokens.1, &mut file, fmt_fp_rule, &relays)?;

            if !missing_fps.is_empty() {
                self.print_missing_fps(tokens.1, &mut file, missing_fps)?;
            }
            self.print_footer();
        }
        println!("[+] Found {} relays: {:?}", relays.len(), self.filters);
        Ok(())
    }

    /// Return the relays' fingerprints in the `FpsFileFiler` filter that were
    /// not found.
    /// Since `self` doesn't have a vector with the parsed fingeprints from
    /// the file, it is needed to parse the filters to obtain them.
    ///
    fn missing_fps(&self, found_fps: Vec<String>) -> Vec<String> {
        // NOTE: The following iterations and loops could be less verbose
        // Create a vector with the missing fps from the previous vector.
        let mut fps = Vec::new();
        for find_filter in &self.filters {
            if let find::Filter::FpsFileFilter(relay_fingerprints) =
                &find_filter.filter
            {
                for relay_fingerprint in relay_fingerprints {
                    if !found_fps.contains(&relay_fingerprint.to_string()) {
                        fps.push(relay_fingerprint.to_string())
                    }
                }
            }
        }
        fps
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
            ConfigSubCommand::RejectBad(r) => {
                r.generate(netdir, &REJECTBAD_TOKENS)
            }
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

    #[test]
    fn missing_relays_ok() {
        let path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("testdata/fps.txt");
        let bad_command = BadCommand {
            ticket: 1,
            filters: Vec::from([find::FindFilter::new(
                false,
                find::Filter::FpsFileFilter(util::fpfile2fps(&path).unwrap()),
            )]),
        };
        let found_fps =
            vec!["0011BD2485AD45D984EC4159C88FC066E5E3300E".to_string()];
        let missing_fps = bad_command.missing_fps(found_fps);
        let expected_missing_fps =
            vec!["0123456789ABCDEF0123456789ABCDEF01234567".to_string()];
        assert!(expected_missing_fps == missing_fps);
    }
}
