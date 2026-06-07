use human_bytes::human_bytes;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::{
    system::{ProcessInfo, SortBy},
    theme::{self, border_style, dim_style, header_style, selected_style, title_style},
};

pub fn draw(
    f: &mut Frame,
    area: Rect,
    processes: &[ProcessInfo],
    selected_index: usize,
    sort_by: SortBy,
) {
    let total = processes.len();

    let block = Block::default()
        .title(Span::styled(
            format!(" PROCESSES ({total})  sort:{} ", sort_by.label()),
            title_style(),
        ))
        .borders(Borders::ALL)
        .border_style(border_style());

    let widths = [
        Constraint::Length(7),  // PID
        Constraint::Min(16),    // NAME  (flexible)
        Constraint::Length(8),  // CPU%
        Constraint::Length(10), // MEM
        Constraint::Length(8),  // STATUS
        Constraint::Length(7),  // USER
    ];

    let header = Row::new(vec![
        make_header("PID",    SortBy::Pid,    sort_by),
        make_header("NAME",   SortBy::Name,   sort_by),
        make_header("CPU%",   SortBy::Cpu,    sort_by),
        make_header("MEM",    SortBy::Memory, sort_by),
        Cell::from(Span::styled("STATUS", header_style())),
        Cell::from(Span::styled("USER",   header_style())),
    ])
    .style(Style::default().add_modifier(Modifier::UNDERLINED));

    let rows: Vec<Row> = processes
        .iter()
        .enumerate()
        .map(|(i, p)| build_row(i, p, selected_index))
        .collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .highlight_style(selected_style())
        .highlight_symbol("▶ ");

    let mut state = TableState::default();
    state.select(Some(selected_index));

    f.render_stateful_widget(table, area, &mut state);
}

fn build_row(i: usize, p: &ProcessInfo, selected: usize) -> Row<'static> {
    let is_sel = i == selected;

    let cpu_color = if is_sel {
        theme::TEXT_HIGHLIGHT
    } else {
        theme::gauge_color(p.cpu_percent as f64 / 100.0)
    };

    let sel_or = |style: ratatui::style::Style| if is_sel { selected_style() } else { style };

    Row::new(vec![
        Cell::from(Span::styled(p.pid.to_string(),                sel_or(dim_style()))),
        Cell::from(Span::styled(truncate(&p.name, 18),            sel_or(Style::default().fg(theme::TEXT)))),
        Cell::from(Span::styled(format!("{:.1}%", p.cpu_percent), Style::default().fg(cpu_color))),
        Cell::from(Span::styled(human_bytes(p.mem_bytes as f64),  sel_or(dim_style()))),
        Cell::from(Span::styled(p.status.clone(),                 sel_or(dim_style()))),
        Cell::from(Span::styled(truncate(&p.user, 7),             sel_or(dim_style()))),
    ])
}

fn make_header(label: &'static str, column: SortBy, active: SortBy) -> Cell<'static> {
    if column == active {
        Cell::from(Span::styled(
            format!("{label} ▲"),
            Style::default().fg(theme::BORDER).add_modifier(Modifier::BOLD),
        ))
    } else {
        Cell::from(Span::styled(label, header_style()))
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut t: String = s.chars().take(max - 1).collect();
        t.push('…');
        t
    }
}
