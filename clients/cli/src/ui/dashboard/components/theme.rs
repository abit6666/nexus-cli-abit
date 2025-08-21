//! theme.rs
//!
//! Defines the color palette and styles for the branded sci-fi UI.

use ratatui::prelude::{Color, Modifier, Style};

// Your Brand Colors
pub const PRIMARY_BLACK: Color = Color::Rgb(0, 0, 0);
pub const PRIMARY_WHITE: Color = Color::Rgb(255, 255, 255);
pub const SECONDARY_DARK: Color = Color::Rgb(27, 27, 30);
pub const SECONDARY_LIGHT: Color = Color::Rgb(248, 248, 248);
pub const ACCENT_BLUE: Color = Color::Rgb(0, 149, 255);

// UI Element Colors
pub const COLOR_SUCCESS: Color = Color::LightGreen;
pub const COLOR_WARNING: Color = Color::LightYellow;
pub const COLOR_ERROR: Color = Color::LightRed;
pub const COLOR_DIM: Color = Color::Gray;

// --- STYLES ---

pub fn title_style() -> Style {
    Style::default()
        .fg(ACCENT_BLUE)
        .add_modifier(Modifier::BOLD)
}

pub fn block_title_style() -> Style {
    Style::default().fg(PRIMARY_WHITE)
}

pub fn border_style() -> Style {
    Style::default().fg(ACCENT_BLUE)
}

pub fn text_style() -> Style {
    Style::default().fg(SECONDARY_LIGHT)
}

pub fn dim_text_style() -> Style {
    Style::default().fg(COLOR_DIM)
}
