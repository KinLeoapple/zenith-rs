use crate::theme::color::{Color, Text, Theme};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Default {
    Theme
}

impl Default {
    pub fn bg(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            "bg-gray-900"
        } else {
            "bg-gray-90"
        }
    }

    pub fn text_color(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            Text::Primary.dark()
        } else {
            Text::Primary.light()
        }
    }

    pub fn button_bg_color(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            Color::_500.primary()
        } else {
            Color::_300.primary()
        }
    }

    pub fn button_hover_bg_color(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            Color::_600.primary()
        } else {
            Color::_400.primary()
        }
    }

    pub fn list_items_bg_color(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            Color::_700.neutral()
        } else {
            Color::_300.neutral()
        }
    }

    pub fn list_items_hover_bg_color(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            Color::_600.neutral()
        } else {
            Color::_400.neutral()
        }
    }

    pub fn avatar_bg_color(&self, theme: &str) -> &str {
        self.button_bg_color(theme)
    }

    pub fn avatar_icon_color(&self, theme: &str) -> &str {
        self.button_hover_bg_color(theme)
    }

    pub fn textarea_bg_color(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            Color::_900.primary()
        } else {
            Color::_100.primary()
        }
    }

    pub fn textarea_border_color(&self, theme: &str) -> &str {
        self.button_bg_color(theme)
    }

    pub fn textarea_border_focus_color(&self, theme: &str) -> &str {
        if theme == Theme::Dark.theme() {
            Color::_200.primary()
        } else {
            Color::_800.primary()
        }
    }

    pub fn input_bg_color(&self, theme: &str) -> &str {
        self.textarea_bg_color(theme)
    }

    pub fn input_border_color(&self, theme: &str) -> &str {
        self.textarea_border_color(theme)
    }

    pub fn input_border_focus_color(&self, theme: &str) -> &str {
        self.textarea_border_focus_color(theme)
    }
}