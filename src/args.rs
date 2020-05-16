use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings};

use crate::aliases::AliasDictionary;
use crate::commands::Command;
use crate::global::{parse_function_args, remove_whitespace, Result};
use std::str::FromStr;

pub fn parse_args(aliases: &AliasDictionary) -> Result<(Command, Vec<String>, ArgMatches)> {
    let app = App::new("tospoof")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColorAuto)
        //.setting(AppSettings::NoBinaryName)
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::SubcommandPrecedenceOverArg)
        .version("0.9.0-alpha.2")
        .author("Vasily Bolgar <vasily.bolgar@gmail.com>")
        .about("Utility assists with 'hosts' file manipulations")
        .subcommand(
            App::new(Command::ON.to_string())
                .about(concat!(
                    "Enables alias presets (or raw arguments) provided as list of arguments.\n",
                    "First parameter is considered as address and processed with dig / nslookup"
                ))
                .arg(Arg::with_name("arguments").required(true).multiple(true)),
        )
        .subcommand(
            App::new(Command::PRINT.to_string()).about("Prints dynamic block from hosts file"),
        )
        .subcommand(
            App::new(Command::UPDATE.to_string())
                .about("Update hosts file with data consumed from stdin/tty pipe")
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .setting(ArgSettings::MultipleOccurrences)
                        .about("Prints updated data"),
                ),
        );

    // TODO later: generate::<Bash, _>(&mut app.clone(), "tospoof", &mut io::stdout());

    let matches = app.get_matches();
    let subcommand = matches.subcommand();

    let cmd = Command::from_str(subcommand.0)?;
    let args = process_args(aliases, &subcommand.1.unwrap());
    Ok((cmd, args, matches.to_owned()))
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

        for item in &func.list {
            let mut processed = item.clone();
            for (idx, name) in func.args.iter().enumerate() {
                let value = args.get(idx).unwrap();
                processed = processed.replace(&format!("<{}>", name), value);
            }
            ret.append(&mut expand_arg(aliases, &processed));
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
