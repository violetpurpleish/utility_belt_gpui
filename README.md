# utility_belt_gpui

Reusable [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) widgets for desktop applications. The crate is built against `gpui` 0.2.2 and `gpui-component` 0.5.1.

## Features

- **Toolbar** — a theme-aware, draggable titlebar/toolbar with leading and trailing content.
- **Sidebar** — an animated, collapsible sidebar with a configurable header, actions, and scrollable content.
- **Catppuccin Violet theme** — light and dark `gpui-component` theme configurations with system mode preservation.
- **Cross-platform window dragging** — native macOS dragging with a GPUI fallback on other platforms.

## Installation

Add the repository as a Git dependency:

```toml
[dependencies]
utility_belt_gpui = { git = "https://github.com/gitwyrm/utility_belt_gpui.git" }
```

When developing against a local checkout, use a path dependency instead:

```toml
[dependencies]
utility_belt_gpui = { path = "../utility_belt_gpui" }
```

The crate currently targets Rust edition 2024 and exposes the following modules:

```rust
use utility_belt_gpui::{sidebar, theme, toolbar};
```

## Initialization and theming

Initialize `gpui-component` before installing the custom theme:

```rust
app.run(|cx| {
    gpui_component::init(cx);
    utility_belt_gpui::theme::apply(cx);

    // Open windows and start your application here.
});
```

`theme::apply` replaces both the light and dark theme configurations while preserving the mode selected by the system. The individual configurations are also available through `theme::light()` and `theme::dark()`.

## Toolbar

`Toolbar` uses a builder API. Children added with `child` appear on the left of the flexible spacer; children added with `trailing` appear on the right.

```rust
use utility_belt_gpui::toolbar::Toolbar;

let toolbar = Toolbar::new("My App")
    .child(add_button)
    .trailing(save_button)
    .build(cx);
```

The toolbar handles window dragging by default. Double-clicking the toolbar zooms the window, while a single click starts a window drag. Supply `on_drag` when an application needs custom behavior:

```rust
let toolbar = Toolbar::new("My App")
    .on_drag(|event, window, app| {
        // Custom handling for the toolbar's left-button mouse-down event.
        let _ = (event, window, app);
    })
    .build(cx);
```

### Transparent titlebar setup

For the toolbar to occupy the titlebar area, configure the window with a transparent titlebar. On macOS, set `traffic_light_position` so the window controls do not overlap the toolbar content:

```rust
use gpui::{Bounds, Point, TitlebarOptions, WindowBounds, WindowOptions, px, size};

let bounds = Bounds::centered(None, size(px(900.0), px(600.0)), cx);

cx.open_window(
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions {
            title: Some("My App".into()),
            appears_transparent: true,
            traffic_light_position: Some(Point::new(px(10.0), px(19.0))),
        }),
        ..Default::default()
    },
    |_window, _cx| {
        // Create the application's root view here.
    },
)?;
```

## Sidebar

Keep a `SidebarState` in the owning view and pass it to `Sidebar::build`. The builder advances the width animation and requests animation frames as needed.

```rust
use utility_belt_gpui::sidebar::{Sidebar, SidebarState};

struct AppView {
    sidebar_state: SidebarState,
}

impl AppView {
    fn new() -> Self {
        Self {
            sidebar_state: SidebarState::new(true, 260.0),
        }
    }

    fn toggle_sidebar(&mut self) {
        self.sidebar_state.toggle();
    }
}
```

Build the sidebar from a render method or another place with access to the GPUI window and application context:

```rust
use gpui::{div, Window};
use gpui_component::v_flex;
use utility_belt_gpui::sidebar::Sidebar;

fn sidebar_element(
    &mut self,
    window: &mut Window,
    cx: &gpui::App,
) -> impl gpui::IntoElement {
    Sidebar::new()
        .title("History")
        .header_action(clear_button)
        .content(
            v_flex()
                .gap_1()
                .child(div().child("Recent item")),
        )
        .build(&mut self.sidebar_state, window, cx)
}
```

`SidebarState::new(is_open, width)` sets the initial state and open width in pixels. Call `toggle()` from an event handler to animate between the open width and zero. The sidebar content is placed inside a scrollable container, and multiple header actions can be added with repeated calls to `header_action`.

## Window dragging

`toolbar::default_drag_handler` is shared by the toolbar and sidebar header and is available for reuse. The public `toolbar::DragHandler` type alias can be used when storing a drag callback.

- On macOS, single-click dragging uses the native `performWindowDragWithEvent:` API.
- On other platforms, single-click dragging falls back to `Window::start_window_move()`.
- Double-clicking calls `Window::zoom_window()` on all platforms.

Both builders support `.on_drag(...)` for replacing this behavior.

## Development

From the repository root:

```sh
cargo check
cargo test
cargo clippy --all-targets
cargo fmt
```

The project includes unit tests for the theme, toolbar builders, sidebar builders, and sidebar animation state.

## Third-party notices

The Catppuccin Violet palette is inspired by [Catppuccin](https://github.com/catppuccin/catppuccin). The palette and licensing notice are documented in [`THIRD-PARTY-LICENSES.md`](THIRD-PARTY-LICENSES.md).
