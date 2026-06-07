use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    system::{ActiveProcess, Trend},
    theme::{self, border_style, dim_style, title_style, ACTIVITY_DOWN, ACTIVITY_STABLE, ACTIVITY_UP},
};

pub fn draw(f: &mut Frame, area: Rect, active: &[ActiveProcess]) {
    let block = Block::default()
        .title(Span::styled(" ACTIVE ", title_style()))
        .borders(Borders::ALL)
        .border_style(border_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    if active.is_empty() {
        let msg = Paragraph::new(Line::from(vec![Span::styled(
            "No rapid changes",
            dim_style(),
        )]));
        f.render_widget(msg, inner);
        return;
    }

    let row_height = 2u16;
    let constraints: Vec<Constraint> = active
        .iter()
        .take((inner.height / row_height) as usize)
        .map(|_| Constraint::Length(row_height))
        .collect();

    if constraints.is_empty() {
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, proc) in active.iter().enumerate() {
        if i >= rows.len() { break; }
        draw_entry(f, rows[i], proc);
    }
}

fn draw_entry(f: &mut Frame, area: Rect, proc: &ActiveProcess) {
    let (arrow, color) = match proc.trend {
        Trend::Rising  => ("▲", ACTIVITY_UP),
        Trend::Falling => ("▼", ACTIVITY_DOWN),
        Trend::Stable  => ("─", ACTIVITY_STABLE),
    };

    let delta_sign = if proc.delta >= 0.0 { "+" } else { "" };

    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{arrow} "), Style::default().fg(color)),
            Span::styled(truncate_name(&proc.name, 16), Style::default().fg(theme::TEXT)),
        ]),
        Line::from(vec![
            Span::styled("  CPU: ", dim_style()),
            Span::styled(format!("{:.1}%", proc.cpu_percent), Style::default().fg(color)),
            Span::styled(
                format!(" ({delta_sign}{:.1}%)", proc.delta),
                Style::default().fg(delta_color(proc.delta)),
            ),
        ]),
    ];

    f.render_widget(Paragraph::new(lines), area);
}

fn delta_color(delta: f32) -> Color {
    if delta > 0.0 { ACTIVITY_UP }
    else if delta < 0.0 { ACTIVITY_DOWN }
    else { ACTIVITY_STABLE }
}

fn truncate_name(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut t: String = s.chars().take(max - 1).collect();
        t.push('…');
        t
    }
}
