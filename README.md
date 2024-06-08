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
