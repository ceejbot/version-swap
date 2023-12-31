# version-swap

`version-swap` is a command-line tool with a boring name that you use with your mod manager to swap versions of the Skryim executable when you launch the game. This allows you to have one game folder and run several different game versions. For example, you might want to run Skyrim version 1.5.97 so you can build grass caches with the [.Net Script Framework](https://www.nexusmods.com/skyrimspecialedition/mods/21294), then switch to version 1.6.1130 so you can have the latest animation native mods while you play.

These instructions help you get going with the tool so you can run any version of Skyrim you want to, without stress.

## Installation

First, make sure that [BEES](https://www.nexusmods.com/skyrimspecialedition/mods/106441) is installed and enabled for all of the versions of the game you're swapping among. If you don't run this, any plugins with the extended formid ranges will crash your game. It works for everything; you should run it.

Now that you have your game full of bees, let's set up version-swap as a tool in your preferred mod manager.

### Mod Organizer 2

If you are using [Kezyma's Root Builder](https://www.nexusmods.com/skyrimspecialedition/mods/31720) plugin for MO2, you can make a root mod for this tool and avoid having to put files in your game folder. Steps:

1. Make an empty mod in MO2, and name it something useful, like `version-swap`.
2. Make a folder named `root` inside this new mod.
2. Unpack the mod archive into the `root` directory.
4. As you follow the setup steps, put all the files into `root/Versions` and the directories in there.

If you're not using Root Builder, do everything inside the game directory instead of "root". (But consider using Root Builder.)

Next, add an executable for `version-swap.exe`. Make sure you're running the executable from the game directory, not the mod directory! Give it the arguments `wait --check`.

Mine looks like this:

![screenshot of mo2 showing the executable set up](./assets/mo2_setup.png)

Run it to verify that it's working. It will very likely complain that your setup isn't right yet, because we haven't yet copied all the DLLs and executables into place. That's fine for now. If the swapper is running and reporting correctly on the state of your game directory, you can copy that executable setup and modify it to launch the game. Change the arguments to `run 1.5.97` or whichever version you want to swap to and run. Do this as many times as you have versions to swap to.

### Vortex

TKTK

## Set up

We're going to set up version-swap with your mod manager, then make sure it has the data it needs to swap between versions. If you used [Skyrim Lite Loader](https://www.nexusmods.com/skyrimspecialedition/mods/58271) before, this process will be familiar to you. We need to stash more data than we used to, however.

### Before we start

These instructions assume you're modifying the game directory. If your mod manager allows you to set up "root mods", or mods that install files into the root of the game directory, you might want use a root mod instead of modifying your game directory.

### Step 1: the swap tool

Copy the `version-swap.exe` file into your game directory. Add it as a tool for your mod organizer. You'll want to make three different shortcuts for three different ways of running the tool:

- `version-swap run 1.6.1130`: Set up your game directory for version 1.6.1130 (the latest version at the time of this writing) and run it.
- `version-swap run 1.5.97`: Set up your game directory for 1.5.97 (or "SE") and run that.
- `version-swap check`: Check to see if your directory set up has all the files required to make swapping work.

We suggest you use the exact version numbers of the game instead of the "AE" and "SE" nicknames just in case you ever want to support more than one version of AE. If there's a new game version released, you can make a new directory for that version and set up a shortcut for it in your mod organizer. The tool does not dictate this to you, however. You can name versions anything you like. If you feel very silly, you can swap between "skyrim-stormcloaks" and "skyrim-imperials".

### Step 2: SKSE dlls

In your game root folder, put SKSE's dll for each of the game versions you're switching between. At the time of writing, those are `skse64_1_5_97.dll` and `skse64_1_6_1130.dll`. You can safely leave all SKSE dlls in the game folder no matter which version you want to run-- the loader executable is what determines which version of SKSE is used.

There's one more file you'll need for version 1.5.97: `skse64_steam_loader.dll`. There is no equivalent to this DLL for 1.6 builds of SKSE.

### Step 3: game assets

Set up folders with the executables the swapper will be moving into place. This tool comes with a folder structure set up to help you do this. We cannot, however, distribute any game assets, so you'll need to copy all game files from your own installation.

Make a folder named `Versions` inside your game directory. Inside this folder, make a folder named `skyrim-1.5.97`. Inside this folder, copy the following files:

- `SkyrimSE.exe` for 1.5.97
- the file `steam_api64.dll` from 1.5.97
- `skse64_loader.exe` from the 1.5.97 of SKSE

Then make a second directory, named `skyrim-1.6.1130`. Put the same files inside but for the newer versions of the game.

__Pro tip:__ If a new version of Skyrim is released, you can support it with this tool by making a new directory named `skyrim-*number*` and putting that version's builds of the same files into it.

### Step 4: review the folder

When you're done, your game folder (or root mod) should include these files:

```text
.
├── version-swap.exe
├── skse64_1_5_97.dll
├── skse64_1_6_1130.dll
├── skse64_steam_loader.dll
└── Versions
   ├── skyrim-1.5.97
   │  ├── skse64_loader.exe
   │  ├── SkyrimSE.exe
   │  └── steam_api64.dll
   └── skyrim-1.6.1130
      ├── skse64_loader.exe
      ├── SkyrimSE.exe
      └── steam_api64.dll
```

__Pro tip:__ These files are the ones you *must* swap in to run different versions of the game. This tool will copy in *any* files you have in a version folder, so if your setup has additional DLLs you need to swap, version-swap can do it for you. It will not copy entire subdirectories! Use a mod manager to handle things with nested folders.

### Step 7: see what the tool says

Check your setup. Run `version-swap check` to verify that you've got everything in place. If the tool reports problems, follow the instructions it gives to fix things. If everything looks good you're now ready to swap and launch!

## More things you can do

- `version-swap check`: Check that your version swap data is set up properly.
- `version-swap run X.Y.Z`: Set up the game directory to run a specific version and then launch the game.
- `version-swap swap X.Y.Z`: Set up the game directory to run a specific version, but don't launch it.
- `version-swap launch`: Launch the game at whatever version it is right now using either SKSE or the game executable directly. Pass the `--no-skse` flag to skip SKSE, like this: `version-swap launch --no-skse`.
- `version-swap help`: Show all tool help, which includes options for making the tool quieter or noisier.

## FAQ

__Q:__ Can I use this to swap between 1.6.640 and 1.5.97?
__A:__ Yes. The tool will still assume you need the fishing esl and the resource pack esl, though.

__Q:__ Can I have more than two versions set up for swapping?
__A:__ Yes, as many as you like. In the short term, until 1.6.1130 is fully supported by mods, I personally will have 1.5.97, 1.6.640, and 1.6.1130 set up.

## Credits and permissions

This tool was inspired by [Skyrim Lite Loader](https://www.nexusmods.com/skyrimspecialedition/mods/58271), though it shares no code with it. It was written at the request of some people on the subreddit Discord who knew I'd be a sucker.

Source for this tool is available [in its GitHub repo](https://github.com/ceejbot/version-swap). under [the Parity Public License](https://paritylicense.com). This license requires people who build with this software to share their work. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding. You are welcome to do anything you like with this software, so long as you share what you do.
