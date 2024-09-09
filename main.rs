use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Task {
    pid: u32,  // Process ID
    time_arrival: u32,  // Arrival time of the task
    burst_time: u32,  // CPU burst time required by the task
    remaining_time: u32,  // Time remaining for the task completion (used in SRTF and RR)
    start_time: Option<u32>,  // Start time of the task in CPU
    finish_time: Option<u32>,  // Finish time of the task from CPU
}

// Function to sort tasks by arrival time
fn sort_tasks_by_arrival(mut tasks: Vec<Task>) -> Vec<Task> {
    tasks.sort_by_key(|task| task.time_arrival);
    tasks
}

// Function to handle CPU idle time
fn handle_idle_time(current_time: &mut u32, next_arrival_time: u32) {
    println!("Time {}: Idle", current_time);
    *current_time = next_arrival_time;
}

// FCFS Scheduler Function
fn fcfs(mut tasks: Vec<Task>) -> Vec<Task> {
    println!("FCFS (First Come First Serve) Output:");
    tasks = sort_tasks_by_arrival(tasks);
    let mut current_time = 0;

    for task in tasks.iter_mut() {
        if current_time < task.time_arrival {
            handle_idle_time(&mut current_time, task.time_arrival);
        }
        task.start_time = Some(current_time);
        println!("Time {}: Task {} starts", current_time, task.pid);
        current_time += task.burst_time;
        task.finish_time = Some(current_time);
        println!("Time {}: Task {} finishes", current_time, task.pid);
    }
    tasks
}

// Function to move tasks to the queue based on their arrival time
fn move_tasks_to_queue(current_time: u32, tasks: &mut Vec<Task>, queue: &mut VecDeque<Task>) {
    while let Some(task) = tasks.first().cloned() {
        if task.time_arrival <= current_time {
            println!("Time {}: Task {} arrived", current_time, task.pid);
            queue.push_back(tasks.remove(0));
        } else {
            break;
        }
    }
}

// Function to execute a task in RR
fn execute_task_in_rr(current_time: &mut u32, task: &mut Task, quantum: u32, tasks: &mut Vec<Task>, queue: &mut VecDeque<Task>) {
    let execution_time = std::cmp::min(task.remaining_time, quantum);
    for _ in 0..execution_time {
        *current_time += 1;
        task.remaining_time -= 1;
        move_tasks_to_queue(*current_time, tasks, queue);
    }
}

// RR Scheduler Function
fn round_robin(mut tasks: Vec<Task>, quantum: u32) -> Vec<Task> {
    println!("RR (Round Robin) with time_quantum = {}", quantum);
    let mut queue = VecDeque::new();
    let mut current_time = 0;
    let mut completed_tasks = Vec::new();

    while !tasks.is_empty() || !queue.is_empty() {
        move_tasks_to_queue(current_time, &mut tasks, &mut queue);

        if let Some(mut task) = queue.pop_front() {
            if task.start_time.is_none() {
                task.start_time = Some(current_time);
                println!("Time {}: Task {} starts", current_time, task.pid);
            } else {
                println!("Time {}: Task {} resumes", current_time, task.pid);
            }

            execute_task_in_rr(&mut current_time, &mut task, quantum, &mut tasks, &mut queue);

            if task.remaining_time > 0 {
                println!("Time {}: Task {} paused", current_time, task.pid);
                queue.push_back(task);
            } else {
                task.finish_time = Some(current_time);
                println!("Time {}: Task {} finishes", current_time, task.pid);
                completed_tasks.push(task);
            }
        } else {
            // Handle idle CPU time
            println!("Time {}: CPU Idle", current_time);
            if let Some(next_task) = tasks.first() {
                current_time = next_task.time_arrival;
            }
        }
    }

    completed_tasks
}

// SRTF Scheduler Function
fn srtf(mut tasks: Vec<Task>) -> Vec<Task> {
    println!("SRTF (Shortest Remaining Time First) Output:");
    let mut current_time = 0;
    let mut completed_tasks = Vec::new();
    let mut task_queue = Vec::new();

    while !tasks.is_empty() || !task_queue.is_empty() {
        while let Some(task) = tasks.first() {
            if task.time_arrival <= current_time {
                task_queue.push(tasks.remove(0));
            } else {
                break;
            }
        }

        if let Some(index) = task_queue.iter().enumerate().min_by_key(|&(_, task)| task.remaining_time).map(|(i, _)| i) {
            let mut task = task_queue.remove(index);
            if task.start_time.is_none() {
                task.start_time = Some(current_time);
                println!("Time {}: Task {} starts", current_time, task.pid);
            }

            current_time += 1;
            task.remaining_time -= 1;

            if task.remaining_time == 0 {
                task.finish_time = Some(current_time);
                println!("Time {}: Task {} finishes", current_time, task.pid);
                completed_tasks.push(task);
            } else {
                task_queue.push(task);
            }
        } else {
            // Handle idle CPU time
            println!("Time {}: CPU Idle", current_time);
            if let Some(next_task) = tasks.first() {
                current_time = next_task.time_arrival;
            }
        }
    }

    completed_tasks
}


// Function to calculate average waiting time and turnaround time
fn calculate_metrics(tasks: &[Task]) -> (f64, f64) {
    let total_waiting_time: u32 = tasks.iter().map(|task| task.finish_time.unwrap() - task.time_arrival - task.burst_time).sum();
    let total_turnaround_time: u32 = tasks.iter().map(|task| task.finish_time.unwrap() - task.time_arrival).sum();

    let average_waiting_time = total_waiting_time as f64 / tasks.len() as f64;
    let average_turnaround_time = total_turnaround_time as f64 / tasks.len() as f64;

    (average_waiting_time, average_turnaround_time)
}

// Function to read tasks from input.txt file
fn read_tasks_from_file(file_path: &str) -> io::Result<Vec<Task>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let tasks = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let parts: Vec<u32> = line.split_whitespace()
                                      .filter_map(|word| word.parse::<u32>().ok())
                                      .collect();
            if parts.len() == 3 {
                Some(Task {
                    pid: parts[0],
                    time_arrival: parts[1],
                    burst_time: parts[2],
                    remaining_time: parts[2],
                    start_time: None,
                    finish_time: None,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(tasks)
}

// Main function
fn main() {
    let tasks_file_path = "input.txt";
    match read_tasks_from_file(tasks_file_path) {
        Ok(tasks) => {
            let completed_fcfs = fcfs(tasks.clone());
            let completed_rr = round_robin(tasks.clone(), 2);
            let completed_srtf = srtf(tasks.clone());
        
            let (fcfs_avg_wait, fcfs_avg_turn) = calculate_metrics(&completed_fcfs);
            let (rr_avg_wait, rr_avg_turn) = calculate_metrics(&completed_rr);
            let (srtf_avg_wait, srtf_avg_turn) = calculate_metrics(&completed_srtf);
            
            
            println!("FCFS Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", fcfs_avg_wait, fcfs_avg_turn);
            println!("RR Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", rr_avg_wait, rr_avg_turn);
            println!("SRTF Average Waiting Time: {:.2}, Average Turnaround Time: {:.2}", srtf_avg_wait, srtf_avg_turn);
        },
        Err(e) => {
            eprintln!("Failed to read tasks from {}: {}", tasks_file_path, e);
        }
    }
}