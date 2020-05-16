mod aliases;
mod args;
mod commands;
mod dig;
mod global;

use aliases::*;
use args::*;
use global::*;

fn get_configs_dir() -> Result<String> {
    // if let Some(path) = std::env::current_exe()?.parent() {
    //     Ok(path.as_os_str().to_str().unwrap().to_string())
    // } else {
    //     Ok("".to_string())
    // }
    Ok("/home/reanimator/Projects/Rust/tospoof".to_string())
}

fn main() -> Result<()> {
    let dir = get_configs_dir()?;
    // TODO parse_app_config(dir);
    let aliases = parse_aliases_config(&dir)?;

    let (cmd, args, matches) = parse_args(&aliases)?;
    cmd.execute(&args, &matches)?;

    Ok(())
}
