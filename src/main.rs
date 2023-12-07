use clap::{Parser, Subcommand};
use eyre::{Report, Result};
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[clap(name = "version-swap", version)]
/// Tool intro goes here.
///
/// Longer tool help and discussion goes here. This can be multiple lines.
pub struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Check that your game directory is set up properly.
    Check,
    /// Set up the game directory to run this version and then launch SKSE.
    Run { version: String },
}

fn main() -> Result<(), Report> {
    let args = Args::parse();
    match args.cmd {
        Command::Check => println!(
            "You ran `{}`. We would check that your game directory is set up properly.",
            "version-swap check".blue()
        ),
        Command::Run { version } => {
            println!(
                "You ran `{} {}`. We would set up that version then run SKSE.",
                "version-swap run".blue(),
                version.blue().bold()
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_tests() {
        // you should write tests
    }
}
