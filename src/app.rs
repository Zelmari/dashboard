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

    // Navigation only. Ctrl-C is the sole exit path, handled in main.rs.
    // Ctrl-N / Ctrl-P are emacs-style aliases for down / up.
    pub fn on_key(
        &mut self,
        key: crossterm::event::KeyCode,
        modifiers: crossterm::event::KeyModifiers,
    ) -> bool {
        use crossterm::event::{KeyCode, KeyModifiers};

        let ctrl = modifiers.contains(KeyModifiers::CONTROL);

        match key {
            KeyCode::Down              => { self.process_move(1);       true }
            KeyCode::Up                => { self.process_move_back(1);  true }
            KeyCode::Char('n') if ctrl => { self.process_move(1);       true }
            KeyCode::Char('p') if ctrl => { self.process_move_back(1);  true }
            KeyCode::PageDown          => { self.process_move(10);      true }
            KeyCode::PageUp            => { self.process_move_back(10); true }
            KeyCode::Home              => { self.selected_process = 0;  true }
            KeyCode::End               => {
                self.selected_process = self.process_count().saturating_sub(1);
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

    // Reserved for a future process-detail panel.
    #[allow(dead_code)]
    pub fn selected_process_name(&self) -> Option<&str> {
        self.snapshot.as_ref()?.processes.get(self.selected_process)
            .map(|p| p.name.as_str())
    }
}
