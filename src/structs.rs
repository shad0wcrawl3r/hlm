use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CpuStat {
    pub average: CpuTime,
    pub per_cpu: HashMap<usize, CpuTime>,
    pub interrupts: usize,
    pub ctxt: usize,
    pub btime: usize,
    pub processes: usize,
    pub processes_running: usize,
    pub processes_blocked: usize,
    pub softirqs: Irqs,
}

#[derive(Debug, Default)]
pub struct Irqs {
    pub total: usize,
    pub timer: usize,
    pub network: usize,
    pub block: usize,
    pub irqpoll: usize,
    pub tasklet: usize,
    pub swiotlb: usize,
    pub hrtimer: usize,
    pub rcu: usize,
    pub other: usize,
}

#[derive(Debug, Default)]
pub struct CpuTime {
    user: i64,
    nice: i64,
    system: i64,
    idle: i64,
    iowait: i64,
    irq: i64,
    softirq: i64,
    steal: i64,
}
impl CpuTime {
    pub fn new(info: Vec<i64>) -> Self {
        CpuTime {
            user: info[1],
            nice: info[2],
            system: info[3],
            idle: info[4],
            iowait: info[5],
            irq: info[6],
            softirq: info[7],
            steal: info[8],
        }
    }
}
