//! Dashboard main renderer

use super::components::{footer, header, info_panel, logs, metrics, theme};
use super::state::DashboardState;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Style;
use ratatui::widgets::{Block, BorderType};
use ratatui::Frame;

pub fn render_dashboard(f: &mut Frame, state: &DashboardState) {
    // Set the main background color
    f.render_widget(
        Block::default().style(Style::default().bg(theme::SECONDARY_DARK)),
        f.area(),
    );

    // Main layout: Header, a three-column content area, and a Footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Slimmer Header
            Constraint::Min(0),    // Main content area
            Constraint::Length(1), // Footer Ticker
        ])
        .split(f.area());

    header::render_header(f, main_chunks[0], state);
    footer::render_footer(f, main_chunks[2], state);

    // A block to create a frame around the main content
    let content_frame = Block::default()
        .title(" NEXUS PROVER DASHBOARD ")
        .title_style(theme::title_style())
        .border_type(BorderType::Double)
        .border_style(theme::border_style());
    f.render_widget(content_frame, main_chunks[1]);

    // Three-column layout inside the main content frame
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(25), // Left Column
            Constraint::Percentage(50), // Center Column (Logs)
            Constraint::Percentage(25), // Right Column
        ])
        .split(main_chunks[1]);

    // --- Left Column: System Info & Metrics ---
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)])
        .split(content_chunks[0]);
    info_panel::render_info_panel(f, left_chunks[0], state);
    metrics::render_system_charts(f, left_chunks[1], state);

    // --- Center Column: Activity Logs ---
    logs::render_logs_panel(f, content_chunks[1], state);

    // --- Right Column: zkVM Stats ---
    metrics::render_zkvm_metrics(f, content_chunks[2], state);
}