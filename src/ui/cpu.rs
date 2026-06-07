use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::{
    system::CpuData,
    theme::{border_style, dim_style, gauge_color, title_style, TEXT, GAUGE_BG},
};

pub fn draw(f: &mut Frame, area: Rect, cpu: &CpuData) {
    let block = Block::default()
        .title(Span::styled(format!(" CPU ─ {} ", cpu.brand), title_style()))
        .borders(Borders::ALL)
        .border_style(border_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // sparkline — thin, just 1 line
            Constraint::Length(1), // overall CPU bar
            Constraint::Min(1),    // per-core rows
            Constraint::Length(1), // load average footer
        ])
        .split(inner);

    draw_sparkline(f, rows[0], cpu);
    draw_overall_bar(f, rows[1], cpu);
    draw_core_grid(f, rows[2], cpu);
    draw_load_avg(f, rows[3], cpu);
}

fn draw_sparkline(f: &mut Frame, area: Rect, cpu: &CpuData) {
    let data: Vec<u64> = cpu.history.iter().map(|&v| v as u64).collect();
    let color = gauge_color(cpu.total_usage as f64 / 100.0);
    let spark = Sparkline::default()
        .data(&data)
        .max(100)
        .style(Style::default().fg(color));
    f.render_widget(spark, area);
}

fn draw_overall_bar(f: &mut Frame, area: Rect, cpu: &CpuData) {
    if area.width < 12 {
        return;
    }

    let pct = cpu.total_usage.round() as u16;
    let color = gauge_color(cpu.total_usage as f64 / 100.0);

    let label_w  = 4usize;
    let pct_str  = format!(" {pct}%");
    let bar_w    = (area.width as usize).saturating_sub(label_w + pct_str.len());

    let filled = (bar_w * pct as usize / 100).min(bar_w);
    let empty  = bar_w - filled;

    let line = Line::from(vec![
        Span::styled("CPU ", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        Span::styled("█".repeat(filled), Style::default().fg(color)),
        Span::styled("░".repeat(empty),  Style::default().fg(GAUGE_BG)),
        Span::styled(pct_str,            Style::default().fg(color).add_modifier(Modifier::BOLD)),
    ]);

    f.render_widget(Paragraph::new(line), area);
}

fn draw_core_grid(f: &mut Frame, area: Rect, cpu: &CpuData) {
    if cpu.cores.is_empty() || area.height == 0 || area.width < 10 {
        return;
    }

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let half = cpu.cores.len().div_ceil(2);

    for col_idx in 0..2usize {
        let col_area = cols[col_idx];
        let start    = col_idx * half;
        let end      = (start + half).min(cpu.cores.len());
        let slice    = &cpu.cores[start..end];

        let row_constraints: Vec<Constraint> = slice
            .iter()
            .map(|_| Constraint::Length(1))
            .collect();

        if row_constraints.is_empty() {
            continue;
        }

        let row_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(col_area);

        for (i, core) in slice.iter().enumerate() {
            draw_core_row(f, row_rects[i], &core.name, core.usage, col_area.width);
        }
    }
}

fn draw_core_row(f: &mut Frame, area: Rect, name: &str, usage: f32, col_width: u16) {
    if col_width < 10 {
        return;
    }

    let pct      = usage.round() as u16;
    let color    = gauge_color(usage as f64 / 100.0);
    let pct_str  = format!("{pct:>3}%");
    let label_w  = 4usize;
    let pct_w    = pct_str.len() + 1;
    let bar_w    = (col_width as usize).saturating_sub(label_w + pct_w).max(1);

    let filled = (bar_w * pct as usize / 100).min(bar_w);
    let empty  = bar_w - filled;

    let line = Line::from(vec![
        Span::styled(format!("{name:<3} "),    Style::default().fg(TEXT)),
        Span::styled("█".repeat(filled),        Style::default().fg(color)),
        Span::styled("░".repeat(empty),         Style::default().fg(GAUGE_BG)),
        Span::styled(format!(" {pct_str}"),     Style::default().fg(color)),
    ]);

    f.render_widget(Paragraph::new(line), area);
}

fn draw_load_avg(f: &mut Frame, area: Rect, cpu: &CpuData) {
    let (la1, la5, la15) = cpu.load_avg;
    let text = if la1 == 0.0 && la5 == 0.0 && la15 == 0.0 {
        format!("Cores: {}", cpu.cores.len())
    } else {
        format!("Load avg: {la1:.2}  {la5:.2}  {la15:.2}")
    };

    let padded = format!("{text:>width$}", width = area.width as usize);
    f.render_widget(Paragraph::new(Span::styled(padded, dim_style())), area);
}
