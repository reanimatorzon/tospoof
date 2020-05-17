//! # To Spoof
//!
//! `tospoof` is a binary assisting with 'hosts' file manipulations.

use aliases::*;
use args::*;
use global::*;

#[cfg(debug_assertions)]
use dirs::home_dir;

#[cfg(not(debug_assertions))]
use anyhow::bail;

mod aliases;
mod args;
mod commands;
mod dig;
mod global;

fn main() -> Result<()> {
    let dir = get_configs_dir()?;
    // TODO parse_app_config(dir);
    let aliases = parse_aliases_config(&dir)?;

    let (cmd, args, matches) = parse_args(&aliases)?;
    cmd.execute(&args, &matches)?;

    Ok(())
}

#[cfg(not(debug_assertions))]
fn get_configs_dir() -> Result<String> {
    if let Some(path) = std::env::current_exe()?.parent() {
        Ok(String::from(path.to_str().unwrap()))
    } else {
        bail!("config directory not found")
    }
}

#[cfg(debug_assertions)]
fn get_configs_dir() -> Result<String> {
    Ok(format!(
        "{}{}",
        home_dir().unwrap().to_str().unwrap(),
        "/Projects/Rust/tospoof".to_string()
    ))
}
