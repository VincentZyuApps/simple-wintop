use std::time::Duration;

pub struct SystemData {
    pub cpus: Vec<CpuData>,
    pub memory: MemoryData,
    pub swap: SwapData,
    pub tasks: TasksData,
    pub load_avg: LoadAverageData,
    pub uptime: Duration,
}

pub struct CpuData {
    pub name: String,
    pub usage: f64,
}

pub struct MemoryData {
    pub total: u64,
    pub used: u64,
}

pub struct SwapData {
    pub total: u64,
    pub used: u64,
}

pub struct TasksData {
    pub total: u32,
    pub running: u32,
}

pub struct LoadAverageData {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}
