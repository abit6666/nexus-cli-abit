//! Dashboard metrics components
//!
//! Renders system and zkVM metrics

use super::super::state::DashboardState;
use super::theme;
use ratatui::prelude::{Alignment, Constraint, Direction, Layout, Modifier, Style};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, Paragraph,
};
use ratatui::Frame;

/// Renders the system metric charts (CPU and RAM).
pub fn render_system_charts(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    state: &DashboardState,
) {
    let metrics = &state.system_metrics;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // --- CPU Chart ---
    let cpu_data: Vec<(f64, f64)> = state
        .cpu_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v as f64))
        .collect();

    let cpu_datasets = vec![Dataset::default()
        .name("CPU %")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(metrics.cpu_color()))
        .data(&cpu_data)];

    let cpu_chart = Chart::new(cpu_datasets)
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::raw("CPU Usage ("),
                    Span::styled(
                        format!("{:.1}%", metrics.cpu_percent),
                        Style::default()
                            .fg(metrics.cpu_color())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(")"),
                ]))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(theme::border_style())
                .border_type(BorderType::Rounded),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(theme::COLOR_DIM))
                .bounds([0.0, 60.0]),
        )
        .y_axis(
            Axis::default()
                .title("Usage %")
                .style(Style::default().fg(theme::COLOR_DIM))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::from("0"),
                    Span::from("50"),
                    Span::from("100"),
                ]),
        );
    f.render_widget(cpu_chart, chunks[0]);

    // --- RAM Chart ---
    let ram_data: Vec<(f64, f64)> = state
        .ram_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v as f64))
        .collect();
    let ram_datasets = vec![Dataset::default()
        .name("RAM %")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(metrics.ram_color()))
        .data(&ram_data)];

    let ram_chart = Chart::new(ram_datasets)
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::raw("RAM Usage ("),
                    Span::styled(
                        metrics.format_ram(),
                        Style::default()
                            .fg(metrics.ram_color())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" / {:.1}GB)", state.total_ram_gb)),
                ]))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(theme::border_style())
                .border_type(BorderType::Rounded),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(theme::COLOR_DIM))
                .bounds([0.0, 60.0]),
        )
        .y_axis(
            Axis::default()
                .title("Usage %")
                .style(Style::default().fg(theme::COLOR_DIM))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::from("0"),
                    Span::from("50"),
                    Span::from("100"),
                ]),
        );
    f.render_widget(ram_chart, chunks[1]);
}

/// Renders the zkVM statistics panel with gauges.
pub fn render_zkvm_metrics(f: &mut Frame, area: ratatui::layout::Rect, state: &DashboardState) {
    let metrics = &state.zkvm_metrics;

    let zkvm_block = Block::default()
        .title(" zkVM Stats ")
        .title_style(theme::block_title_style())
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(theme::ACCENT_BLUE));

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(zkvm_block.inner(area));

    f.render_widget(zkvm_block, area);

    let info_text = vec![
        Line::from(vec![
            Span::styled("Tasks Fetched:  ", theme::dim_text_style()),
            Span::styled(
                format!("{}", metrics.tasks_fetched),
                theme::text_style().add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Difficulty Req: ", theme::dim_text_style()),
            Span::styled(
                "Large",
                Style::default()
                    .fg(theme::ACCENT_BLUE)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Performance:    ", theme::dim_text_style()),
            Span::styled(
                format!("{:.2} GFLOP/s", state.system_metrics.gflops),
                theme::text_style().add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    f.render_widget(Paragraph::new(info_text), inner_chunks[0]);

    let completed_percent = if metrics.tasks_fetched > 0 {
        (metrics.tasks_submitted as f64 / metrics.tasks_fetched as f64 * 100.0) as u16
    } else {
        0
    };
    let completed_gauge = Gauge::default()
        .block(Block::default().title("Tasks Completed"))
        .gauge_style(
            Style::default()
                .fg(theme::COLOR_SUCCESS)
                .bg(theme::PRIMARY_BLACK),
        )
        .percent(completed_percent)
        .label(format!("{} / {}", metrics.tasks_submitted, metrics.tasks_fetched));
    f.render_widget(completed_gauge, inner_chunks[2]);

    let success_gauge = Gauge::default()
        .block(Block::default().title("Success Rate"))
        .gauge_style(
            Style::default()
                .fg(metrics.success_rate_color())
                .bg(theme::PRIMARY_BLACK),
        )
        .percent(metrics.success_rate() as u16)
        .label(format!("{:.1}%", metrics.success_rate()));
    f.render_widget(success_gauge, inner_chunks[3]);
}