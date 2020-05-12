use clap::{App, AppSettings, Arg, ArgMatches};

use crate::aliases::AliasDictionary;
use crate::commands::Command;
use crate::global::{parse_function_args, remove_whitespace, Result};
use std::str::FromStr;

pub fn parse_args(aliases: &AliasDictionary) -> Result<(Command, Vec<String>)> {
    let matches = App::new("tospoof")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.9.0-alpha.1")
        .author("Vasily Bolgar <vasily.bolgar@gmail.com>")
        .about("ABOUT")
        .subcommand(
            App::new(Command::ON.to_string())
                .about("_ABOUT_ON_")
                .arg(Arg::with_name("arguments").required(true).multiple(true)),
        )
        .subcommand(
            App::new(Command::OFF.to_string())
                .about("_ABOUT_OFF_")
                .arg(Arg::with_name("arguments").required(true).multiple(true)),
        )
        .get_matches();

    let subcommand = matches.subcommand();

    let cmd = Command::from_str(subcommand.0)?;
    let args = process_args(aliases, &subcommand.1.unwrap());
    Ok((cmd, args))
}

fn process_args(aliases: &AliasDictionary, args: &ArgMatches) -> Vec<String> {
    if let Some(args) = args.values_of("arguments") {
        let mut ret = vec![];
        for arg in args {
            ret.append(&mut expand_arg(aliases, arg));
        }
        ret
    } else {
        vec![]
    }
}

fn expand_arg(aliases: &AliasDictionary, token: &str) -> Vec<String> {
    let (sign, args) = parse_function_args(token);
    let func = aliases.get(&sign);

    if let Some(func) = func {
        assert_args_count(sign, func.args.len(), args.len());

        let mut ret = vec![];
        for (idx, name) in func.args.iter().enumerate() {
            let value = args.get(idx).unwrap();
            for item in &func.list {
                let processed = item.replace(&format!("<{}>", name), value);
                ret.append(&mut expand_arg(aliases, &processed));
            }
        }

        ret
    } else {
        vec![remove_whitespace(token)]
    }
}

fn assert_args_count(sign: String, expected: usize, actual: usize) {
    assert_eq!(
        expected, actual,
        "application error: alias '{}' has {} argument(s), actual argument(s) count = {}",
        sign, expected, actual
    );
}
