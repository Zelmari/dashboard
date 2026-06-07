pub mod activity;
pub mod cpu;
pub mod memory;
pub mod processes;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;
use crate::theme::dim_style;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35), // CPU
            Constraint::Min(10),        // Processes + sidebar
            Constraint::Length(1),      // Status bar
        ])
        .split(size);

    if let Some(snap) = &app.snapshot {
        cpu::draw(f, rows[0], &snap.cpu);
    }

    let lower = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(rows[1]);

    if let Some(snap) = &app.snapshot {
        processes::draw(f, lower[0], &snap.processes, app.selected_process, app.sort_by);
    }

    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(lower[1]);

    if let Some(snap) = &app.snapshot {
        memory::draw(f, sidebar[0], &snap.mem);
        activity::draw(f, sidebar[1], &snap.active);
    }

    if let Some(snap) = &app.snapshot {
        draw_status_bar(f, rows[2], app, snap.uptime_secs);
    }
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &App, uptime_secs: u64) {
    let uptime = format_uptime(uptime_secs);

    let line = Line::from(vec![
        Span::styled(format!(" up {uptime} "), dim_style()),
        Span::styled("│", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!(" sort:{} ", app.sort_by.label()),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("│", Style::default().fg(Color::DarkGray)),
        Span::styled(
            " [q]Quit  [↑↓/jk]Nav  [Tab]Sort  [c]CPU [m]Mem [p]PID [n]Name  [g/G]Top/Bot ",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    f.render_widget(Paragraph::new(line), area);
}

fn format_uptime(secs: u64) -> String {
    let days  = secs / 86_400;
    let hours = (secs % 86_400) / 3_600;
    let mins  = (secs % 3_600) / 60;

    if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    }
}
