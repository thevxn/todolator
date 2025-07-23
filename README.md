# Todolator

## TODO

- Add recurring tasks to FE (BE is ready)
  - Enable deleting all recurrences (delete the definition) or just one single instance (add to exceptions on the definition)
- See <https://docs.rs/tauri/2.0.0/tauri/window/struct.Window.html>: pub fn request_user_attention()
- After dragging the scroll bar with mouse, navigating with keys is weird
- Automated backups?
- Add ellipsis to title & desc
  - Hovering over them should show a popup/popover with the full text
- Fix title bar on Mac (<https://v2.tauri.app/learn/window-customization/#creating-a-custom-titlebar>)
- If you open delete modal with mouse, keybinds dont work
- Remove seconds from date inputs
- Opening a task in edit mode and closing the modal does not reset current task (the task is filled in when New Task is opened after)
