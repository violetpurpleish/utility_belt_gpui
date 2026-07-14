//! An animated, collapsible sidebar widget for gpui applications.
//!
//! The sidebar provides a sliding panel that animates between open and closed
//! states.  It is driven by a state object ([`SidebarState`]) that tracks
//! `is_open` and the current animated width, and a builder ([`Sidebar`]) that
//! configures the header and body content.
//!
//! # Example
//!
//! ```ignore
//! use utility_belt_gpui::sidebar::{Sidebar, SidebarState};
//!
//! // In your app state:
//! //   sidebar_state: SidebarState,
//!
//! // Initialise with:
//! //   sidebar_state: SidebarState::new(true, 260.0),
//!
//! // Toggle open/closed:
//! //   self.sidebar_state.toggle();
//!
//! // Inside a Render impl:
//! Sidebar::new()
//!     .title("History")
//!     .header_action(clear_button)
//!     .content(
//!         v_flex()
//!             .p_2()
//!             .gap_1()
//!             .children(history_items)
//!     )
//!     .build(&mut state.sidebar_state, window, cx)
//! ```

use gpui::{
    App, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement as _,
    SharedString, Styled, Window, div, px,
};
use gpui_component::scroll::ScrollableElement;
use gpui_component::{ActiveTheme, h_flex, v_flex};

use crate::toolbar::{DragHandler, default_drag_handler};

/// Tracks the open/closed state and animation progress of a sidebar.
///
/// Create one of these in your application state and call [`toggle`](SidebarState::toggle)
/// to switch between open and closed.  Pass a mutable reference to
/// [`Sidebar::build`] so it can drive the animation each frame.
pub struct SidebarState {
    /// Whether the sidebar is logically open.
    pub is_open: bool,
    /// Monotonically increasing counter bumped on each toggle (useful for
    /// forcing re-renders / animation restarts).
    pub anim_id: usize,
    /// The current rendered width of the sidebar (animates towards the target
    /// width when toggled).
    pub current_width: f32,
    width: f32,
}

impl SidebarState {
    /// Create a new sidebar state.
    ///
    /// * `is_open` — whether the sidebar starts open.
    /// * `width` — the full width (in pixels) the sidebar animates to when open.
    pub fn new(is_open: bool, width: f32) -> Self {
        Self {
            is_open,
            anim_id: 0,
            current_width: if is_open { width } else { 0.0 },
            width,
        }
    }

    /// Toggle the sidebar open/closed.
    ///
    /// Call this from your event handler (e.g., a toolbar button click).
    /// The animation will be driven automatically the next time
    /// [`Sidebar::build`] is called.
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        self.anim_id = self.anim_id.wrapping_add(1);
    }

    /// Advance the animation one step and return the current width.
    ///
    /// Callers don't normally need this directly — it is invoked by
    /// [`Sidebar::build`].
    pub fn animate(&mut self, window: &mut Window) -> f32 {
        let target_w = if self.is_open { self.width } else { 0.0 };
        let diff = target_w - self.current_width;
        if diff.abs() > 0.5 {
            let step = (self.width / 12.0).max(2.0);
            if diff > 0.0 {
                self.current_width = (self.current_width + step).min(target_w);
            } else {
                self.current_width = (self.current_width - step).max(target_w);
            }
            window.request_animation_frame();
        }
        self.current_width
    }
}

/// A builder for an animated collapsible sidebar.
///
/// See the [module-level documentation](self) for a usage example.
pub struct Sidebar {
    title: SharedString,
    header_actions: Vec<gpui::AnyElement>,
    content: Option<gpui::AnyElement>,
    drag_handler: Option<DragHandler>,
}

impl Sidebar {
    /// Create a new sidebar builder.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            title: "Sidebar".into(),
            header_actions: Vec::new(),
            content: None,
            drag_handler: None,
        }
    }
}

impl Sidebar {
    /// Set the header title.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Add an action element to the right side of the header (e.g. a "Clear"
    /// button).
    pub fn header_action(mut self, element: impl IntoElement) -> Self {
        self.header_actions.push(element.into_any_element());
        self
    }

    /// Set the body content of the sidebar.
    ///
    /// The content is placed inside a scrollable container. Pass a single
    /// element (typically a `v_flex` wrapping your items).
    pub fn content(mut self, element: impl IntoElement) -> Self {
        self.content = Some(element.into_any_element());
        self
    }

    /// Override the default drag handler for the sidebar header.
    ///
    /// By default the same [`default_drag_handler`] used by [`Toolbar`] is
    /// applied, giving macOS native window-drag behaviour.
    pub fn on_drag(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.drag_handler = Some(Box::new(handler));
        self
    }

    /// Build the sidebar element.
    ///
    /// * `state` — mutable reference to the [`SidebarState`] controlling
    ///   open/closed state and animation.
    /// * `window` — needed for `request_animation_frame` calls.
    /// * `cx` — the gpui application context (`&App` or `&mut Context<T>`).
    pub fn build(
        self,
        state: &mut SidebarState,
        window: &mut Window,
        cx: &App,
    ) -> impl IntoElement + use<> {
        let w = state.animate(window);

        if w < 1.0 && !state.is_open {
            return div().w(px(0.)).flex_shrink_0().into_any_element();
        }

        let handler = self
            .drag_handler
            .unwrap_or_else(|| Box::new(default_drag_handler));

        let content = self.content.unwrap_or_else(|| div().into_any_element());

        v_flex()
            .w(px(w))
            .flex_shrink_0()
            .h_full()
            .bg(cx.theme().sidebar)
            .text_color(cx.theme().sidebar_foreground)
            .border_l_1()
            .border_color(cx.theme().sidebar_border)
            .child(
                h_flex()
                    .h(px(50.0))
                    .items_center()
                    .pl_3()
                    .pr_3()
                    .bg(cx.theme().background)
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .id("sidebar-header")
                    .on_mouse_down(MouseButton::Left, handler)
                    .child(div().text_size(px(15.)).child(self.title))
                    .child(div().flex_1().h_full())
                    .children(self.header_actions),
            )
            .child(div().flex_1().overflow_y_scrollbar().child(content))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    // ---- SidebarState unit tests ----

    #[test]
    fn test_new_open() {
        let state = SidebarState::new(true, 260.0);
        assert!(state.is_open);
        assert_eq!(state.current_width, 260.0);
        assert_eq!(state.anim_id, 0);
    }

    #[test]
    fn test_new_closed() {
        let state = SidebarState::new(false, 260.0);
        assert!(!state.is_open);
        assert_eq!(state.current_width, 0.0);
        assert_eq!(state.anim_id, 0);
    }

    #[test]
    fn test_new_custom_width() {
        let state = SidebarState::new(true, 400.0);
        assert_eq!(state.current_width, 400.0);
        assert!(state.is_open);
    }

    #[test]
    fn test_toggle_open_closed() {
        let mut state = SidebarState::new(true, 260.0);
        state.toggle();
        assert!(!state.is_open);
        assert_eq!(state.anim_id, 1);

        state.toggle();
        assert!(state.is_open);
        assert_eq!(state.anim_id, 2);
    }

    #[test]
    fn test_toggle_anim_id_wrapping() {
        let mut state = SidebarState::new(false, 260.0);
        state.anim_id = usize::MAX;
        state.toggle();
        assert!(state.is_open);
        assert_eq!(state.anim_id, 0);
    }

    // ---- SidebarState animation tests (require gpui context) ----

    struct AnimateView {
        state: SidebarState,
    }

    impl gpui::Render for AnimateView {
        fn render(
            &mut self,
            window: &mut Window,
            _cx: &mut gpui::Context<Self>,
        ) -> impl IntoElement {
            self.state.animate(window);
            div()
        }
    }

    #[gpui::test]
    fn test_animate_opens(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let mut state = SidebarState::new(false, 260.0);
        state.toggle();

        let (view, cx) = cx.add_window_view(|_window, _cx| AnimateView { state });

        view.update(cx, |v, _| {
            assert!(v.state.current_width > 0.0 && v.state.current_width < 260.0);
            assert!(v.state.is_open);
        });
    }

    #[gpui::test]
    fn test_animate_closes(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let mut state = SidebarState::new(true, 260.0);
        state.toggle();

        let (view, cx) = cx.add_window_view(|_window, _cx| AnimateView { state });

        view.update(cx, |v, _| {
            assert!(v.state.current_width > 0.0 && v.state.current_width < 260.0);
            assert!(!v.state.is_open);
        });
    }

    #[gpui::test]
    fn test_animate_full_open(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let state = SidebarState::new(true, 260.0);

        let (view, cx) = cx.add_window_view(|_window, _cx| AnimateView { state });

        view.update(cx, |v, _| {
            assert_eq!(v.state.current_width, 260.0);
        });
    }

    #[gpui::test]
    fn test_animate_full_closed(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let state = SidebarState::new(false, 260.0);

        let (view, cx) = cx.add_window_view(|_window, _cx| AnimateView { state });

        view.update(cx, |v, _| {
            assert_eq!(v.state.current_width, 0.0);
        });
    }

    #[gpui::test]
    fn test_animate_converges_to_target(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let state = SidebarState::new(false, 260.0);

        let (view, cx) = cx.add_window_view(|_window, _cx| AnimateView { state });

        // Toggle to start animating from 0 toward 260
        view.update(cx, |v, _| {
            assert_eq!(v.state.current_width, 0.0);
            v.state.toggle();
        });

        let step = (260.0_f32 / 12.0_f32).max(2.0);
        let steps_needed = (260.0_f32 / step).ceil() as usize;

        // Trigger repeated renders to advance the animation
        for _ in 0..steps_needed {
            cx.update(|window, app| {
                let _ = window.draw(app);
            });
        }

        view.update(cx, |v, _| {
            assert_eq!(v.state.current_width, 260.0);
            assert!(v.state.is_open);
        });
    }

    // ---- Sidebar builder tests ----

    #[test]
    fn test_sidebar_defaults() {
        let sidebar = Sidebar::new();
        assert_eq!(sidebar.title, SharedString::from("Sidebar"));
        assert!(sidebar.header_actions.is_empty());
        assert!(sidebar.content.is_none());
        assert!(sidebar.drag_handler.is_none());
    }

    #[test]
    fn test_sidebar_builder_chaining() {
        let sidebar = Sidebar::new()
            .title("My Sidebar")
            .header_action(div())
            .content(div().child("Body"))
            .on_drag(|_, _, _| {});

        assert_eq!(sidebar.title, SharedString::from("My Sidebar"));
        assert_eq!(sidebar.header_actions.len(), 1);
        assert!(sidebar.content.is_some());
        assert!(sidebar.drag_handler.is_some());
    }

    #[gpui::test]
    fn test_sidebar_build_open(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let mut state = SidebarState::new(true, 200.0);
        let cx = cx.add_empty_window();

        cx.update(|window, app| {
            let _element = Sidebar::new()
                .title("Test")
                .content(div().child("Content"))
                .build(&mut state, window, app)
                .into_any_element();
        });
    }

    #[gpui::test]
    fn test_sidebar_build_closed(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let mut state = SidebarState::new(false, 200.0);
        let cx = cx.add_empty_window();

        cx.update(|window, app| {
            let _element = Sidebar::new()
                .title("Test")
                .build(&mut state, window, app)
                .into_any_element();
        });
    }

    struct SidebarBuildView {
        sidebar_state: SidebarState,
        title: SharedString,
    }

    impl gpui::Render for SidebarBuildView {
        fn render(
            &mut self,
            window: &mut Window,
            cx: &mut gpui::Context<Self>,
        ) -> impl IntoElement {
            Sidebar::new()
                .title(self.title.clone())
                .content(div().child("Content"))
                .build(&mut self.sidebar_state, window, cx)
        }
    }

    #[gpui::test]
    fn test_sidebar_build_animates_state(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let mut state = SidebarState::new(false, 200.0);
        state.toggle();

        let (view, cx) = cx.add_window_view(|_window, _cx| SidebarBuildView {
            sidebar_state: state,
            title: "Test".into(),
        });

        view.update(cx, |v, _| {
            assert!(v.sidebar_state.current_width > 0.0);
            assert!(v.sidebar_state.current_width < 200.0);
        });

        let w1 = view.update(cx, |v, _| v.sidebar_state.current_width);

        // Trigger another render cycle by drawing the window
        cx.update(|window, app| {
            let _ = window.draw(app);
        });

        let w2 = view.update(cx, |v, _| v.sidebar_state.current_width);
        assert!(w2 > w1);
        assert!(w2 <= 200.0);
    }

    #[gpui::test]
    fn test_sidebar_build_with_actions(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
        });

        let mut state = SidebarState::new(true, 300.0);
        let cx = cx.add_empty_window();

        cx.update(|window, app| {
            let _element = Sidebar::new()
                .title("History")
                .header_action(div().child("X"))
                .header_action(div().child("Y"))
                .build(&mut state, window, app)
                .into_any_element();
        });
    }
}
