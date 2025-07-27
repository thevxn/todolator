# Todolator

Todolator is a simple desktop task reminder designed to ensure it is hard to ignore the reminder (unlike other apps using push or in-brwoser notifications).

## Why

I created Todolator because usual push notifications or in-browser notifications from apps like Google Calendar are not enough to grab my attention and not forget the task at hand. I also had experience with desktop apps like [Kana](https://kanasolution.com/products/kana-reminder/) and [Desktop Reminder](https://www.desktop-reminder.com/en/index.html), however these are now quite outdated and/or not completely free.

I also wanted to create a desktop app, learn the basics of Rust and use Tauri - so I can combined all of these in this project.

## Use Cases

- Reminding of important, one-time tasks
  - E.g. an important meeting
- Reminding of recurring tasks
  - E.g. paying your bills
- Combination of the 2 UCs above

## Privacy

Your data stays on your computer, you decide what to do with it. Todolator uses no telemetry, stores no data about you and does not require internet connection.

## Configuration

### App Data Directory

The app data directory location depends on the OS used:

**Windows:**

- `C:\Users\<Username>\AppData\Roaming\Todolator`

**Linux:**

- `/home/<username>/.local/share/Todolator`

**MacOS:**

- `/Users/<Username>/Library/Application Support/Todolator`

In the location, you can find `tasks.json` holding your tasks, `settings.json` containing app settings and `resources/alarm.mp3` containing the alarm notification sound.

### Changing the notification sound

Simply replace the default `alarm.mp3` file in the `resources` directory located inside of the app data directory (see above).

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

- LICENCE
- Try building for MacOS via GHA pipeline provided by tauri
- Mention in readme the location of `tasks.json`, `settings.json` and `alarm.mp3`
- Calendar units for recurrence
- GUI paging

## Future Releases

- MacOS release
  - Fix title bar (<https://v2.tauri.app/learn/window-customization/#creating-a-custom-titlebar>)
- Clean up the app data directory when uninstalling (maybe make it optional with a checkbox?)
- Import/Export tasks
- Filtering & Sorting tasks
- Configurable hotkeys
  - Replace current hardcoded hotkeys with variables & create mapping
- Automated backups?
- After dragging the scroll bar with mouse, navigating with keys is weird
  - Desync

## Attributions

- Default notification sound: <https://pixabay.com/sound-effects/new-notification-07-210334/>
