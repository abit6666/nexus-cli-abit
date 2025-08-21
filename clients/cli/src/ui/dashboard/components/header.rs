//! Dashboard header component

use super::super::state::DashboardState;
use super::theme;
use crate::events::ProverState;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_header(f: &mut Frame, area: ratatui::layout::Rect, state: &DashboardState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // --- Title ---
    let version = env!("CARGO_PKG_VERSION");
    let title_text = format!("NEXUS PROVER // v{}", version);
    let title = Paragraph::new(title_text)
        .style(theme::title_style())
        .alignment(Alignment::Left);
    f.render_widget(title, chunks[0]);

    // --- Status ---
    let (status_text, status_style) = match state.current_prover_state() {
        ProverState::Proving => (
            "STATUS: PROOF GENERATION ONLINE",
            Style::default().fg(theme::COLOR_SUCCESS),
        ),
        ProverState::Waiting => (
            "STATUS: AWAITING TASK",
            Style::default().fg(theme::ACCENT_BLUE),
        ),
    };

    let status = Paragraph::new(status_text)
        .style(status_style.add_modifier(Modifier::BOLD))
        .alignment(Alignment::Right);
    f.render_widget(status, chunks[1]);

    // --- Bottom Border ---
    let bottom_border = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(theme::border_style());
    f.render_widget(bottom_border, area);
}
