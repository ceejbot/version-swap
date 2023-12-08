use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::{Context, Report, Result};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};

#[derive(Parser, Debug)]
#[clap(name = "version-swap", version)]
/// Set up your Skyrim directory to run a specific game version by swapping
/// in the required DLLs and executables.
///
/// Longer tool help and discussion goes here. This can be multiple lines.
pub struct Args {
    /// Run the game directly, without SKSE.
    #[clap(long, short, global = true)]
    no_skse: bool,
    /// Print out more information as the tool runs.
    #[clap(long, short, global = true)]
    verbose: bool,
    /// Print out only very important information.
    #[clap(long, short, global = true)]
    quiet: bool,
    /// The game directory to target. Defaults to the directory the tool is in.
    #[clap(long, short, global = true, default_value = ".")]
    gamedir: String,
    /// What to do.
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Check that your version swap data is set up properly.
    Check,
    /// Set up the game directory to run a specific version and then launch the game.
    Run { version: String },
    /// Set up the game directory to run a specific version.
    Swap { version: String },
    /// Launch the game using either SKSE or the game executable directly.
    Launch,
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
        // Info-level logging should be designed for players to read normally.
        LevelFilter::Info
    };

    // Config for printing to a terminal.
    let config = simplelog::ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_module_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_enable_paris_formatting(true)
        .set_level_padding(simplelog::LevelPadding::Right)
        .build();
    // Config for logging to a file.
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
            } else if entry.path().ends_with(".DS_Store")
                || entry.path().extension() == Some(OsStr::new("acf"))
                || entry.path().extension() == Some(OsStr::new("bsa"))
            {
                None
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
    let version_dir = PathBuf::from(format!("{}/Versions", args.gamedir)).canonicalize()?;
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
        // "skse64_steam_loader.dll", <-- does 1.5.97 need this?
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

        let skse_dll = format!("skse64_{}.dll", version_string.replace('.', "_"));
        let skse_expected = format!(
            "{}/skse64_{}.dll",
            args.gamedir,
            version_string.replace('.', "_")
        );
        if PathBuf::from(&skse_expected).exists() {
            log::debug!("    ✔️  <b>{skse_dll}</> found");
        } else {
            log::warn!(" ⚠️  missing <b>{skse_dll}</>");
            version_good = false;
        }

        if version_good {
            log::info!("SkyrimSE <blue><bold>{version_string}</> ready to run.\n");
        } else {
            log::warn!("Problems found with <blue><bold>skyrim-{version_string}</>!")
        }
    }

    Ok(())
}

fn copy_file_with_check(origin: PathBuf, dest: PathBuf) -> Result<(), Report> {
    let mut hasher = crc32fast::Hasher::new();
    let buf = std::fs::read(&origin)?;
    hasher.update(buf.as_slice());
    let from_chksum = hasher.finalize();
    drop(buf);

    if let Some(destdir) = &dest.parent() {
        // I remain sad this is not named mkdirp().
        std::fs::create_dir_all(destdir)?;
    }

    std::fs::copy(&origin, &dest).context(format!(
        "copying {:?} to {}",
        origin.file_name().unwrap_or(origin.as_os_str()),
        dest.parent().unwrap_or(&dest).display()
    ))?;
    let destbuf = std::fs::read(&dest)?;
    let dest_chksum = crc32fast::hash(destbuf.as_slice());
    drop(destbuf);

    if from_chksum != dest_chksum {
        let msg = format!("Copy of `{}` failed! Source checksum: {from_chksum:#x}. Copy checksum: {dest_chksum:#x}", origin.display());
        Err(eyre::eyre!(msg))
    } else {
        Ok(())
    }
}

/// Swap to the requested game version.
fn swap_to(version: &str, args: &Args) -> Result<(), Report> {
    log::info!("Setting up the game directory for version <b>{version}</b>.");
    let version_dir = format!("{}/Versions/skyrim-{}/", args.gamedir, version);
    let files = files_to_copy(&PathBuf::from(&version_dir))?;
    for f in files {
        let basename = f.to_string_lossy().replace(&version_dir, "");
        let dest = format!("{}{}", args.gamedir, basename);
        log::debug!("    copying <blue>{basename}</>");
        copy_file_with_check(f, PathBuf::from(dest))?
    }
    log::info!("Ready to run!");
    Ok(())
}

/// Launch the game version that is staged in the game directory.
fn launch(args: &Args) -> Result<(), Report> {
    let exe = if args.no_skse {
        "./SkyrimSE.exe"
    } else {
        "./skse64_loader.exe"
    };

    log::debug!("Launching <b>{}/{exe}</>...", &args.gamedir);
    std::env::set_current_dir(&args.gamedir)
        .context("Setting the working directory to the gamedir")?;
    std::process::Command::new(exe)
        .spawn()
        .context(format!("launching {exe}"))?
        .wait()?;
    Ok(())
}

/// Swap to and then launch the requested game version.
fn run_version(version: &str, args: &Args) -> Result<(), Report> {
    swap_to(version, args)?;
    launch(args)?;
    Ok(())
}

/// Process command-line options and act on them.
fn main() -> Result<(), Report> {
    let args = Args::parse();
    initialize_logging(&args)?;

    match args.cmd {
        Command::Check => check_setup(&args)?,
        Command::Run { ref version } => run_version(version.as_str(), &args)?,
        Command::Swap { ref version } => swap_to(version, &args)?,
        Command::Launch => launch(&args)?,
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_tests() {
        // you should write tests
    }

    #[test]
    fn check_file_copy() {
        // untested
    }
}
