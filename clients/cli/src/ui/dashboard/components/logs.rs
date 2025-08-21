//! Dashboard logs panel component

use super::super::state::DashboardState;
use super::super::utils::{clean_http_error_message, format_compact_timestamp};
use super::theme;
use crate::events::EventType;
use ratatui::Frame;
use ratatui::prelude::Style;
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render_logs_panel(f: &mut Frame, area: ratatui::layout::Rect, state: &DashboardState) {
    let max_logs = (area.height.saturating_sub(2)) as usize;
    let log_count = if max_logs > 0 { max_logs } else { 1 };

    let log_lines: Vec<Line> = state
        .activity_logs
        .iter()
        .filter(|event| event.should_display())
        .rev()
        .take(log_count)
        .map(|event| {
            let (status_icon, msg_style) = match event.event_type {
                EventType::Success => ("✔", Style::default().fg(theme::COLOR_SUCCESS)),
                EventType::Error => ("✖", Style::default().fg(theme::COLOR_ERROR)),
                EventType::Waiting => ("…", Style::default().fg(theme::COLOR_DIM)),
                _ => ("›", theme::text_style()),
            };

            let compact_time = format_compact_timestamp(&event.timestamp);
            let cleaned_msg = clean_http_error_message(&event.msg);

            Line::from(vec![
                Span::styled(format!("{} ", compact_time), theme::dim_text_style()),
                Span::styled(format!("{} ", status_icon), msg_style),
                Span::styled(cleaned_msg, msg_style),
            ])
        })
        .collect();

    let logs_block = Block::default()
        .title(" LIVE LOG STREAM ")
        .title_style(theme::block_title_style())
        .borders(Borders::ALL)
        .border_set(symbols::border::QUADRANT_OUTSIDE) // Sci-fi border
        .border_style(theme::border_style());

    let log_widget = Paragraph::new(log_lines)
        .block(logs_block)
        .wrap(Wrap { trim: true });

    f.render_widget(log_widget, area);
}
