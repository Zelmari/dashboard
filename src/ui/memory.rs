use human_bytes::human_bytes;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    system::MemData,
    theme::{border_style, dim_style, gauge_color, title_style, TEXT, GAUGE_BG},
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
            Constraint::Min(4),    // stat text
            Constraint::Length(1), // RAM bar
            Constraint::Length(1), // SWAP bar
        ])
        .split(inner);

    draw_stats(f, rows[0], mem);
    draw_bar(f, rows[1], "RAM ", mem_pct(mem.used_bytes, mem.total_bytes));
    draw_bar(f, rows[2], "SWAP", mem_pct(mem.swap_used_bytes, mem.swap_total_bytes));
}

fn mem_pct(used: u64, total: u64) -> u16 {
    if total == 0 { 0 } else { (used as f64 / total as f64 * 100.0).round() as u16 }
}

fn draw_stats(f: &mut Frame, area: Rect, mem: &MemData) {
    let used_pct  = mem_pct(mem.used_bytes,  mem.total_bytes);
    let avail_pct = 100u16.saturating_sub(used_pct);

    let used_color  = gauge_color(used_pct  as f64 / 100.0);
    let avail_color = gauge_color(avail_pct as f64 / 100.0);

    let lines = vec![
        Line::from(vec![
            Span::styled("Total:  ", dim_style()),
            Span::styled(human_bytes(mem.total_bytes as f64),
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Used:   ", dim_style()),
            Span::styled(
                format!("{}", human_bytes(mem.used_bytes as f64)),
                Style::default().fg(used_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  {}%", used_pct), Style::default().fg(used_color)),
        ]),
        Line::from(vec![
            Span::styled("Avail:  ", dim_style()),
            Span::styled(
                format!("{}", human_bytes(mem.available_bytes as f64)),
                Style::default().fg(TEXT),
            ),
            Span::styled(format!("  {}%", avail_pct), Style::default().fg(avail_color)),
        ]),
        Line::from(vec![
            Span::styled("Swap:   ", dim_style()),
            Span::styled(
                format!("{} / {}",
                    human_bytes(mem.swap_used_bytes as f64),
                    human_bytes(mem.swap_total_bytes as f64)),
                Style::default().fg(TEXT),
            ),
        ]),
    ];

    f.render_widget(Paragraph::new(lines), area);
}

fn draw_bar(f: &mut Frame, area: Rect, label: &str, pct: u16) {
    if area.width < 10 {
        return;
    }

    let color    = gauge_color(pct as f64 / 100.0);
    let pct_str  = format!("{pct:>3}%");
    let label_w  = label.len() + 1;
    let pct_w    = pct_str.len() + 1;
    let bar_w    = (area.width as usize).saturating_sub(label_w + pct_w).max(1);

    let filled = (bar_w * pct as usize / 100).min(bar_w);
    let empty  = bar_w - filled;

    let line = Line::from(vec![
        Span::styled(format!("{label} "),   Style::default().fg(TEXT).add_modifier(Modifier::BOLD)),
        Span::styled("█".repeat(filled),     Style::default().fg(color)),
        Span::styled("░".repeat(empty),      Style::default().fg(GAUGE_BG)),
        Span::styled(format!(" {pct_str}"),  Style::default().fg(color)),
    ]);

    f.render_widget(Paragraph::new(line), area);
}
