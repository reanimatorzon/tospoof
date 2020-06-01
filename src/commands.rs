//! Describes all subcommands and contains their logic

use crate::dig::dig;
use crate::global::{Result, HOSTS_FILE_LOCATION};

use std::fs;
use std::io::{stdin, Read};
use std::str::FromStr;

use anyhow::bail;
use atty::Stream;
use clap::ArgMatches;

/// Marks the start of dynamic block managed by the tool  
const HEAD_COMMENT: &str = "# tospoof: {{";
/// Marks the end of dynamic block managed by the tool
const FOOT_COMMENT: &str = "# tospoof: }}";

/// All the commands
#[derive(Debug)]
pub enum Command {
    /// Enables alias and prints content to stdout
    ON,
    /// Prints dynamic block managed by tool reading 'hosts' file
    PRINT,
    /// Updates 'hosts' file with content consumed from stdin.
    /// Warning: A dynamic block from 'hosts' file configured below will be
    /// erased
    UPDATE,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(cmd: &str) -> std::result::Result<Command, Self::Err> {
        match cmd.to_lowercase().as_str() {
            "on" => Ok(Command::ON),
            "print" => Ok(Command::PRINT),
            "update" => Ok(Command::UPDATE),
            _ => Err(format!("unsupported command: {}", cmd)),
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::ON => "on".to_string(),
            Command::PRINT => "print".to_string(),
            Command::UPDATE => "update".to_string(),
        }
    }
}

impl Command {
    pub fn execute(&self, args: &[String], matches: &ArgMatches) -> Result<()> {
        self.validate_args_count(args)?;

        let stdin_input = &consume_stdin_if_tty()?;
        match self {
            Command::ON => on(stdin_input, args),
            Command::PRINT => print(),
            Command::UPDATE => update(
                stdin_input,
                matches
                    .subcommand()
                    .1
                    .expect("runtime error: failed while parsing subcommand args")
                    .occurrences_of("verbose"),
            ),
        }?;

        Ok(())
    }

    fn validate_args_count(&self, args: &[String]) -> Result<()> {
        let min = match self {
            Command::ON => 2,
            Command::PRINT => 0,
            Command::UPDATE => 0,
        };

        if args.len() < min {
            bail!("not enough arguments for '{}'", self.to_string())
        } else {
            Ok(())
        }
    }
}

fn on(input: &str, args: &[String]) -> Result<()> {
    pretty_print(
        format!(
            "{}\n# tospoof: on: addr = {}\n{} {}",
            input,
            args[0],
            &dig(&args[0])?,
            &args[1..].join(" ")
        )
        .as_str(),
    );
    Ok(())
}

fn print() -> Result<()> {
    pretty_print(&split_hosts_content_blocks().1);
    Ok(())
}

fn update(input: &str, verbose: u64) -> Result<()> {
    let mut content = pretty_format(input);
    let blocks = split_hosts_content_blocks();

    if blocks.1.is_empty() {
        content = format!("{}\n{}\n\n{}\n\n{}\n", blocks.0, HEAD_COMMENT, content, FOOT_COMMENT);
    } else {
        content = format!(
            "{}{}\n\n{}\n\n{}{}",
            blocks.0, HEAD_COMMENT, content, FOOT_COMMENT, blocks.2
        );
    };

    fs::write(HOSTS_FILE_LOCATION, content)?;

    match verbose {
        1 => print()?,
        2..=9 => println!("{}", fs::read_to_string(HOSTS_FILE_LOCATION)?),
        _ => (),
    }

    Ok(())
}

fn split_hosts_content_blocks() -> (String, String, String) {
    let content = &fs::read_to_string(HOSTS_FILE_LOCATION)
        .unwrap_or_else(|_| panic!("no hosts file found in {}", HOSTS_FILE_LOCATION));

    let left = content.find(HEAD_COMMENT);
    let right = content.find(FOOT_COMMENT);

    if let Some(left) = left {
        assert!(right.is_some(),);
        if let Some(right) = right {
            (
                content[0..left].to_string(),
                content[left + HEAD_COMMENT.len()..right].to_string(),
                content[right + FOOT_COMMENT.len()..].to_string(),
            )
        } else {
            panic!(
                "hosts file is broken: '{}' has been found, {} hasn't been found",
                HEAD_COMMENT, FOOT_COMMENT
            );
        }
    } else {
        assert!(
            right.is_none(),
            "hosts file is broken: '{}' has been found, {} hasn't been found",
            FOOT_COMMENT,
            HEAD_COMMENT
        );
        (content.to_string(), String::new(), String::new())
    }
}

fn consume_stdin_if_tty() -> Result<String> {
    Ok(if !atty::is(Stream::Stdin) {
        let vec = &mut Vec::<u8>::new();
        stdin().read_to_end(vec)?;
        String::from_utf8(vec.clone())?
    } else {
        String::new()
    })
}

fn pretty_format(content: &str) -> String {
    let lines: Vec<String> = content
        .split("(\n|\r)+")
        .filter(|line| !line.is_empty())
        .map(str::trim)
        .map(String::from)
        .collect();
    lines.join("\n")
}

fn pretty_print(content: &str) {
    println!("{}", pretty_format(content));
}
