//! Dashboard info panel component
//!
//! Renders system information panel as a data grid.

use super::super::state::DashboardState;
use super::theme;
use crate::environment::Environment;
use ratatui::prelude::{Constraint, Style};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, Table};
use ratatui::Frame;

/// Renders the info panel as a structured table.
pub fn render_info_panel(f: &mut Frame, area: ratatui::layout::Rect, state: &DashboardState) {
    let (env_str, env_color) = match state.environment {
        Environment::Production => ("Production", theme::COLOR_SUCCESS),
        Environment::Custom { .. } => ("Custom", theme::COLOR_WARNING),
    };

    let uptime = state.start_time.elapsed();
    let uptime_string = format!(
        "{}h {}m {}s",
        uptime.as_secs() / 3600,
        (uptime.as_secs() % 3600) / 60,
        uptime.as_secs() % 60
    );

    let rows = vec![
        Row::new(vec![
            Cell::from("Node ID"),
            Cell::from(
                if let Some(id) = state.node_id {
                    id.to_string()
                } else {
                    "Disconnected".to_string()
                },
            )
            .style(theme::text_style()),
        ]),
        Row::new(vec![
            Cell::from("Network"),
            Cell::from(env_str).style(Style::default().fg(env_color)),
        ]),
        Row::new(vec![
            Cell::from("Version"),
            Cell::from(env!("CARGO_PKG_VERSION")).style(theme::text_style()),
        ]),
        Row::new(vec![
            Cell::from("Uptime"),
            Cell::from(uptime_string).style(theme::text_style()),
        ]),
        Row::new(vec![
            Cell::from("Threads"),
            Cell::from(state.num_threads.to_string()).style(theme::text_style()),
        ]),
    ];

    let table = Table::new(rows, vec![Constraint::Length(10), Constraint::Min(0)])
        .block(
            Block::default()
                .title(" SYSTEM INFO ")
                .title_style(theme::block_title_style())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(theme::border_style()),
        )
        .header(
            Row::new(vec!["Metric", "Value"])
                .style(theme::title_style())
                .bottom_margin(1),
        )
        .column_spacing(1);

    f.render_widget(table, area);
}