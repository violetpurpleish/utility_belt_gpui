//! # utility_belt_gpui
//!
//! A collection of reusable gpui widgets for building desktop applications
//! with the gpui framework and gpui-component theming.
//!
//! ## Modules
//!
//! - [`toolbar`] — A draggable titlebar/toolbar widget.
//! - [`sidebar`] — An animated collapsible sidebar widget with integrated state.
//!
//! ## Version
//! 0.1.0

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod sidebar;
pub mod toolbar;
