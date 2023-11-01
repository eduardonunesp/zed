use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn gruvbox_light() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Gruvbox Light".into(),
            is_light: true,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0xc8b899ff).into(),
        border_variant: rgba(0xc8b899ff).into(),
        border_focused: rgba(0xadc5ccff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0xd9c8a4ff).into(),
        surface: rgba(0xecddb4ff).into(),
        background: rgba(0xd9c8a4ff).into(),
        filled_element: rgba(0xd9c8a4ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0xd2dee2ff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0xd2dee2ff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0x282828ff).into(),
        text_muted: rgba(0x5f5650ff).into(),
        text_placeholder: rgba(0x9d0308ff).into(),
        text_disabled: rgba(0x897b6eff).into(),
        text_accent: rgba(0x0b6678ff).into(),
        icon_muted: rgba(0x5f5650ff).into(),
        syntax: SyntaxTheme {
            highlights: vec![
                ("number".into(), rgba(0x8f3e71ff).into()),
                ("link_text".into(), rgba(0x427b58ff).into()),
                ("string.special".into(), rgba(0x8f3e71ff).into()),
                ("string.special.symbol".into(), rgba(0x427b58ff).into()),
                ("function".into(), rgba(0x79740eff).into()),
                ("title".into(), rgba(0x79740eff).into()),
                ("emphasis".into(), rgba(0x0b6678ff).into()),
                ("punctuation".into(), rgba(0x3c3836ff).into()),
                ("string.escape".into(), rgba(0x5d544eff).into()),
                ("type".into(), rgba(0xb57613ff).into()),
                ("string".into(), rgba(0x79740eff).into()),
                ("keyword".into(), rgba(0x9d0006ff).into()),
                ("tag".into(), rgba(0x427b58ff).into()),
                ("primary".into(), rgba(0x282828ff).into()),
                ("link_uri".into(), rgba(0x8f3e71ff).into()),
                ("comment.doc".into(), rgba(0x5d544eff).into()),
                ("boolean".into(), rgba(0x8f3e71ff).into()),
                ("embedded".into(), rgba(0x427b58ff).into()),
                ("hint".into(), rgba(0x677562ff).into()),
                ("emphasis.strong".into(), rgba(0x0b6678ff).into()),
                ("operator".into(), rgba(0x427b58ff).into()),
                ("label".into(), rgba(0x0b6678ff).into()),
                ("comment".into(), rgba(0x7c6f64ff).into()),
                ("function.builtin".into(), rgba(0x9d0006ff).into()),
                ("punctuation.bracket".into(), rgba(0x665c54ff).into()),
                ("text.literal".into(), rgba(0x066578ff).into()),
                ("string.regex".into(), rgba(0xaf3a02ff).into()),
                ("property".into(), rgba(0x282828ff).into()),
                ("attribute".into(), rgba(0x0b6678ff).into()),
                ("punctuation.delimiter".into(), rgba(0x413d3aff).into()),
                ("constructor".into(), rgba(0x0b6678ff).into()),
                ("variable".into(), rgba(0x066578ff).into()),
                ("constant".into(), rgba(0xb57613ff).into()),
                ("preproc".into(), rgba(0x282828ff).into()),
                ("punctuation.special".into(), rgba(0x413d3aff).into()),
                ("punctuation.list_marker".into(), rgba(0x282828ff).into()),
                ("variant".into(), rgba(0x0b6678ff).into()),
                ("predictive".into(), rgba(0x7c9780ff).into()),
                ("enum".into(), rgba(0xaf3a02ff).into()),
            ],
        },
        status_bar: rgba(0xd9c8a4ff).into(),
        title_bar: rgba(0xd9c8a4ff).into(),
        toolbar: rgba(0xfbf1c7ff).into(),
        tab_bar: rgba(0xecddb4ff).into(),
        editor: rgba(0xfbf1c7ff).into(),
        editor_subheader: rgba(0xecddb4ff).into(),
        editor_active_line: rgba(0xecddb4ff).into(),
        terminal: rgba(0xfbf1c7ff).into(),
        image_fallback_background: rgba(0xd9c8a4ff).into(),
        git_created: rgba(0x797410ff).into(),
        git_modified: rgba(0x0b6678ff).into(),
        git_deleted: rgba(0x9d0308ff).into(),
        git_conflict: rgba(0xb57615ff).into(),
        git_ignored: rgba(0x897b6eff).into(),
        git_renamed: rgba(0xb57615ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x0b6678ff).into(),
                selection: rgba(0x0b66783d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x797410ff).into(),
                selection: rgba(0x7974103d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x7c6f64ff).into(),
                selection: rgba(0x7c6f643d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xaf3a04ff).into(),
                selection: rgba(0xaf3a043d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x8f3f70ff).into(),
                selection: rgba(0x8f3f703d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x437b59ff).into(),
                selection: rgba(0x437b593d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9d0308ff).into(),
                selection: rgba(0x9d03083d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xb57615ff).into(),
                selection: rgba(0xb576153d).into(),
            },
        ],
    }
}