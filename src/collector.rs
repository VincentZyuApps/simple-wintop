use std::collections::VecDeque;
use std::time::Duration;

use sysinfo::System;

use crate::data::*;

pub struct Collector {
    system: System,
    cpu_history: VecDeque<f64>,
}

const MAX_HISTORY: usize = 900;

impl Collector {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_cpu_list();
        system.refresh_memory();
        Self {
            system,
            cpu_history: VecDeque::with_capacity(MAX_HISTORY),
        }
    }

    pub fn collect(&mut self) -> SystemData {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.system
            .refresh_processes_specifics(sysinfo::ProcessRefreshKind::new());

        let cpus: Vec<CpuData> = self
            .system
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| CpuData {
                name: i.to_string(),
                usage: cpu.cpu_usage() as f64,
            })
            .collect();

        let overall_usage: f64 = self.system.global_cpu_info().cpu_usage() as f64;
        let num_cores = cpus.len() as f64;
        let load_value = if num_cores > 0.0 {
            overall_usage / 100.0 * num_cores
        } else {
            0.0
        };

        self.cpu_history.push_front(load_value);
        while self.cpu_history.len() > MAX_HISTORY {
            self.cpu_history.pop_back();
        }

        SystemData {
            cpus,
            memory: MemoryData {
                total: self.system.total_memory(),
                used: self.system.used_memory(),
            },
            swap: SwapData {
                total: self.system.total_swap(),
                used: self.system.used_swap(),
            },
            tasks: TasksData {
                total: self.system.total_process(),
                running: self.system.running_process(),
            },
            load_avg: LoadAverageData {
                one: self.average_over(60),
                five: self.average_over(300),
                fifteen: self.average_over(900),
            },
            uptime: Duration::from_secs(self.system.uptime()),
        }
    }

    fn average_over(&self, seconds: usize) -> f64 {
        let n = self.cpu_history.len().min(seconds);
        if n == 0 {
            return 0.0;
        }
        let sum: f64 = self.cpu_history.iter().take(n).sum();
        sum / n as f64
    }
}
