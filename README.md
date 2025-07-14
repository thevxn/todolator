# Todolator

## TODO

- See <https://docs.rs/tauri/2.0.0/tauri/window/struct.Window.html>: pub fn request_user_attention()
- After dragging the scroll bar with mouse, navigating with keys is weird
- Automated backups?
- Add ellipsis to title & desc
  - Hovering over them should show a popup/popover with the full text
- Solve current task not resetting after a new task is created
- Fix title bar on Mac (<https://v2.tauri.app/learn/window-customization/#creating-a-custom-titlebar>)
- Add a way to track the last succesful reminder of a Task Definition so that instances are spawned from the last reminded time instead of starting at the first ocurrence every time
  - Added Recurrence enum to definitions
  - TODO: If the definition is recurring, set the `last_recurrence` to `start` initially
