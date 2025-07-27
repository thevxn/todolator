# Todolator

## Configuration

Your tasks and settings' location will depend on your OS:

**Windows**

- ``

**Linux**

- ``

**MacOS**

- ``

## Run Locally

```sh
npm i && \
npm run tauri dev
```

## Build

```sh
npm run tauri build --release
```

The built binary & installers will be created in `src-tauri/target/release`.

## TODO

- Try building for MacOS via GHA pipeline provided by tauri
- Mention in readme the location of `tasks.json`, `settings.json` and `alarm.mp3`
- Calendar units for recurrence
- GUI paging

## Future Releases

- MacOS release
  - Fix title bar (<https://v2.tauri.app/learn/window-customization/#creating-a-custom-titlebar>)
- Clean up the app data folder when uninstalling (maybe make it optional with a checkbox?)
- Import/Export tasks
- Filtering & Sorting tasks
- Configurable hotkeys
  - Replace current hardcoded hotkeys with variables & create mapping
- Automated backups?
- After dragging the scroll bar with mouse, navigating with keys is weird
  - Desync

## Attributions

- Default notification sound: <https://pixabay.com/sound-effects/new-notification-07-210334/>
