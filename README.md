# skyrim version swap

`version-swap` is a command-line tool with a boring name that you use with your mod manager to swap versions of the Skryim executable when you launch the game. This allows you to have one game folder and run several different game versions. For example, you might want to run Skyrim version 1.5.97 so you can build grass caches with the [.Net Script Framework](https://www.nexusmods.com/skyrimspecialedition/mods/21294), then switch to version 1.6.1130 so you can have the latest animation native mods while you play.

The text for the Nexus mod description is in the [nexus_docs.md file](./nexus_docs.md), in Markdown format.

## Building

This is a Rust project. To build, start by installing the Rust toolchain with [rustup](https://rustup.rs) then run `cargo build`. There are some development conveniences available in the [justfile](https://just.systems). Run `cargo install just` to take advantage of them. While Skyrim only runs on Windows, the tool can be built and run on any OS Rust supports. I've been testing by running it against a game installation mounted through samba on an ARM Mac laptop.

The source is in a single file and quite short, but you can generate and read the usual programmer documentation by running `cargo doc --open`.

I encourage you to think carefully about logging when working on this tool. This project uses the standard Rust [log](https://lib.rs/crates/log) macros to easily support verbosity levels in output. Error- and warn-level logging should be designed to inform players about truly important problems or results, such as misconfiguration of their versions data. Info-level logging must be designed for players to read normally, and should confirm the results of their actions. Players can use the `--verbose` flag to show debug-level logging to investigate problems, so debug-level logs should be designed to help them solve problems.

Trace level is for programmers working on this tool. To enable it, you'll need to make a code level change and recompile. Feel free to leave trace-level log lines checked in.

## Full usage

Run `version-swap help` to get help on the full list of commands available.

```text
Set up your Skyrim directory to run a specific game version by swapping in the required
DLLs and executables.

Usage: version-swap [OPTIONS] <COMMAND>

Commands:
  check   Check that your version swap data is set up properly
  run     Set up the game directory to run a specific version and then launch the game
  swap    Set up the game directory to run a specific version
  launch  Launch the game as-is using either SKSE or the game executable directly
  help    Print this message or the help of the given subcommand(s)

Options:
  -n, --no-skse
          Run the game directly, without SKSE

  -v, --verbose
          Print out more information as the tool runs

  -q, --quiet
          Print out only very important information

  -g, --gamedir <GAMEDIR>
          The game directory to target. Defaults to the directory the tool is in
          [default: .]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## License

[The Parity Public License.](https://paritylicense.com) This license requires people who build with this software to share their work with the community, too. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding. You are welcome to do anything you like with this software, so long as you share what you do.
