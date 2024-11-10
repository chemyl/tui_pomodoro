### Text-based User Interface app. Pomodoro

- create binary crate -> `cargo new project`
- add dependency to `cargo.toml`-> `cursive = "0.21.1"`
- include cursive usage inside `main.rs`
- init `cursive` object
- set terminal `theme`
- create base variables with `Arc::new(Mutex::new())`
- add TUI `Dialog` with buttons
- create 2 `thread`
- * First thread handles countdown of the timer (background thread).
- * Second thread handles refreshing of the UI every second.
- `siv.run()`

### terminal launch
![launcher window](https://github.com/chemyl/tui_pomodoro/blob/master/img.png)
