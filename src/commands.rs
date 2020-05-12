use crate::dig::dig;
use crate::global::Result;

use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    ON,
    OFF,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(cmd: &str) -> std::result::Result<Command, Self::Err> {
        match cmd.to_lowercase().as_str() {
            "on" => Ok(Command::ON),
            "off" => Ok(Command::OFF),
            _ => Err(format!("unsupported command: {}", cmd)),
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::ON => "on".to_string(),
            Command::OFF => "off".to_string(),
        }
    }
}

impl Command {
    pub fn execute(&self, args: &[String]) -> Result<()> {
        self.validate_args_count(args)?;
        match self {
            Command::ON => on(args),
            Command::OFF => off(args),
        }?;
        Ok(())
    }

    fn validate_args_count(&self, args: &[String]) -> Result<()> {
        let min = match self {
            Command::ON => 2,
            Command::OFF => 2,
        };
        if args.len() < min {
            panic!("not enough arguments for '{}'", self.to_string());
        }
        Ok(())
    }
}

fn on(args: &[String]) -> Result<()> {
    out(&dig(&args[0])?, &args[1..]);
    Ok(())
}

#[allow(unused_variables)]
fn off(args: &[String]) -> Result<()> {
    unimplemented!()
}

fn out(addr: &str, hosts: &[String]) {
    println!("{} {}", addr, hosts.join(" "));
}

// fn clean(section: &str) -> String {
//     String::from(section.trim().trim_matches(|c| c == '\n' || c == '\r'))
// }
