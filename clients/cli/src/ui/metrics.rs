//! System metrics collection and display.

use std::time::Instant;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

/// System metrics for display in the dashboard.
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    /// CPU usage percentage (0.0 to 100.0).
    pub cpu_percent: f32,
    /// Current process RAM usage in bytes.
    pub ram_bytes: u64,
    /// Peak process RAM usage in bytes since startup.
    pub peak_ram_bytes: u64,
    /// Total system RAM in bytes.
    pub total_ram_bytes: u64,
    /// Estimated GFLOP/s based on CPU and thread count.
    pub gflops: f64,
    /// Last time CPU was updated for proper refresh timing
    pub last_cpu_update: Option<Instant>,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            ram_bytes: 0,
            peak_ram_bytes: 0,
            total_ram_bytes: {
                let mut sys = System::new();
                sys.refresh_memory();
                sys.total_memory()
            },
            gflops: 0.0, // Initialize gflops
            last_cpu_update: None,
        }
    }
}

impl SystemMetrics {
    /// Update metrics from system information, tracking peak memory over time.
    pub fn update(
        sysinfo: &mut System,
        previous_peak: u64,
        previous_metrics: Option<&SystemMetrics>,
    ) -> Self {
        let now = Instant::now();
        let current_pid = Pid::from(std::process::id() as usize);
        let mut cpu_total = 0.0;
        let mut ram_total = 0;

        let should_update_cpu = previous_metrics
            .and_then(|pm| pm.last_cpu_update)
            .map(|last_update| now.duration_since(last_update) >= sysinfo::MINIMUM_CPU_UPDATE_INTERVAL)
            .unwrap_or(true);

        let last_cpu_update = if should_update_cpu {
            sysinfo.refresh_cpu_usage();
            sysinfo.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::everything(),
            );
            Some(now)
        } else {
            sysinfo.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::nothing().with_memory(), // CORRECTED
            );
            previous_metrics.and_then(|m| m.last_cpu_update)
        };

        if let Some(process) = sysinfo.process(current_pid) {
            cpu_total = if should_update_cpu {
                process.cpu_usage()
            } else {
                previous_metrics.map_or(0.0, |m| m.cpu_percent)
            };
            ram_total = process.memory();
        }

        for process in sysinfo.processes().values() {
            if process.parent() == Some(current_pid)
                && process.name().to_string_lossy().contains("nexus")
            {
                ram_total += process.memory();
                if should_update_cpu {
                    cpu_total += process.cpu_usage();
                }
            }
        }

        Self {
            cpu_percent: cpu_total,
            ram_bytes: ram_total,
            peak_ram_bytes: previous_peak.max(ram_total),
            total_ram_bytes: sysinfo.total_memory(),
            gflops: previous_metrics.map_or(0.0, |m| m.gflops),
            last_cpu_update,
        }
    }

    pub fn ram_ratio(&self) -> f64 {
        if self.total_ram_bytes > 0 {
            self.ram_bytes as f64 / self.total_ram_bytes as f64
        } else {
            0.0
        }
    }

    pub fn format_ram(&self) -> String {
        let mb = self.ram_bytes as f64 / 1_048_576.0;
        if mb >= 1024.0 {
            format!("{:.1} GB", mb / 1024.0)
        } else {
            format!("{:.1} MB", mb)
        }
    }

    pub fn cpu_color(&self) -> ratatui::prelude::Color {
        use ratatui::prelude::Color;
        match self.cpu_percent {
            p if p >= 80.0 => Color::Red,
            p if p >= 60.0 => Color::Yellow,
            _ => Color::Green,
        }
    }

    pub fn ram_color(&self) -> ratatui::prelude::Color {
        use ratatui::prelude::Color;
        let ratio = self.ram_ratio();
        match ratio {
            r if r >= 0.8 => Color::Red,
            r if r >= 0.6 => Color::Yellow,
            _ => Color::Green,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZkVMMetrics {
    pub tasks_fetched: usize,
    pub tasks_submitted: usize,
    pub zkvm_runtime_secs: u64,
    pub last_task_status: String,
    pub _total_points: u64,
}

impl Default for ZkVMMetrics {
    fn default() -> Self {
        Self {
            tasks_fetched: 0,
            tasks_submitted: 0,
            zkvm_runtime_secs: 0,
            last_task_status: "None".to_string(),
            _total_points: 0,
        }
    }
}

impl ZkVMMetrics {
    pub fn success_rate(&self) -> f64 {
        if self.tasks_fetched == 0 {
            0.0
        } else {
            (self.tasks_submitted as f64 / self.tasks_fetched as f64) * 100.0
        }
    }

    pub fn _format_points(&self) -> String {
        let points = self._total_points;
        if points >= 1_000_000 {
            format!("{:.1}M", points as f64 / 1_000_000.0)
        } else if points >= 1_000 {
            format!("{},{:03}", points / 1_000, points % 1_000)
        } else {
            points.to_string()
        }
    }

    pub fn success_rate_color(&self) -> ratatui::prelude::Color {
        use ratatui::prelude::Color;
        let rate = self.success_rate();
        match rate {
            r if r >= 75.0 => Color::Green,
            r if r >= 50.0 => Color::Yellow,
            _ => Color::Red,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TaskFetchInfo {}
