use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};

use crate::{
    system::CpuData,
    theme::{self, border_style, dim_style, gauge_color, header_style, title_style},
};

pub fn draw(f: &mut Frame, area: Rect, cpu: &CpuData) {
    let block = Block::default()
        .title(Span::styled(
            format!(" CPU ─ {} ", cpu.brand),
            title_style(),
        ))
        .borders(Borders::ALL)
        .border_style(border_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // sparkline
            Constraint::Length(2), // overall gauge + load avg
            Constraint::Min(1),    // per-core grid (takes whatever remains)
        ])
        .split(inner);

    draw_sparkline(f, rows[0], cpu);
    draw_overall_row(f, rows[1], cpu);
    draw_core_grid(f, rows[2], cpu);
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

fn draw_overall_row(f: &mut Frame, area: Rect, cpu: &CpuData) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    let pct = cpu.total_usage.round() as u16;
    let color = gauge_color(cpu.total_usage as f64 / 100.0);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(theme::GAUGE_BG))
        .percent(pct)
        .label(Span::styled(
            format!("{pct}%"),
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        ));
    f.render_widget(gauge, cols[0]);

    let (la1, la5, la15) = cpu.load_avg;
    let load_text = if la1 == 0.0 && la5 == 0.0 && la15 == 0.0 {
        format!(" Cores: {}", cpu.cores.len())
    } else {
        format!(" Load: {la1:.2}  {la5:.2}  {la15:.2}")
    };

    f.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(load_text, dim_style())])),
        cols[1],
    );
}

fn draw_core_grid(f: &mut Frame, area: Rect, cpu: &CpuData) {
    if cpu.cores.is_empty() || area.height == 0 {
        return;
    }

    let cell_width = 13u16;
    let cols_per_row = ((area.width) / cell_width).max(1) as usize;
    let num_rows = cpu.cores.len().div_ceil(cols_per_row);

    let row_h = (area.height / num_rows as u16).max(1);
    let row_constraints: Vec<Constraint> = (0..num_rows)
        .map(|_| Constraint::Length(row_h))
        .collect();

    let row_rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(area);

    for (row_i, row_area) in row_rects.iter().enumerate() {
        let start = row_i * cols_per_row;
        let end   = (start + cols_per_row).min(cpu.cores.len());
        let slice = &cpu.cores[start..end];

        let col_constraints: Vec<Constraint> =
            slice.iter().map(|_| Constraint::Ratio(1, slice.len() as u32)).collect();

        let col_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(*row_area);

        for (col_i, core) in slice.iter().enumerate() {
            draw_core_cell(f, col_rects[col_i], &core.name, core.usage);
        }
    }
}

fn draw_core_cell(f: &mut Frame, area: Rect, name: &str, usage: f32) {
    if area.width < 6 {
        return;
    }

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(4), Constraint::Min(4)])
        .split(area);

    f.render_widget(
        Paragraph::new(Span::styled(format!("{name:<3} "), header_style())),
        cols[0],
    );

    let pct = usage.round() as u16;
    let color = gauge_color(usage as f64 / 100.0);
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(color).bg(theme::GAUGE_BG))
            .percent(pct)
            .label(format!("{pct}%")),
        cols[1],
    );
}
