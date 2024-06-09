# Task Scheduler
This project is a task scheduler implemented in Rust. It supports three scheduling algorithms: First-Come, First-Served (FCFS), Round Robin (RR), and Shortest Remaining Time First (SRTF). The scheduler reads task information from an input file and outputs the scheduling results and statistics.

## Features
- First-Come, First-Served (FCFS): Tasks are processed in the order they arrive.
- Round Robin (RR): Tasks are processed in a cyclic order, with a specified time quantum.
- Shortest Remaining Time First (SRTF): Tasks with the shortest remaining processing time are processed first.
## Input File
- The input file should be named input.txt and located in the same directory as the executable. Each line in the file should contain three values separated by spaces:
- Process ID (PID)
- Arrival Time
- Burst Time
# Task Scheduler

This project is a task scheduler implemented in Rust. It supports three scheduling algorithms: First-Come, First-Served (FCFS), Round Robin (RR), and Shortest Remaining Time First (SRTF). The scheduler reads task information from an input file and outputs the scheduling results and statistics.

## Features

- **First-Come, First-Served (FCFS)**: Tasks are processed in the order they arrive.
- **Round Robin (RR)**: Tasks are processed in a cyclic order, with a specified time quantum.
- **Shortest Remaining Time First (SRTF)**: Tasks with the shortest remaining processing time are processed first.

## Input File

The input file should be named `input.txt` and located in the same directory as the executable. Each line in the file should contain three values separated by spaces:

1. Process ID (PID)
2. Arrival Time
3. Burst Time

Example:
```
1 0 5
2 2 3
3 4 1
```

## Installation

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone the repository:
   ```sh
   git clone https://github.com/maryamjbr/cpu_scheduling.git
   cd cpu_scheduling
   ```
3. Build the project:
   ```sh
   cargo init
   cargo build --release
   ```

## Usage

1. You can prepare your own `input.txt` file with task information.
2. Run the scheduler:
   ```sh
   cargo run --release
   ```

## Example Output

The program outputs the start and finish times of each task for each scheduling algorithm, along with the average waiting and turnaround times.

### FCFS Scheduling Results

```
Time 0: Task 1 starts
Time 5: Task 1 finishes
Time 5: Task 2 starts
Time 8: Task 2 finishes
Time 8: Task 3 starts
Time 9: Task 3 finishes
Average Waiting Time: 3.67
Average Turnaround Time: 7.00
```

### Round Robin Scheduling Results

```
Time 0: Task 1 starts
Time 2: Task 1 finishes
Time 2: Task 2 starts
Time 4: Task 2 finishes
Time 4: Task 3 starts
Time 5: Task 3 finishes
Average Waiting Time: 2.33
Average Turnaround Time: 5.67
```

### SRTF Scheduling Results

```
Time 0: Task 1 starts
Time 5: Task 1 finishes
Time 5: Task 2 starts
Time 8: Task 2 finishes
Time 8: Task 3 starts
Time 9: Task 3 finishes
Average Waiting Time: 3.67
Average Turnaround Time: 7.00
```
