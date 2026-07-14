//! A reusable toolbar / titlebar widget for gpui applications.
//!
//! The [`Toolbar`] struct provides a builder-pattern widget that renders a
//! horizontal bar at the top of a window, styled with the application theme.
//! It supports:
//!
//! - A title label on the left side.
//! - Arbitrary child elements on the left (before the spacer).
//! - Arbitrary trailing elements on the right (after the spacer).
//! - Built-in window dragging (double-click to zoom, single-click drag
//!   via native macOS API or the gpui fallback on other platforms).
//!
//! # Required `main.rs` setup
//!
//! To enable the transparent titlebar and correct drag behaviour you **must**
//! configure the window options so the toolbar sits flush against the top of
//! the window frame and macOS traffic-light buttons are positioned correctly:
//!
//! ```ignore
//! use gpui::*;
//!
//! let bounds = Bounds::centered(None, size(px(900.0), px(600.0)), cx);
//! cx.open_window(
//!     WindowOptions {
//!         window_bounds: Some(WindowBounds::Windowed(bounds)),
//!         titlebar: Some(TitlebarOptions {
//!             title: Some("My App".into()),
//!             appears_transparent: true,
//!             traffic_light_position: Some(Point::new(px(10.0), px(19.0))),
//!         }),
//!         ..Default::default()
//!     },
//!     |window, cx| { /* ... */ },
//! )
//! .unwrap();
//! ```
//!
//! The `appears_transparent` flag lets the toolbar content show through the
//! titlebar area, and `traffic_light_position` offsets the window-control
//! buttons so they don't overlap the toolbar's title text.
//!
//! # Example
//!
//! ```ignore
//! use utility_belt_gpui::toolbar::Toolbar;
//!
//! // Inside a Render impl:
//! Toolbar::new("My App")
//!     .child(add_button)
//!     .trailing(save_button)
//!     .build(cx)
//! ```

use gpui::{
    App, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement as _,
    SharedString, Styled, Window, div, px,
};
use gpui_component::{ActiveTheme, h_flex};

/// Signature for window-drag callbacks attached to the toolbar / sidebar header.
pub type DragHandler = Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>;

/// The default drag handler for the toolbar.
///
/// Double-clicking zooms the window; single-click initiates a native
/// window drag on macOS (using `objc` to invoke
/// `performWindowDragWithEvent:`) or falls back to
/// `Window::start_window_move()` on other platforms.
pub fn default_drag_handler(event: &MouseDownEvent, window: &mut Window, _app: &mut App) {
    if event.click_count >= 2 {
        window.zoom_window();
    } else {
        #[cfg(target_os = "macos")]
        unsafe {
            let app: *mut objc::runtime::Object =
                msg_send![class!(NSApplication), sharedApplication];
            let native_event: *mut objc::runtime::Object = msg_send![app, currentEvent];
            let native_window: *mut objc::runtime::Object = msg_send![app, keyWindow];
            if !native_window.is_null() && !native_event.is_null() {
                let _: () = msg_send![native_window, performWindowDragWithEvent: native_event];
            }
        }
        #[cfg(not(target_os = "macos"))]
        window.start_window_move();
    }
}

/// A builder for a draggable toolbar / titlebar.
///
/// See the [module-level documentation](self) for setup instructions
/// and a usage example.
pub struct Toolbar {
    title: SharedString,
    children: Vec<gpui::AnyElement>,
    trailing_children: Vec<gpui::AnyElement>,
    drag_handler: Option<DragHandler>,
}

impl Toolbar {
    /// Create a new toolbar with the given title.
    ///
    /// The title is displayed on the left side of the bar.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            children: Vec::new(),
            trailing_children: Vec::new(),
            drag_handler: None,
        }
    }

    /// Add a child element to the leading (left) section of the toolbar,
    /// before the spacer.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Add a child element to the trailing (right) section of the toolbar,
    /// after the spacer.
    pub fn trailing(mut self, child: impl IntoElement) -> Self {
        self.trailing_children.push(child.into_any_element());
        self
    }

    /// Override the default window-drag handler.
    ///
    /// The default uses [`default_drag_handler`] which supports macOS native
    /// drag and a cross-platform fallback.
    pub fn on_drag(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.drag_handler = Some(Box::new(handler));
        self
    }

    /// Build the toolbar element.
    ///
    /// `cx` must be `&App` (or a reference to something that derefs to
    /// `App`, such as `&mut Context<T>`) so that the toolbar can resolve
    /// the current theme colours for the background and border.
    pub fn build(self, cx: &App) -> impl IntoElement + use<> {
        let handler = self
            .drag_handler
            .unwrap_or_else(|| Box::new(default_drag_handler));

        let mut bar = h_flex()
            .h(px(50.0))
            .w_full()
            .items_center()
            .gap_2()
            .pl(px(80.0))
            .pr_3()
            .overflow_hidden()
            .bg(cx.theme().background)
            .border_b_1()
            .border_color(cx.theme().border)
            .id("toolbar-container")
            .on_mouse_down(MouseButton::Left, handler)
            .child(
                div()
                    .text_size(px(14.0))
                    .child(self.title)
                    .id("toolbar-title"),
            );

        for child in self.children {
            bar = bar.child(child);
        }

        bar = bar.child(div().flex_1());

        for child in self.trailing_children {
            bar = bar.child(child);
        }

        bar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn test_toolbar_build(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let _element = cx.read(|app| Toolbar::new("Test App").build(app).into_any_element());
    }

    #[gpui::test]
    fn test_toolbar_with_children(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let _element = cx.read(|app| {
            Toolbar::new("My App")
                .child(div().child("Left"))
                .child(div().child("Right"))
                .trailing(div().child("End"))
                .build(app)
                .into_any_element()
        });
    }

    #[gpui::test]
    fn test_toolbar_multiple_trailing(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let _element = cx.read(|app| {
            Toolbar::new("App")
                .child(div())
                .child(div())
                .child(div())
                .trailing(div())
                .trailing(div())
                .build(app)
                .into_any_element()
        });
    }

    #[gpui::test]
    fn test_toolbar_custom_drag_handler(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let _element = cx.read(|app| {
            Toolbar::new("Test App")
                .on_drag(|_: &MouseDownEvent, _: &mut Window, _: &mut App| {})
                .build(app)
                .into_any_element()
        });
    }

    #[gpui::test]
    fn test_toolbar_empty_title(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let _element = cx.read(|app| Toolbar::new("").build(app).into_any_element());
    }
}
