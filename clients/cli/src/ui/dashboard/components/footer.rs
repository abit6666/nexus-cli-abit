//! Dashboard footer component
//!
//! Renders an animated footer ticker.

use super::super::state::DashboardState;
use super::theme;
use ratatui::layout::Alignment;
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render_footer(f: &mut Frame, area: ratatui::layout::Rect, state: &DashboardState) {
    let base_text = " [Q] QUIT | NEXUS NETWORK PROVER | ALL SYSTEMS OPERATIONAL ";
    let full_text = base_text.repeat(3); // Repeat to ensure it can scroll

    // Animate the scroll position based on the tick
    let text_len = full_text.len();
    let start_index = (state.tick as usize / 2) % base_text.len();

    let scrolling_text = if text_len > start_index {
        &full_text[start_index..]
    } else {
        ""
    };

    let footer_text = Paragraph::new(scrolling_text)
        .alignment(Alignment::Left)
        .style(
            Style::default()
                .fg(theme::PRIMARY_WHITE)
                .bg(theme::ACCENT_BLUE)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(footer_text, area);
}