use std::fmt::{self, Debug};

use gpui::{Hsla, Rgba};
use theme::{
    Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, SystemColors,
    ThemeColorsRefinement, UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily,
    UserThemeStylesRefinement,
};

struct RawSyntaxPrinter<'a>(&'a str);

impl<'a> Debug for RawSyntaxPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct HslaPrinter(Hsla);

impl Debug for HslaPrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", IntoPrinter(&Rgba::from(self.0)))
    }
}

struct IntoPrinter<'a, D: Debug>(&'a D);

impl<'a, D: Debug> Debug for IntoPrinter<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}.into()", self.0)
    }
}

pub struct OptionPrinter<'a, T>(&'a Option<T>);

impl<'a, T: Debug> Debug for OptionPrinter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(value) => write!(f, "Some({:?})", value),
            None => write!(f, "None"),
        }
    }
}

pub struct VecPrinter<'a, T>(&'a Vec<T>);

impl<'a, T: Debug> Debug for VecPrinter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec!{:?}", &self.0)
    }
}

pub struct UserThemeFamilyPrinter(UserThemeFamily);

impl UserThemeFamilyPrinter {
    pub fn new(theme_family: UserThemeFamily) -> Self {
        Self(theme_family)
    }
}

impl Debug for UserThemeFamilyPrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserThemeFamily")
            .field("name", &IntoPrinter(&self.0.name))
            .field("author", &IntoPrinter(&self.0.author))
            .field(
                "themes",
                &VecPrinter(
                    &self
                        .0
                        .themes
                        .iter()
                        .map(|theme| UserThemePrinter(theme))
                        .collect(),
                ),
            )
            .finish()
    }
}

pub struct UserThemePrinter<'a>(&'a UserTheme);

impl<'a> Debug for UserThemePrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserTheme")
            .field("name", &IntoPrinter(&self.0.name))
            .field("appearance", &AppearancePrinter(self.0.appearance))
            .field("styles", &UserThemeStylesRefinementPrinter(&self.0.styles))
            .finish()
    }
}

pub struct AppearancePrinter(Appearance);

impl Debug for AppearancePrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Appearance::{:?}", self.0)
    }
}

pub struct UserThemeStylesRefinementPrinter<'a>(&'a UserThemeStylesRefinement);

impl<'a> Debug for UserThemeStylesRefinementPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserThemeStylesRefinement")
            .field("colors", &ThemeColorsRefinementPrinter(&self.0.colors))
            .field("status", &StatusColorsRefinementPrinter(&self.0.status))
            .field(
                "syntax",
                &OptionPrinter(
                    &self
                        .0
                        .syntax
                        .as_ref()
                        .map(|syntax| UserSyntaxThemePrinter(syntax)),
                ),
            )
            .finish()
    }
}

pub struct SystemColorsPrinter<'a>(&'a SystemColors);

impl<'a> Debug for SystemColorsPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemColors")
            .field("transparent", &HslaPrinter(self.0.transparent))
            .field(
                "mac_os_traffic_light_red",
                &HslaPrinter(self.0.mac_os_traffic_light_red),
            )
            .field(
                "mac_os_traffic_light_yellow",
                &HslaPrinter(self.0.mac_os_traffic_light_yellow),
            )
            .field(
                "mac_os_traffic_light_green",
                &HslaPrinter(self.0.mac_os_traffic_light_green),
            )
            .finish()
    }
}

pub struct ThemeColorsRefinementPrinter<'a>(&'a ThemeColorsRefinement);

impl<'a> Debug for ThemeColorsRefinementPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let theme_colors = vec![
            ("border", self.0.border),
            ("border_variant", self.0.border_variant),
            ("border_focused", self.0.border_focused),
            ("border_selected", self.0.border_selected),
            ("border_transparent", self.0.border_transparent),
            ("border_disabled", self.0.border_disabled),
            (
                "elevated_surface_background",
                self.0.elevated_surface_background,
            ),
            ("surface_background", self.0.surface_background),
            ("background", self.0.background),
            ("element_background", self.0.element_background),
            ("element_hover", self.0.element_hover),
            ("element_active", self.0.element_active),
            ("element_selected", self.0.element_selected),
            ("element_disabled", self.0.element_disabled),
            ("drop_target_background", self.0.drop_target_background),
            ("ghost_element_background", self.0.ghost_element_background),
            ("ghost_element_hover", self.0.ghost_element_hover),
            ("ghost_element_active", self.0.ghost_element_active),
            ("ghost_element_selected", self.0.ghost_element_selected),
            ("ghost_element_disabled", self.0.ghost_element_disabled),
            ("text", self.0.text),
            ("text_muted", self.0.text_muted),
            ("text_placeholder", self.0.text_placeholder),
            ("text_disabled", self.0.text_disabled),
            ("text_accent", self.0.text_accent),
            ("icon", self.0.icon),
            ("icon_muted", self.0.icon_muted),
            ("icon_disabled", self.0.icon_disabled),
            ("icon_placeholder", self.0.icon_placeholder),
            ("icon_accent", self.0.icon_accent),
            ("status_bar_background", self.0.status_bar_background),
            ("title_bar_background", self.0.title_bar_background),
            ("toolbar_background", self.0.toolbar_background),
            ("tab_bar_background", self.0.tab_bar_background),
            ("tab_inactive_background", self.0.tab_inactive_background),
            ("tab_active_background", self.0.tab_active_background),
            ("editor_background", self.0.editor_background),
            ("editor_gutter_background", self.0.editor_gutter_background),
            (
                "editor_subheader_background",
                self.0.editor_subheader_background,
            ),
            (
                "editor_active_line_background",
                self.0.editor_active_line_background,
            ),
            (
                "editor_highlighted_line_background",
                self.0.editor_highlighted_line_background,
            ),
            ("editor_line_number", self.0.editor_line_number),
            (
                "editor_active_line_number",
                self.0.editor_active_line_number,
            ),
            ("editor_invisible", self.0.editor_invisible),
            ("editor_wrap_guide", self.0.editor_wrap_guide),
            ("editor_active_wrap_guide", self.0.editor_active_wrap_guide),
            (
                "editor_document_highlight_read_background",
                self.0.editor_document_highlight_read_background,
            ),
            (
                "editor_document_highlight_write_background",
                self.0.editor_document_highlight_write_background,
            ),
            ("terminal_background", self.0.terminal_background),
            (
                "terminal_ansi_bright_black",
                self.0.terminal_ansi_bright_black,
            ),
            ("terminal_ansi_bright_red", self.0.terminal_ansi_bright_red),
            (
                "terminal_ansi_bright_green",
                self.0.terminal_ansi_bright_green,
            ),
            (
                "terminal_ansi_bright_yellow",
                self.0.terminal_ansi_bright_yellow,
            ),
            (
                "terminal_ansi_bright_blue",
                self.0.terminal_ansi_bright_blue,
            ),
            (
                "terminal_ansi_bright_magenta",
                self.0.terminal_ansi_bright_magenta,
            ),
            (
                "terminal_ansi_bright_cyan",
                self.0.terminal_ansi_bright_cyan,
            ),
            (
                "terminal_ansi_bright_white",
                self.0.terminal_ansi_bright_white,
            ),
            ("terminal_ansi_black", self.0.terminal_ansi_black),
            ("terminal_ansi_red", self.0.terminal_ansi_red),
            ("terminal_ansi_green", self.0.terminal_ansi_green),
            ("terminal_ansi_yellow", self.0.terminal_ansi_yellow),
            ("terminal_ansi_blue", self.0.terminal_ansi_blue),
            ("terminal_ansi_magenta", self.0.terminal_ansi_magenta),
            ("terminal_ansi_cyan", self.0.terminal_ansi_cyan),
            ("terminal_ansi_white", self.0.terminal_ansi_white),
        ];

        f.write_str("ThemeColorsRefinement {")?;

        for (color_name, color) in theme_colors {
            if let Some(color) = color {
                f.write_str(color_name)?;
                f.write_str(": ")?;
                f.write_str("Some(")?;
                HslaPrinter(color).fmt(f)?;
                f.write_str(")")?;
                f.write_str(",")?;
            }
        }

        f.write_str("..Default::default()")?;
        f.write_str("}")
    }
}

pub struct StatusColorsRefinementPrinter<'a>(&'a StatusColorsRefinement);

impl<'a> Debug for StatusColorsRefinementPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_colors = vec![
            ("conflict", self.0.conflict),
            ("created", self.0.created),
            ("deleted", self.0.deleted),
            ("error", self.0.error),
            ("hidden", self.0.hidden),
            ("ignored", self.0.ignored),
            ("info", self.0.info),
            ("modified", self.0.modified),
            ("renamed", self.0.renamed),
            ("success", self.0.success),
            ("warning", self.0.warning),
        ];

        f.write_str("StatusColorsRefinement {")?;

        for (color_name, color) in status_colors {
            if let Some(color) = color {
                f.write_str(color_name)?;
                f.write_str(": ")?;
                f.write_str("Some(")?;
                HslaPrinter(color).fmt(f)?;
                f.write_str(")")?;
                f.write_str(",")?;
            }
        }

        f.write_str("..Default::default()")?;
        f.write_str("}")
    }
}

pub struct PlayerColorsPrinter<'a>(&'a PlayerColors);

impl<'a> Debug for PlayerColorsPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PlayerColors")
            .field(&VecPrinter(
                &self
                    .0
                     .0
                    .iter()
                    .map(|player_color| PlayerColorPrinter(player_color))
                    .collect(),
            ))
            .finish()
    }
}

pub struct PlayerColorPrinter<'a>(&'a PlayerColor);

impl<'a> Debug for PlayerColorPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayerColor")
            .field("cursor", &HslaPrinter(self.0.cursor))
            .field("background", &HslaPrinter(self.0.background))
            .field("selection", &HslaPrinter(self.0.selection))
            .finish()
    }
}

pub struct UserSyntaxThemePrinter<'a>(&'a UserSyntaxTheme);

impl<'a> Debug for UserSyntaxThemePrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserSyntaxTheme")
            .field(
                "highlights",
                &VecPrinter(
                    &self
                        .0
                        .highlights
                        .iter()
                        .map(|(token, highlight)| {
                            (IntoPrinter(token), UserHighlightStylePrinter(&highlight))
                        })
                        .collect(),
                ),
            )
            .finish()
    }
}

pub struct UserHighlightStylePrinter<'a>(&'a UserHighlightStyle);

impl<'a> Debug for UserHighlightStylePrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UserHighlightStyle {")?;

        if let Some(color) = self.0.color {
            f.write_str("color")?;
            f.write_str(": ")?;
            f.write_str("Some(")?;
            HslaPrinter(color).fmt(f)?;
            f.write_str(")")?;
            f.write_str(",")?;
        }

        if let Some(font_style) = self.0.font_style {
            f.write_str("font_style")?;
            f.write_str(": ")?;
            f.write_str("Some(")?;
            write!(f, "UserFontStyle::{:?}", font_style)?;
            f.write_str(")")?;
            f.write_str(",")?;
        }

        if let Some(font_weight) = self.0.font_weight.as_ref() {
            f.write_str("font_weight")?;
            f.write_str(": ")?;
            f.write_str("Some(")?;
            write!(f, "UserFontWeight({:?})", font_weight.0)?;
            f.write_str(")")?;
            f.write_str(",")?;
        }

        f.write_str("..Default::default()")?;
        f.write_str("}")
    }
}