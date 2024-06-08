use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Task {
    pid: usize,
    arrival_time: usize,
    burst_time: usize,
    remaining_time: usize,
}

impl Task {
    fn new(pid: usize, arrival_time: usize, burst_time: usize) -> Task {
        Task {
            pid,
            arrival_time,
            burst_time,
            remaining_time: burst_time,
        }
    }
}

fn read_tasks(filename: &str) -> Vec<Task> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let parts: Vec<usize> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() == 3 {
                Some(Task::new(parts[0], parts[1], parts[2]))
            } else {
                None
            }
        })
        .collect()
}

fn calculate_statistics(results: &[(usize, usize, usize)], tasks: &[Task]) -> (f64, f64) {
    let burst_time_sum: usize = tasks.iter().map(|task| task.burst_time).sum();
    let total_turnaround_time: usize = results
        .iter()
        .map(|(pid, start, finish)| {
            let task = tasks.iter().find(|task| task.pid == *pid).unwrap();
            finish - task.arrival_time
        })
        .sum();

    let total_waiting_time = total_turnaround_time - burst_time_sum;
    let avg_waiting_time = total_waiting_time as f64 / tasks.len() as f64;
    let avg_turnaround_time = total_turnaround_time as f64 / tasks.len() as f64;
    (avg_waiting_time, avg_turnaround_time)
}

fn fcfs(tasks: &[Task]) -> Vec<(usize, usize, usize)> {
    let mut queue: VecDeque<Task> = tasks.iter().cloned().collect();
    queue.make_contiguous().sort_by_key(|task| task.arrival_time);
    let mut current_time = 0;
    let mut results = Vec::new();
    while let Some(task) = queue.pop_front() {
        let start_time = current_time.max(task.arrival_time);
        let finish_time = start_time + task.burst_time;
        results.push((task.pid, start_time, finish_time));
        current_time = finish_time;
    }
    results
}
fn round_robin(tasks: &[Task], time_quantum: usize) -> Vec<(usize, usize, usize)> {
    let mut tasks: Vec<Task> = tasks.iter().cloned().collect();
    tasks.sort_by_key(|task| task.arrival_time);
    let mut current_time =tasks[0].arrival_time;
    // println!("{}",current_time);
    let mut results = Vec::new();
    let mut ready_queue: VecDeque<Task> = VecDeque::new();
    ready_queue.push_back(tasks.remove(0));
    while !tasks.is_empty() || !ready_queue.is_empty() {
        

        if let Some(mut task) = ready_queue.pop_front() {
            let start_time = current_time;
            let run_time = task.remaining_time.min(time_quantum);
            current_time += run_time;
            task.remaining_time -= run_time;
            if task.remaining_time > 0 {
                while let Some(task) = tasks.first() {
                    if task.arrival_time <= current_time {
                        ready_queue.push_back(tasks.remove(0));
                        
        
                    } else {
                        break;
                    }
                }
                ready_queue.push_back(task);
            } else {
                results.push((task.pid, start_time, current_time));
            }
        } else if let Some(next_task) = tasks.first() {
            current_time = next_task.arrival_time;
        }
    }

    results
}
fn srtf(tasks: &[Task]) -> Vec<(usize, usize, usize)> {
    let mut tasks: Vec<Task> = tasks.iter().cloned().collect();
    tasks.sort_by_key(|task| task.arrival_time);
    let mut current_time = tasks[0].arrival_time;
    let mut results = Vec::new();
    let mut ready_queue: Vec<Task> = Vec::new();
    let mut running_task: Option<Task> = None;

    while !tasks.is_empty() || !ready_queue.is_empty() || running_task.is_some() {
        while let Some(task) = tasks.first() {
            if task.arrival_time <= current_time {
                let mut new_task = tasks.remove(0);
                new_task.remaining_time = new_task.burst_time;
                ready_queue.push(new_task);
                ready_queue.sort_by_key(|task| task.remaining_time);
            } else {
                break;
            }
        }

        if running_task.is_none() && !ready_queue.is_empty() {
            running_task = Some(ready_queue.remove(0));
        }

        if let Some(mut task) = running_task.take() {
            task.remaining_time -= 1;
            current_time += 1;
            if task.remaining_time == 0 {
                results.push((task.pid, current_time - task.burst_time, current_time));
            } else {
                ready_queue.push(task);
                ready_queue.sort_by_key(|task| task.remaining_time);
            }
        } else {
            current_time += 1;
        }
    }

    results
}
fn main() {
    let tasks = read_tasks("input.txt");
    if tasks.is_empty() {
        println!("No tasks to schedule.");
        return;
    }
    // FCFS Scheduling
    let results_fcfs = fcfs(&tasks);
    let (avg_waiting_time_fcfs, avg_turnaround_time_fcfs) = calculate_statistics(&results_fcfs, &tasks);
    println!("FCFS Scheduling Results:");
    for (pid, start, finish) in &results_fcfs {
        println!("Time {}: Task {} starts", start, pid);
        println!("Time {}: Task {} finishes", finish, pid);
    }
    println!("Average Waiting Time: {:.2}", avg_waiting_time_fcfs);
    println!("Average Turnaround Time: {:.2}", avg_turnaround_time_fcfs);
    // Round Robin Scheduling
    let time_quantum = 2; // Example quantum; adjust as needed
    let results_rr = round_robin(&tasks, time_quantum);
    let (avg_waiting_time_rr, avg_turnaround_time_rr) = calculate_statistics(&results_rr, &tasks);
    println!("Round Robin Scheduling Results:");
    for (pid, start, finish) in &results_rr {
        println!("Time {}: Task {} starts", start, pid);
        println!("Time {}: Task {} finishes", finish, pid);
    }
    println!("Average Waiting Time: {:.2}", avg_waiting_time_rr);
    println!("Average Turnaround Time: {:.2}", avg_turnaround_time_rr);
    // SRTF Scheduling
    let results_srtf = srtf(&tasks);
    let (avg_waiting_time_srtf, avg_turnaround_time_srtf) = calculate_statistics(&results_srtf, &tasks);
    println!("SRTF Scheduling Results:");
    for (pid, start, finish) in &results_srtf {
        println!("Time {}: Task {} starts", start, pid);
        println!("Time {}: Task {} finishes", finish, pid);
    }
    println!("Average Waiting Time: {:.2}", avg_waiting_time_srtf);
    println!("Average Turnaround Time: {:.2}", avg_turnaround_time_srtf);
}
