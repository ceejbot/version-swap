# version-swap

`version-swap` is a command-line tool with a boring name that you use with your mod manager to swap versions of the Skryim executable when you launch the game. This allows you to have one game folder and run several different game versions. For example, you might want to run Skyrim version 1.5.97 so you can build grass caches with the [.Net Script Framework](https://www.nexusmods.com/skyrimspecialedition/mods/21294), then switch to version 1.6.1130 so you can have the latest animation native mods while you play.

These instructions help you get going with the tool so you can run any version of Skyrim you want to, without stress.

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
- the empty `_ResourcePack.esl` plugin that came with this tool; docs to be improved here

Then make a second directory, named `skyrim-1.6.1130`. Put the same files inside but for the newer versions of the game.

__Pro tip:__ If a new version of Skyrim is released, you can support it with this tool by making a new directory named `skyrim-*number*` and putting that version's builds of the same files into it.

### Step 4: creations

In each of these subdirectories, make a `data` directory. We'll be stashing the game data that's different here, so the tool can swap them in and out. The new game version has new versions of the four Creation Club mods, but only the Fishing mod has a plugin with incompatible changes. Copy the older version of `ccBGSSSE001-Fish.esm` into the data directory for 1.5.97, and the newer version into the data directory for 1.6.1130. If you don't want to run the fishing creation, leave it out of BOTH directories.

### Step 5: the resource pack plugin

Make sure the empty `_ResourcePack.esl` plugin that came with this tool is in the `data` directory of the `1.5.97` game info. Copy the 1.6.1130 `_ResourcePack.esl` plugin into the matching `data` directory. We need to do this because older game versions crash if they attempt to load that plugin file. You can safely leave the `_ResourcePack.bsa` file in place! The game won't do anything with it without the matching plugin.

### Step 6: review the folder

When you're done, your game folder (or root mod) should include these files:

```text
.
├── version-swap.exe
├── skse64_1_5_97.dll
├── skse64_1_6_1130.dll
├── skse64_steam_loader.dll
└── Versions
   ├── skyrim-1.5.97
   │  ├── data
   │  │  ├── _ResourcePack.esl
   │  │  └── ccBGSSSE001-Fish.esm
   │  ├── skse64_loader.exe
   │  ├── SkyrimSE.exe
   │  └── steam_api64.dll
   └── skyrim-1.6.1130
      ├── data
      │  ├── _ResourcePack.esl
      │  └── ccBGSSSE001-Fish.esm
      ├── skse64_loader.exe
      ├── SkyrimSE.exe
      └── steam_api64.dll
```

__Pro tip:__ These files are the ones you *must* swap in to run different versions of the game. This tool will copy in *any* files you have in a version folder, so if your setup has additional DLLs you need to swap, version-swap can do it for you. It will not copy entire subdirectories! Use a mod manager for that.

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