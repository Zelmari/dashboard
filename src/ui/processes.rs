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
    theme::{
        self, border_style, dim_style, header_style, process_name_color,
        selected_style, title_style,
    },
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
        Constraint::Length(7),
        Constraint::Min(16),
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Length(7),
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

fn build_row(rank: usize, p: &ProcessInfo, selected: usize) -> Row<'static> {
    let is_sel    = rank == selected;
    let name_col  = process_name_color(rank);
    let cpu_color = if is_sel { theme::TEXT_HIGHLIGHT }
                    else { theme::gauge_color(p.cpu_percent as f64 / 100.0) };
    let sel_or    = |s: Style| if is_sel { selected_style() } else { s };

    Row::new(vec![
        Cell::from(Span::styled(p.pid.to_string(), sel_or(dim_style()))),
        Cell::from(Span::styled(truncate(&p.name, 18),
            if is_sel { selected_style() } else { Style::default().fg(name_col) })),
        Cell::from(Span::styled(format!("{:.1}%", p.cpu_percent),
            Style::default().fg(cpu_color))),
        Cell::from(Span::styled(human_bytes(p.mem_bytes as f64), sel_or(dim_style()))),
        Cell::from(Span::styled(p.status.clone(), sel_or(dim_style()))),
        Cell::from(Span::styled(truncate(&p.user, 7), sel_or(dim_style()))),
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
    if s.chars().count() <= max { s.to_string() }
    else { let mut t: String = s.chars().take(max-1).collect(); t.push('…'); t }
}
