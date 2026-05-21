use std::collections::VecDeque;

#[derive(Clone)]
struct Process {
    id: usize,
    burst_time: u32,
}

fn fcfs(processes: &Vec<Process>) {
    println!("--- FCFS Scheduling ---");
    let mut current_time = 0;

    for p in processes {
        println!(
            "Process P{} runs from {} to {}",
            p.id,
            current_time,
            current_time + p.burst_time
        );

        current_time += p.burst_time;
    }

    println!("Total time to complete all {}", current_time);
    println!();
}

fn sjf(processes: &Vec<Process>) {
    println!("--- FCFS Scheduling ---");
    let mut current_time = 0;

    let mut sorted = processes.clone();

    sorted.sort_by_key(|p| p.burst_time);
    for p in sorted {
        println!(
            "Process P{} runs from {} to {}",
            p.id,
            current_time,
            current_time + p.burst_time
        );

        current_time += p.burst_time;
    }

    println!("Total time to complete all {}", current_time);
    println!();
}

fn round_robin(processes: &Vec<Process>, quantum: u32) {
    println!("--- Round Robin Scheduling ---");

    let mut queue: VecDeque<Process> = VecDeque::new();

    for p in processes {
        queue.push_back(p.clone());
    }

    let mut remaining: Vec<u32> = processes.iter().map(|p| p.burst_time).collect();

    let mut current_time = 0;

    while let Some(process) = queue.pop_front() {
        let idx = process.id - 1;

        if remaining[idx] == 0 {
            continue;
        }
        let run_time = remaining[idx].min(quantum);

        println!(
            "Process P{} runs from {} to {}",
            process.id,
            current_time,
            current_time + run_time
        );

        current_time += run_time;
        remaining[idx] -= run_time;

        if remaining[idx] > 0 {
            queue.push_back(process);
        }
    }
    println!("Total time to complete all {}", current_time);
    println!();
}
fn main() {
    let processes = vec![
        Process {
            id: 1,
            burst_time: 5,
        },
        Process {
            id: 2,
            burst_time: 3,
        },
        Process {
            id: 3,
            burst_time: 8,
        },
    ];

    fcfs(&processes);

    sjf(&processes);

    round_robin(&processes, 2);
}
