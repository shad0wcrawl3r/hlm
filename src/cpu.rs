use crate::{
    files::read_file,
    structs::{CpuStat, CpuTime, Irqs},
};

fn parse_cpu_time(parts: Vec<&str>) -> CpuTime {
    // Parse each part to f64 and collect into a vector
    let values: Vec<i64> = parts[1..]
        .into_iter()
        .map(|&s| s.parse::<i64>().unwrap_or(0)) // Parse each part to f64
        .collect();
    CpuTime::new(values)
    // Create a CpuStat instance using the parsed values
}
pub fn get_cpu_info() -> std::io::Result<()> {
    let stat_file = read_file("/proc/stat")?;
    // println!("{:#?}", stat_file);
    let mut cpu_stat = CpuStat::default();
    for line in stat_file.lines() {
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        if line.starts_with("cpu ") {
            cpu_stat.average = parse_cpu_time(line_parts);
        } else if line.starts_with("cpu") {
            let cpu_id = line_parts[0].replace("cpu", "");
            if let Ok(cpu_id) = cpu_id.parse::<usize>() {
                cpu_stat.per_cpu.insert(cpu_id, parse_cpu_time(line_parts));
            }
        } else {
            match line_parts[0] {
                "intr" => cpu_stat.interrupts = line_parts[1].parse::<usize>().unwrap_or(0),
                "ctxt" => cpu_stat.ctxt = line_parts[1].parse::<usize>().unwrap_or(0),
                "btime" => cpu_stat.btime = line_parts[1].parse::<usize>().unwrap_or(0),
                "processes" => cpu_stat.processes = line_parts[1].parse::<usize>().unwrap_or(0),
                "procs_running" => {
                    cpu_stat.processes_running = line_parts[1].parse::<usize>().unwrap_or(0)
                }
                "procs_blocked" => {
                    cpu_stat.processes_blocked = line_parts[1].parse::<usize>().unwrap_or(0)
                }
                "softirq" => {
                    cpu_stat.softirqs = Irqs {
                        total: line_parts[1].parse::<usize>().unwrap_or(0),
                        timer: line_parts[2].parse::<usize>().unwrap_or(0),
                        network: line_parts[3].parse::<usize>().unwrap_or(0),
                        block: line_parts[4].parse::<usize>().unwrap_or(0),
                        irqpoll: line_parts[5].parse::<usize>().unwrap_or(0),
                        tasklet: line_parts[6].parse::<usize>().unwrap_or(0),
                        swiotlb: line_parts[7].parse::<usize>().unwrap_or(0),
                        hrtimer: line_parts[8].parse::<usize>().unwrap_or(0),
                        rcu: line_parts[9].parse::<usize>().unwrap_or(0),
                        other: line_parts[10].parse::<usize>().unwrap_or(0),
                    }
                }
                _ => continue,
            }
        }
    }
    println!("{cpu_stat:#?}");
    Ok(())
}
