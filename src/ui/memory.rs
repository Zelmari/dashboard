use human_bytes::human_bytes;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::{
    system::MemData,
    theme::{self, border_style, dim_style, gauge_color, title_style},
};

pub fn draw(f: &mut Frame, area: Rect, mem: &MemData) {
    let block = Block::default()
        .title(Span::styled(" MEM ", title_style()))
        .borders(Borders::ALL)
        .border_style(border_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // stat lines
            Constraint::Length(2), // RAM gauge
            Constraint::Length(2), // Swap gauge
        ])
        .split(inner);

    draw_stats(f, rows[0], mem);
    draw_ram_gauge(f, rows[1], mem);
    draw_swap_gauge(f, rows[2], mem);
}

fn draw_stats(f: &mut Frame, area: Rect, mem: &MemData) {
    let used_pct = if mem.total_bytes > 0 {
        mem.used_bytes as f64 / mem.total_bytes as f64 * 100.0
    } else {
        0.0
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("Total: ", dim_style()),
            Span::styled(human_bytes(mem.total_bytes as f64), Style::default().fg(theme::TEXT)),
            Span::styled("  Used: ", dim_style()),
            Span::styled(
                format!("{} ({:.0}%)", human_bytes(mem.used_bytes as f64), used_pct),
                Style::default().fg(gauge_color(used_pct / 100.0)),
            ),
        ]),
        Line::from(vec![
            Span::styled("Avail: ", dim_style()),
            Span::styled(human_bytes(mem.available_bytes as f64), Style::default().fg(theme::TEXT)),
        ]),
        Line::from(vec![
            Span::styled("Swap:  ", dim_style()),
            Span::styled(
                format!("{} / {}", human_bytes(mem.swap_used_bytes as f64), human_bytes(mem.swap_total_bytes as f64)),
                Style::default().fg(theme::TEXT_DIM),
            ),
        ]),
    ];

    f.render_widget(Paragraph::new(lines), area);
}

fn draw_ram_gauge(f: &mut Frame, area: Rect, mem: &MemData) {
    let pct = if mem.total_bytes > 0 {
        (mem.used_bytes as f64 / mem.total_bytes as f64 * 100.0) as u16
    } else { 0 };
    let color = gauge_color(pct as f64 / 100.0);

    let cols = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(4)])
        .split(area);

    f.render_widget(Paragraph::new(Span::styled("RAM  ", dim_style())), cols[0]);
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(color).bg(theme::GAUGE_BG))
            .percent(pct)
            .label(format!("{pct}%")),
        cols[1],
    );
}

fn draw_swap_gauge(f: &mut Frame, area: Rect, mem: &MemData) {
    let pct = if mem.swap_total_bytes > 0 {
        (mem.swap_used_bytes as f64 / mem.swap_total_bytes as f64 * 100.0) as u16
    } else { 0 };
    let color = gauge_color(pct as f64 / 100.0);

    let cols = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(4)])
        .split(area);

    f.render_widget(Paragraph::new(Span::styled("SWAP ", dim_style())), cols[0]);
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(color).bg(theme::GAUGE_BG))
            .percent(pct)
            .label(format!("{pct}%")),
        cols[1],
    );
}
