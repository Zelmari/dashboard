use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::{
    system::CpuData,
    theme::{border_style, dim_style, gauge_color, title_style, GAUGE_BG, TEXT},
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
            Constraint::Length(1), // sparkline
            Constraint::Length(1), // overall bar
            Constraint::Min(1),    // core grid
            Constraint::Length(1), // load avg
        ])
        .split(inner);

    draw_sparkline(f, rows[0], cpu);
    draw_overall_bar(f, rows[1], cpu);
    draw_core_grid(f, rows[2], cpu);
    draw_load_avg(f, rows[3], cpu);
}

fn draw_sparkline(f: &mut Frame, area: Rect, cpu: &CpuData) {
    let data: Vec<u64> = cpu.history.iter().map(|&v| v as u64).collect();
    let spark = Sparkline::default()
        .data(&data)
        .max(100)
        .style(Style::default().fg(gauge_color(cpu.total_usage as f64 / 100.0)));
    f.render_widget(spark, area);
}

fn draw_overall_bar(f: &mut Frame, area: Rect, cpu: &CpuData) {
    if area.width < 12 { return; }
    let pct     = cpu.total_usage.round() as u16;
    let color   = gauge_color(cpu.total_usage as f64 / 100.0);
    let pct_str = format!(" {:>3}%", pct);
    let bar_w   = (area.width as usize).saturating_sub(4 + pct_str.len());
    let filled  = (bar_w * pct as usize / 100).min(bar_w);
    let empty   = bar_w - filled;

    f.render_widget(Paragraph::new(Line::from(vec![
        Span::styled("CPU ", Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        Span::styled("█".repeat(filled), Style::default().fg(color)),
        Span::styled("░".repeat(empty),  Style::default().fg(GAUGE_BG)),
        Span::styled(pct_str,            Style::default().fg(color).add_modifier(Modifier::BOLD)),
    ])), area);
}

fn draw_core_grid(f: &mut Frame, area: Rect, cpu: &CpuData) {
    if cpu.cores.is_empty() || area.height == 0 || area.width < 20 { return; }

    // Direct Rect arithmetic — avoids Layout rounding errors that caused
    // the two-column misalignment in the previous version.
    let half_w     = area.width / 2;
    let left_area  = Rect { x: area.x, y: area.y,
                            width: half_w.saturating_sub(1), height: area.height };
    let right_area = Rect { x: area.x + half_w, y: area.y,
                            width: area.width - half_w, height: area.height };

    let half = cpu.cores.len().div_ceil(2);
    render_core_column(f, left_area,  &cpu.cores[..half.min(cpu.cores.len())]);
    render_core_column(f, right_area, &cpu.cores[half.min(cpu.cores.len())..]);
}

fn render_core_column(f: &mut Frame, area: Rect, cores: &[crate::system::CoreInfo]) {
    if cores.is_empty() || area.height == 0 { return; }

    let constraints: Vec<Constraint> = cores.iter()
        .map(|_| Constraint::Length(1)).collect();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, core) in cores.iter().enumerate() {
        if i >= rows.len() { break; }
        render_core_row(f, rows[i], &core.name, core.usage);
    }
}

fn render_core_row(f: &mut Frame, area: Rect, name: &str, usage: f32) {
    if area.width < 8 { return; }
    let pct     = usage.round() as u16;
    let color   = gauge_color(usage as f64 / 100.0);
    let pct_str = format!(" {:>3}%", pct);
    let label_w = 4usize;
    let bar_w   = (area.width as usize).saturating_sub(label_w + pct_str.len()).max(1);
    let filled  = (bar_w * pct as usize / 100).min(bar_w);
    let empty   = bar_w - filled;

    f.render_widget(Paragraph::new(Line::from(vec![
        Span::styled(format!("{:<3} ", name), Style::default().fg(TEXT)),
        Span::styled("█".repeat(filled),       Style::default().fg(color)),
        Span::styled("░".repeat(empty),        Style::default().fg(GAUGE_BG)),
        Span::styled(pct_str,                  Style::default().fg(color)),
    ])), area);
}

fn draw_load_avg(f: &mut Frame, area: Rect, cpu: &CpuData) {
    let (la1, la5, la15) = cpu.load_avg;
    let text = if la1 == 0.0 && la5 == 0.0 {
        format!("Cores: {}", cpu.cores.len())
    } else {
        format!("Load avg: {la1:.2}  {la5:.2}  {la15:.2}")
    };
    let padded = format!("{text:>width$}", width = area.width as usize);
    f.render_widget(Paragraph::new(Span::styled(padded, dim_style())), area);
}
