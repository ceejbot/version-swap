use std::fs::File;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::{Report, Result};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};

#[derive(Parser, Debug)]
#[clap(name = "version-swap", version)]
/// Tool intro goes here.
///
/// Longer tool help and discussion goes here. This can be multiple lines.
pub struct Args {
    /// Run the game directly, without SKSE.
    #[clap(long, short, global = true)]
    no_skse: bool,
    /// Be more chatty about what the tool is doing.
    #[clap(long, short, global = true)]
    verbose: bool,
    /// Be not chatty at all about what the tool is doing.
    #[clap(long, short, global = true)]
    quiet: bool,
    /// Optionally, the game directory to target. Defaults to the directory the tools is in.
    #[clap(long, short, global = true, default_value = ".")]
    gamedir: String,
    /// What to do.
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

/// Set up tee-ed logging to console and to a log file unless requested not to.
fn initialize_logging(args: &Args) -> Result<(), Report> {
    let level = if args.verbose {
        // Debug-level logging should be designed for players to read when they
        // are trying to debug problems.
        LevelFilter::Debug
    } else if args.quiet {
        // Error- and warn-level logging should be designed to inform players about truly important
        // problems or results.
        LevelFilter::Warn
    } else {
        // Info-level should be designed for players to read normally.
        LevelFilter::Info
    };

    // Shown in terminal.
    let config = simplelog::ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_module_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_enable_paris_formatting(true)
        .set_level_padding(simplelog::LevelPadding::Right)
        .build();
    // Logged to file.
    let writeconfig = simplelog::ConfigBuilder::new()
        .set_enable_paris_formatting(false)
        .set_module_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_time_format_rfc3339()
        .set_time_offset_to_local() // this is fallible, so...
        .unwrap_or_else(|xs| xs)
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(level, config, TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(
            LevelFilter::Debug,
            writeconfig,
            File::options()
                .create(true)
                .append(true)
                .open("version-swap.log")?,
        ),
    ])?;
    Ok(())
}

/// Collect relevant files in the given subdirectory of Versions, including
/// any plain files in a `data` subdirectory.
fn files_to_copy(dirname: &PathBuf) -> Result<Vec<PathBuf>> {
    let files: Vec<PathBuf> = std::fs::read_dir(dirname)?
        .filter_map(|xs| {
            let Ok(entry) = xs else {
                return None;
            };
            if entry.path().is_dir() {
                if entry.file_name().eq_ignore_ascii_case("data") {
                    // Note the bug.
                    Some(files_to_copy(&entry.path()).unwrap_or_default())
                } else {
                    None
                }
            } else {
                Some(vec![entry.path()])
            }
        })
        .flatten()
        .collect();

    Ok(files)
}

fn check_setup(args: &Args) -> Result<(), Report> {
    log::info!("Checking the setup in <b><green>{}</>", args.gamedir);

    // collect directories matching the pattern `skyrim_*` in "{args.gamedir}/Versions"
    let version_dir = PathBuf::from(format!("{}/Versions", args.gamedir));
    let versions: Vec<(PathBuf, String)> = std::fs::read_dir(version_dir)?
        .filter_map(|entry| {
            let Ok(dir) = entry else {
                return None;
            };
            if !dir.path().is_dir() {
                return None;
            }
            let binding = dir.file_name();
            let lossy_fname = binding.to_string_lossy();

            if lossy_fname.starts_with("skyrim-") {
                let vstr = lossy_fname.replace("skyrim-", "");
                Some((dir.path(), vstr))
            } else {
                None
            }
        })
        .collect();

    let required = vec![
        "SkyrimSE.exe",
        "skse64_loader.exe",
        "steam_api64.dll",
        "data/ccBGSSSE001-Fish.esm",
        "data/_ResourcePack.esl",
        // "skse64_steam_loader.dll",
    ];

    for (version_dir, version_string) in versions {
        let mut version_good = true;
        log::info!("Checking game version <blue><bold>{version_string}</>");
        let files = files_to_copy(&version_dir)?;
        for mandatory in required.as_slice() {
            if files.contains(&PathBuf::from(format!(
                "{}/{mandatory}",
                version_dir.display()
            ))) {
                log::debug!("    ✔️  <b>{mandatory}</> found");
            } else {
                version_good = false;
                log::warn!("    ⚠️  <b>{mandatory}</> missing");
            }
        }

        let skse_dll = format!("skse64_{}.dll", version_string.replace(".", "_"));
        let skse_expected = format!(
            "{}/skse64_{}.dll",
            args.gamedir,
            version_string.replace(".", "_")
        );
        if PathBuf::from(&skse_expected).exists() {
            log::debug!("    ✔️  <b>{skse_dll}</> found");
        } else {
            log::warn!(" ⚠️  missing <b>{skse_dll}</>");
            version_good = false;
        }

        if version_good {
            log::info!("SkyrimSE <blue><bold>{version_string}</> ready for swapping.\n");
        } else {
            log::warn!("Problems found with <blue><bold>skyrim-{version_string}</>!")
        }
    }

    Ok(())
}

fn run_version(version: &str, _args: &Args) -> Result<(), Report> {
    log::info!(
        "You ran `<blue>version-swap run <b>{}</>`. We would set up that version then run SKSE.",
        version
    );
    log::warn!("NOT IMPLEMENTED YET");
    Ok(())
}

fn main() -> Result<(), Report> {
    let args = Args::parse();
    initialize_logging(&args)?;

    match args.cmd {
        Command::Check => check_setup(&args)?,
        Command::Run { ref version } => run_version(version.as_str(), &args)?,
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
