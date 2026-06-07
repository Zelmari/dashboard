use anyhow::Result;
use crate::system::{Snapshot, SortBy, SystemInfo};

pub struct App {
    pub should_quit: bool,

    pub snapshot: Option<Snapshot>,

    pub selected_process: usize,

    pub sort_by: SortBy,

    pub tick: u64,

    pub refresh_ms: u64,

    collector: SystemInfo,
}

impl App {
    pub fn new(refresh_ms: u64) -> Result<Self> {
        let mut collector = SystemInfo::new()?;
        let sort_by = SortBy::Cpu;
        let snapshot = Some(collector.refresh(sort_by)?);

        Ok(Self {
            should_quit: false,
            snapshot,
            selected_process: 0,
            sort_by,
            tick: 0,
            refresh_ms,
            collector,
        })
    }

    pub fn on_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);

        match self.collector.refresh(self.sort_by) {
            Ok(snap) => self.snapshot = Some(snap),
            Err(e)   => eprintln!("[dashboard] refresh error: {e}"),
        }
    }

    pub fn on_key(&mut self, key: crossterm::event::KeyCode) -> bool {
        use crossterm::event::KeyCode;

        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
                true
            }

            KeyCode::Down | KeyCode::Char('j') => {
                self.process_move(1);
                true
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.process_move_back(1);
                true
            }
            KeyCode::PageDown => {
                self.process_move(10);
                true
            }
            KeyCode::PageUp => {
                self.process_move_back(10);
                true
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.selected_process = 0;
                true
            }
            KeyCode::End | KeyCode::Char('G') => {
                let max = self.process_count().saturating_sub(1);
                self.selected_process = max;
                true
            }

            KeyCode::Char('c') => { self.sort_by = SortBy::Cpu;    true }
            KeyCode::Char('m') => { self.sort_by = SortBy::Memory; true }
            KeyCode::Char('p') => { self.sort_by = SortBy::Pid;    true }
            KeyCode::Char('n') => { self.sort_by = SortBy::Name;   true }
            KeyCode::Tab => {
                self.sort_by = self.sort_by.next();
                // Reset selection to top when sort changes.
                self.selected_process = 0;
                true
            }

            _ => false,
        }
    }

    fn process_move(&mut self, n: usize) {
        let max = self.process_count().saturating_sub(1);
        self.selected_process = (self.selected_process + n).min(max);
    }

    fn process_move_back(&mut self, n: usize) {
        self.selected_process = self.selected_process.saturating_sub(n);
    }

    pub fn process_count(&self) -> usize {
        self.snapshot.as_ref().map(|s| s.processes.len()).unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn selected_process_name(&self) -> Option<&str> {
        self.snapshot.as_ref()?.processes.get(self.selected_process)
            .map(|p| p.name.as_str())
    }
}
