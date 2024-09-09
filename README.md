# CPU Scheduling Algorithms Simulator

This project is a CPU scheduling algorithms simulator implemented in Rust, created as part of a university "Operating Systems" course at Amirkabir University of Technology. It simulates three different scheduling algorithms: 
1. First-Come, First-Served (FCFS)
2. Round Robin (RR)
3. Shortest Remaining Time First (SRTF)

The simulator reads tasks from an input file, executes them according to the specified scheduling algorithms, and prints the execution timeline along with average waiting and turnaround times for each algorithm.

## Features

- Simulates three CPU scheduling algorithms.
- Handles idle CPU time when no tasks are available.
- Calculates and displays average waiting time and turnaround time for each algorithm.

## Getting Started

### Prerequisites

- Rust programming language installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

## How to Use

1. Clone the repository:  
   `   git clone https://github.com/AliGhasemi01/cpu-scheduling-simulator.git`

2. Open the project in your code editor.

### Input Format

The simulator expects an input file named `input.txt` in the same directory as the executable. The input file should contain the following information for each task:

```
pid arrival_time burst_time
```

Where:
- `pid`: A unique identifier for the process (integer).
- `arrival_time`: The time at which the process arrives in the system (integer).
- `burst_time`: The total time required by the process to complete its execution on the CPU (integer).

Each task should be on a new line. Here’s an example of how the `input.txt` file should look:

```
1 0 4
2 1 5
3 2 2
4 3 1
5 4 3
6 5 4
```

### Running the Simulator

To run the simulator, navigate to the project directory and execute the following command:

`cargo run`

This will compile the Rust code and run the simulator. The output will display the execution timeline for each algorithm, including when each task starts and finishes, as well as the average waiting and turnaround times.

### Example Output

When you run the simulator with the provided input, you might see output similar to the following:

```
FCFS (First Come First Serve) Output:
Time 0: Task 1 starts
Time 4: Task 1 finishes
Time 4: Task 2 starts
Time 9: Task 2 finishes
...
FCFS Average Waiting Time: 6.00, Average Turnaround Time: 9.17
RR (Round Robin) with time_quantum = 2
...
SRTF (Shortest Remaining Time First) Output:
...
SRTF Average Waiting Time: 4.16, Average Turnaround Time: 7.33
```
