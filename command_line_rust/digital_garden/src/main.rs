use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::{eyre, Context, Result};
use digital_garden::write;
use directories::UserDirs;
/// A CLI for the growing and curation of a digital garden
#[derive(Parser)]
#[command(name = "garden", version)]
struct Opt {
    #[arg(short = 'p', long, env)]
    garden_path: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// write something in your garden
    ///
    /// This command will open your $EDITOR, wait for you to
    /// write something, and then save the file to your garden.
    Write {
        /// Optionally set a title for what you are going to write about
        #[arg(short, long)]
        title: Option<String>,
    },
}

fn get_default_garden_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| eyre!("Could not find home directory"))?;
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::parse();
    let garden_path = match opt.garden_path {
        Some(path_buf) => Ok(path_buf),
        None => get_default_garden_dir().wrap_err("`garden_path` was not supplied"),
    }?;
    match opt.cmd {
        Command::Write { title } => write(garden_path, title),
    }
}
