# utility_belt_gpui

Reusable gpui widgets built on gpui 0.2.2 and gpui-component 0.5.1.

## Build & Test
- `cargo check` — verify compilation
- `cargo test` — run all tests
- `cargo clippy --all-targets` — lint check
- `cargo fmt` — format code

## Project Structure
- `src/lib.rs` — crate root, module declarations
- `src/toolbar.rs` — `Toolbar` builder, `default_drag_handler`, `DragHandler` type
- `src/sidebar.rs` — `SidebarState`, `Sidebar` builder

## Dependencies
- gpui (with `test-support` feature)
- gpui-component 0.5.1
- objc 0.2 (macOS window drag)

## Key API

### Toolbar
```rust
use utility_belt_gpui::toolbar::Toolbar;

Toolbar::new("My App")
    .child(add_button)          // left of spacer
    .trailing(save_button)      // right of spacer
    .build(cx)                  // &App or &mut Context<T>
```
Requires `appears_transparent: true` in `TitlebarOptions` in `main.rs`.

### Sidebar
```rust
use utility_belt_gpui::sidebar::{Sidebar, SidebarState};

// In state: sidebar_state: SidebarState,
// Initialise: SidebarState::new(true, 260.0)
// Toggle: self.sidebar_state.toggle()

Sidebar::new()
    .title("History")
    .header_action(clear_button)
    .content(v_flex().children(items))
    .build(&mut state.sidebar_state, window, cx)
```

### DragHandler
The `DragHandler` type alias and `default_drag_handler` function are public
in `toolbar` and can be reused by the sidebar header.

## Conventions
- Edition 2024: `impl Trait` return types use `+ use<>` to avoid capturing
  lifetimes from `&App` / `&mut Window` parameters.
- `#[cfg(target_os = "macos")]` guards the objc drag path; non-macOS falls
  back to `window.start_window_move()`.

## Consuming crate
Add to `Cargo.toml`:
```toml
utility_belt_gpui = { path = "../utility_belt_gpui" }
```

## Committing
Only commit if the user asks you to. Always do `cargo fmt` before committing.
