# TimeTracker

An app for tracking time™.

Now seriously, at some point in the future you will be able to (hopefully) log all kinds of events, activities, what you're doing, where you're going, how long it takes, and also things like weight, exercise logs, mood, sleeping time, etc. And later maybe some in-app analytics.

And everything is based on transparent, human-readable text files, which can be synced e.g. peer-to-peer using Syncthing.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Quickstart (dev)

First follow [Tauri prerequisites](https://tauri.app/start/prerequisites/). Install the tools required for the platforms you want to use (desktop/Android/iOS).

Install [pnpm](https://pnpm.io/installation) as the JavaScript package manager. It's somewhat similar to `npm`, but has a global package cache, so it takes up much less space on disk and doesn't have to download all dependencies every time.

Go to repo root.

Use `pnpm install` to install dependencies.

Then use `pnpm tauri dev` to run the app on desktop, or `pnpm tauri android dev`/`pnpm tauri ios dev` to run on mobile. On the first run it will probably install Rust dependencies, so it might take some time.

## Troubleshooting

### Incorrect Java path (Linux)

`/opt/android-studio/jbr doesn't exist`

On Linux, the [official guide](https://developer.android.com/studio/install#linux) mentions two install paths: `/usr/local/` or `/opt/`. If you install into `/usr/local/`, you need to change `JAVA_PATH` accordingly: `export JAVA_HOME=/usr/local/android-studio/jbr`. I know, pretty obvious, but if you installed Android Studio a few months ago, it's easy to miss.

### Incorrect Android architecture

`Error running 'app' The currently selected variant "armDebug" uses split APKs, but none of the 1 split apks are compatible with the current device with ABIs "arm64-v8a"`

Open project in Android Studio, on the left bar, open Build Variants panel, and change Active Build Variant to arm64Debug (or other, depending on your device architecture). [Solution from @roomtin on Tauri's Discord.](https://discord.com/channels/616186924390023171/731495028677148753/1462569288844972085)

### Can't connect to Android device (Linux?)

Make sure you have `adb` installed and the daemon running. Try e.g. `Android/Sdk/platform-tools/adb devices -l` (or wherever your `adb` is) to list connected devices.
