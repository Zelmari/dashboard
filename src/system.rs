use anyhow::Result;
use sysinfo::{CpuExt, PidExt, ProcessExt, ProcessStatus, System, SystemExt};
use std::collections::VecDeque;

pub const CPU_HISTORY_LEN: usize = 60;

pub const ACTIVITY_THRESHOLD: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct CoreInfo {
    /// e.g. "C0", "C1" …
    pub name: String,
    /// 0.0 – 100.0
    pub usage: f32,
}

#[derive(Debug, Clone)]
pub struct CpuData {
    pub brand: String,
    pub total_usage: f32,
    pub cores: Vec<CoreInfo>,
    pub history: VecDeque<f64>,
    pub load_avg: (f64, f64, f64),
}

#[derive(Debug, Clone)]
pub struct MemData {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortBy {
    Cpu,
    Memory,
    Pid,
    Name,
}

impl SortBy {
    pub fn next(self) -> Self {
        match self {
            SortBy::Cpu    => SortBy::Memory,
            SortBy::Memory => SortBy::Pid,
            SortBy::Pid    => SortBy::Name,
            SortBy::Name   => SortBy::Cpu,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SortBy::Cpu    => "CPU%",
            SortBy::Memory => "MEM",
            SortBy::Pid    => "PID",
            SortBy::Name   => "NAME",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_bytes: u64,
    pub status: String,
    pub user: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Trend {
    Rising,
    Falling,
    #[allow(dead_code)]
    Stable,
}

#[derive(Debug, Clone)]
pub struct ActiveProcess {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_bytes: u64,
    pub trend: Trend,
    pub delta: f32,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub cpu: CpuData,
    pub mem: MemData,
    pub processes: Vec<ProcessInfo>,
    pub active: Vec<ActiveProcess>,
    pub uptime_secs: u64,
}

pub struct SystemInfo {
    sys: System,
    cpu_history: VecDeque<f64>,
    prev_cpu: std::collections::HashMap<u32, f32>,
}

impl SystemInfo {
    pub fn new() -> Result<Self> {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys.refresh_all();

        Ok(Self {
            sys,
            cpu_history: VecDeque::with_capacity(CPU_HISTORY_LEN),
            prev_cpu: std::collections::HashMap::new(),
        })
    }

    pub fn refresh(&mut self, sort_by: SortBy) -> Result<Snapshot> {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.sys.refresh_processes();

        let cpu = self.build_cpu_data();
        let mem = self.build_mem_data();
        let uptime_secs = self.sys.uptime();
        let (processes, active) = self.build_process_data(sort_by);

        Ok(Snapshot { cpu, mem, processes, active, uptime_secs })
    }

    fn build_cpu_data(&mut self) -> CpuData {
        let cpus = self.sys.cpus();

        let total_usage = if cpus.is_empty() {
            0.0
        } else {
            cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
        };

        if self.cpu_history.len() >= CPU_HISTORY_LEN {
            self.cpu_history.pop_front();
        }
        self.cpu_history.push_back(total_usage as f64);

        let cores: Vec<CoreInfo> = cpus
            .iter()
            .enumerate()
            .map(|(i, c)| CoreInfo {
                name: format!("C{i}"),
                usage: c.cpu_usage(),
            })
            .collect();

        let brand = cpus
            .first()
            .map(|c| c.brand().trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let la = self.sys.load_average();

        CpuData {
            brand,
            total_usage,
            cores,
            history: self.cpu_history.clone(),
            load_avg: (la.one, la.five, la.fifteen),
        }
    }

    fn build_mem_data(&self) -> MemData {
        MemData {
            total_bytes:     self.sys.total_memory(),
            used_bytes:      self.sys.used_memory(),
            available_bytes: self.sys.available_memory(),
            swap_total_bytes: self.sys.total_swap(),
            swap_used_bytes:  self.sys.used_swap(),
        }
    }

    fn build_process_data(&mut self, sort_by: SortBy) -> (Vec<ProcessInfo>, Vec<ActiveProcess>) {
        let mut processes: Vec<ProcessInfo> = self
            .sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                pid:         p.pid().as_u32(),
                name:        p.name().to_string(),
                cpu_percent: p.cpu_usage(),
                mem_bytes:   p.memory(),
                status:      format_status(p.status()),
                user:        p.user_id()
                              .map(|u| format!("{}", **u))
                              .unwrap_or_else(|| "-".to_string()),
            })
            .collect();

        match sort_by {
            SortBy::Cpu => processes.sort_by(|a, b| {
                b.cpu_percent.partial_cmp(&a.cpu_percent)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            SortBy::Memory => processes.sort_by(|a, b| b.mem_bytes.cmp(&a.mem_bytes)),
            SortBy::Pid    => processes.sort_by_key(|p| p.pid),
            SortBy::Name   => processes.sort_by(|a, b| a.name.cmp(&b.name)),
        }

        let mut active: Vec<ActiveProcess> = processes
            .iter()
            .filter_map(|p| {
                let prev = self.prev_cpu.get(&p.pid).copied().unwrap_or(p.cpu_percent);
                let delta = p.cpu_percent - prev;
                if delta.abs() >= ACTIVITY_THRESHOLD {
                    Some(ActiveProcess {
                        pid:         p.pid,
                        name:        p.name.clone(),
                        cpu_percent: p.cpu_percent,
                        mem_bytes:   p.mem_bytes,
                        trend: if delta > 0.0 { Trend::Rising } else { Trend::Falling },
                        delta,
                    })
                } else {
                    None
                }
            })
            .collect();

        active.sort_by(|a, b| {
            b.delta.abs().partial_cmp(&a.delta.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        active.truncate(8);

        self.prev_cpu = processes.iter().map(|p| (p.pid, p.cpu_percent)).collect();

        (processes, active)
    }
}

fn format_status(status: ProcessStatus) -> String {
    match status {
        ProcessStatus::Run     => "Run".to_string(),
        ProcessStatus::Sleep   => "Sleep".to_string(),
        ProcessStatus::Idle    => "Idle".to_string(),
        ProcessStatus::Stop    => "Stop".to_string(),
        ProcessStatus::Zombie  => "Zombie".to_string(),
        ProcessStatus::Dead    => "Dead".to_string(),
        ProcessStatus::Tracing => "Trace".to_string(),
        ProcessStatus::Parked  => "Parked".to_string(),
        other                  => format!("{other:?}"),
    }
}
