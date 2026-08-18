//! A reusable Catppuccin-inspired theme for gpui-component applications.
//!
//! The palette uses Catppuccin's deep indigo and warm paper surfaces with
//! violet primary accents. Call [`apply`] after `gpui_component::init` to
//! install it as the application's light and dark theme.

use std::rc::Rc;

use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeConfig, ThemeConfigColors, ThemeMode};

fn color(hex: &'static str) -> Option<SharedString> {
    Some(hex.into())
}

macro_rules! set_colors {
    ($colors:ident { $($field:ident: $value:literal),+ $(,)? }) => {
        $(
            $colors.$field = color($value);
        )+
    };
}

/// Install the Catppuccin Violet light and dark themes.
///
/// `gpui_component::init(cx)` must be called before this function. The
/// existing system-selected mode is preserved, and the corresponding custom
/// configuration is then applied to the global theme.
pub fn apply(cx: &mut App) {
    let mode = Theme::global(cx).mode;

    {
        let theme = Theme::global_mut(cx);
        theme.dark_theme = Rc::new(dark());
        theme.light_theme = Rc::new(light());
    }

    Theme::change(mode, None, cx);
}

/// Catppuccin Violet dark theme.
pub fn dark() -> ThemeConfig {
    let mut colors = ThemeConfigColors::default();

    set_colors!(colors {
        accent: "#45475a",
        accent_foreground: "#cdd6f4",
        accordion: "#181825",
        accordion_hover: "#313244",
        background: "#1e1e2e",
        border: "#313244",
        group_box: "#181825",
        group_box_foreground: "#cdd6f4",
        group_box_title_foreground: "#b4befe",
        caret: "#cba6f7",
        chart_1: "#89b4fa",
        chart_2: "#94e2d5",
        chart_3: "#a6e3a1",
        chart_4: "#fab387",
        chart_5: "#cba6f7",
        danger: "#f38ba8",
        danger_active: "#d96b89",
        danger_foreground: "#11111b",
        danger_hover: "#f5a0b7",
        description_list_label: "#313244",
        description_list_label_foreground: "#a6adc8",
        drag_border: "#b4befe",
        drop_target: "#cba6f733",
        foreground: "#cdd6f4",
        info: "#89b4fa",
        info_active: "#6f9de0",
        info_foreground: "#11111b",
        info_hover: "#74c7ec",
        input: "#45475a",
        link: "#89b4fa",
        link_active: "#74c7ec",
        link_hover: "#b4befe",
        list: "#1e1e2e",
        list_active: "#45475a",
        list_active_border: "#cba6f7",
        list_even: "#181825",
        list_head: "#313244",
        list_hover: "#313244",
        muted: "#313244",
        muted_foreground: "#a6adc8",
        popover: "#181825",
        popover_foreground: "#cdd6f4",
        primary: "#cba6f7",
        primary_active: "#b990f5",
        primary_foreground: "#11111b",
        primary_hover: "#b4befe",
        progress_bar: "#cba6f7",
        ring: "#cba6f7",
        scrollbar: "#1e1e2e",
        scrollbar_thumb: "#585b70",
        scrollbar_thumb_hover: "#6c7086",
        secondary: "#313244",
        secondary_active: "#45475a",
        secondary_foreground: "#cdd6f4",
        secondary_hover: "#45475a",
        selection: "#cba6f7",
        sidebar: "#181825",
        sidebar_accent: "#313244",
        sidebar_accent_foreground: "#cdd6f4",
        sidebar_border: "#313244",
        sidebar_foreground: "#cdd6f4",
        sidebar_primary: "#cba6f7",
        sidebar_primary_foreground: "#11111b",
        skeleton: "#313244",
        slider_bar: "#45475a",
        slider_thumb: "#cba6f7",
        success: "#a6e3a1",
        success_foreground: "#11111b",
        success_hover: "#b5e8ae",
        success_active: "#8bcf87",
        bullish: "#a6e3a1",
        bearish: "#f38ba8",
        switch: "#45475a",
        switch_thumb: "#cdd6f4",
        tab: "#1e1e2e",
        tab_active: "#313244",
        tab_active_foreground: "#cdd6f4",
        tab_bar: "#1e1e2e",
        tab_bar_segmented: "#313244",
        tab_foreground: "#a6adc8",
        table: "#1e1e2e",
        table_active: "#45475a",
        table_active_border: "#cba6f7",
        table_even: "#181825",
        table_head: "#313244",
        table_head_foreground: "#a6adc8",
        table_hover: "#313244",
        table_row_border: "#313244",
        title_bar: "#181825",
        title_bar_border: "#313244",
        tiles: "#181825",
        warning: "#f9e2af",
        warning_active: "#d9c184",
        warning_hover: "#f6d79a",
        warning_foreground: "#11111b",
        overlay: "#11111bcc",
        window_border: "#313244",
    });

    ThemeConfig {
        name: "Catppuccin Violet Dark".into(),
        mode: ThemeMode::Dark,
        colors,
        ..Default::default()
    }
}

/// Catppuccin Violet light theme.
pub fn light() -> ThemeConfig {
    let mut colors = ThemeConfigColors::default();

    set_colors!(colors {
        accent: "#dce0e8",
        accent_foreground: "#4c4f69",
        accordion: "#e6e9ef",
        accordion_hover: "#dce0e8",
        background: "#eff1f5",
        border: "#ccd0da",
        group_box: "#e6e9ef",
        group_box_foreground: "#4c4f69",
        group_box_title_foreground: "#8839ef",
        caret: "#7c3aed",
        chart_1: "#1e66f5",
        chart_2: "#179299",
        chart_3: "#40a02b",
        chart_4: "#fe640b",
        chart_5: "#8839ef",
        danger: "#d20f39",
        danger_active: "#b80d31",
        danger_foreground: "#ffffff",
        danger_hover: "#e3424f",
        description_list_label: "#e6e9ef",
        description_list_label_foreground: "#6c6f85",
        drag_border: "#7c3aed",
        drop_target: "#7c3aed33",
        foreground: "#4c4f69",
        info: "#1e66f5",
        info_active: "#1854ca",
        info_foreground: "#ffffff",
        info_hover: "#04a5e5",
        input: "#bcc0cc",
        link: "#1e66f5",
        link_active: "#1854ca",
        link_hover: "#04a5e5",
        list: "#eff1f5",
        list_active: "#dce0e8",
        list_active_border: "#7c3aed",
        list_even: "#e6e9ef",
        list_head: "#e6e9ef",
        list_hover: "#dce0e8",
        muted: "#ccd0da",
        muted_foreground: "#6c6f85",
        popover: "#ffffff",
        popover_foreground: "#4c4f69",
        primary: "#7c3aed",
        primary_active: "#5b21b6",
        primary_foreground: "#ffffff",
        primary_hover: "#6d28d9",
        progress_bar: "#7c3aed",
        ring: "#7c3aed",
        scrollbar: "#eff1f5",
        scrollbar_thumb: "#bcc0cc",
        scrollbar_thumb_hover: "#acb0be",
        secondary: "#e6e9ef",
        secondary_active: "#dce0e8",
        secondary_foreground: "#4c4f69",
        secondary_hover: "#dce0e8",
        selection: "#7c3aed",
        sidebar: "#e6e9ef",
        sidebar_accent: "#dce0e8",
        sidebar_accent_foreground: "#4c4f69",
        sidebar_border: "#ccd0da",
        sidebar_foreground: "#4c4f69",
        sidebar_primary: "#7c3aed",
        sidebar_primary_foreground: "#ffffff",
        skeleton: "#e6e9ef",
        slider_bar: "#bcc0cc",
        slider_thumb: "#7c3aed",
        success: "#40a02b",
        success_foreground: "#ffffff",
        success_hover: "#4cae36",
        success_active: "#358724",
        bullish: "#40a02b",
        bearish: "#d20f39",
        switch: "#bcc0cc",
        switch_thumb: "#ffffff",
        tab: "#eff1f5",
        tab_active: "#e6e9ef",
        tab_active_foreground: "#4c4f69",
        tab_bar: "#eff1f5",
        tab_bar_segmented: "#e6e9ef",
        tab_foreground: "#6c6f85",
        table: "#eff1f5",
        table_active: "#dce0e8",
        table_active_border: "#7c3aed",
        table_even: "#e6e9ef",
        table_head: "#e6e9ef",
        table_head_foreground: "#6c6f85",
        table_hover: "#dce0e8",
        table_row_border: "#ccd0da",
        title_bar: "#e6e9ef",
        title_bar_border: "#ccd0da",
        tiles: "#e6e9ef",
        warning: "#df8e1d",
        warning_active: "#c47c18",
        warning_hover: "#e7a03b",
        warning_foreground: "#ffffff",
        overlay: "#4c4f6999",
        window_border: "#ccd0da",
    });

    ThemeConfig {
        name: "Catppuccin Violet Light".into(),
        mode: ThemeMode::Light,
        colors,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[test]
    fn themes_have_distinct_modes_and_names() {
        assert_eq!(dark().name.to_string(), "Catppuccin Violet Dark");
        assert_eq!(dark().mode, ThemeMode::Dark);
        assert_eq!(light().name.to_string(), "Catppuccin Violet Light");
        assert_eq!(light().mode, ThemeMode::Light);
    }

    #[test]
    fn themes_define_the_shared_component_surfaces() {
        for theme in [dark(), light()] {
            assert!(theme.colors.accent.is_some());
            assert!(theme.colors.input.is_some());
            assert!(theme.colors.list_hover.is_some());
            assert!(theme.colors.popover.is_some());
            assert!(theme.colors.sidebar.is_some());
            assert!(theme.colors.tab_active.is_some());
            assert!(theme.colors.table_hover.is_some());
        }
    }

    #[gpui::test]
    fn apply_installs_both_themes(cx: &mut TestAppContext) {
        cx.update(|app| {
            gpui_component::theme::init(app);
            apply(app);

            let theme = Theme::global(app);
            assert_eq!(
                theme.light_theme.name.to_string(),
                "Catppuccin Violet Light"
            );
            assert_eq!(theme.dark_theme.name.to_string(), "Catppuccin Violet Dark");
        });
    }
}
