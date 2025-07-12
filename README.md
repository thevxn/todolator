# Todolator

## TODO

- See <https://docs.rs/tauri/2.0.0/tauri/window/struct.Window.html>: pub fn request_user_attention()
- After dragging the scroll bar with mouse, navigating with keys is weird
- Automated backups?
- Add ellipsis to title & desc
  - Hovering over them should show a popup/popover with the full text
- Solve current task not resetting after a new task is created
- Fix title bar on Mac (<https://v2.tauri.app/learn/window-customization/#creating-a-custom-titlebar>)
- Decide how to handle recurrence (on demand calculation of recurrences vs pre-calculated instances)

## Design

- Lazily calculate recurrences?
- Binary heap for the actual scheduled reminders (instances)
- Templates/definitions
