pub mod activity;
pub mod cpu;
pub mod memory;
pub mod processes;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Min(10),
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
}
