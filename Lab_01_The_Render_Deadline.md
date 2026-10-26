# Lab 1: The Render Deadline
## The Speedup Calculator

**CS 411: Computing Paradigms — Spring 2027**  
**Week 1: Concurrency, Parallelism, and Amdahl’s Law**  
**Due:** January 17, 2027  
**Value:** 100 points

## The mission

The render queue is growing and the deadline is not moving. Adding workers may reduce the parallel portion of the job, but setup, coordination, and final assembly still happen in sequence. Your task is to determine what more workers can actually buy—and where the speedup stops.

You will first analyze the system with Amdahl’s Law. Then you will implement and benchmark a parallel Rust workload, compare measured performance with the model, and explain the gap between prediction and observation.

## Where this fits in the course map

This lab is the evidence-producing end of the Week 1 learning sequence.

| Course-map objective | Week 1 preparation | Evidence in this lab |
| --- | --- | --- |
| **1.1** Differentiate concurrency from parallelism | VoiceThread 1 and the Week 1 slides | Part A, Question 1; explanation of the benchmark design |
| **1.2** Apply Amdahl’s Law to theoretical speedup limits | Week 1 worked traces | Part A, Questions 2–3; predicted-speedup column and plot |
| **1.3** Identify serial and parallelizable work | Kleppmann Ch. 1 scalability reading and the render-deadline model | Part B decomposition, measured sequential fraction, and reflection |

The Week 1 slides give you the model. The lab asks whether the model survives contact with a real machine.

## Before you begin

1. Complete **VoiceThread 1: Concurrency, Parallelism & Amdahl’s Law**.
2. Read the assigned scalability material from Kleppmann, Chapter 1.
3. Review the Week 1 slides, especially the render pipeline, the fixed-work assumption, and the worked `s = 0.25` trace.
4. Open the **Spring 2027 Lab 1 template link posted in Blackboard** and create your own repository from it. Do not use the archived Fall 2026 organization link.
5. Clone your repository and run the unchanged starter:

   ```text
   cargo test
   cargo run --bin speedup -- --limit 200000
   ```

   The starter is intentionally incomplete. A failure from the two TODO functions is expected before you implement them.

## Part A — Model the limit (40 points)

Answer in `Lab1_Analysis.md` or `Lab1_Analysis.pdf`. Show your work.

### 1. Concurrency is not parallelism (8 points)

Define **concurrency** and **parallelism** in your own words. Give one example of a system that is concurrent but not parallel, and explain why the distinction matters before adding threads.

### 2. Revisit the render deadline (12 points)

The Week 1 render pipeline has sequential fraction `s = 0.25`.

Use

`Speedup(N) = 1 / (s + (1 - s) / N)`

to compute the speedup for `N = 2`, `N = 8`, and `N = 32`. Show each substitution. Then compute the theoretical maximum as `N → ∞` and explain the ceiling in one sentence.

### 3. Reverse the model (12 points)

A team wants a fixed-size image pipeline to achieve a speedup of `6×` on `16` cores. Solve algebraically for the largest sequential fraction `s` that makes the target possible. Show the algebra, then state whether the target is plausible and what engineering change would matter most.

### 4. Prediction check (8 points)

Before running your completed benchmark, predict which thread count—`1`, `2`, `4`, or `8`—will deliver the best measured speedup on your machine. Give one reason the measured curve may fall below Amdahl’s prediction.

## Part B — Measure the machine (50 points)

The starter is a Cargo project with a CPU-bound baseline, `sequential_sum_of_primes`. Complete the TODOs in `src/lib.rs`; the two binaries are already wired to your functions.

### 1. Implement the parallel workload (18 points)

Implement:

```rust
pub fn parallel_sum_of_primes(limit: u64, num_threads: usize) -> u64
```

Requirements:

- divide `[2, limit)` into contiguous chunks;
- use the requested number of Rust threads;
- sum each chunk independently;
- join the workers and reduce the partial sums;
- return exactly the same result as the sequential baseline; and
- handle limits that do not divide evenly across the thread count.

### 2. Estimate the serial fraction (10 points)

Implement:

```rust
pub fn measure_sequential_fraction(limit: u64) -> f64
```

Use the starter comments as a reasonable measurement strategy. In `Lab1_Analysis`, identify what your estimate treats as serial work and acknowledge why it is only an estimate.

### 3. Benchmark and verify (12 points)

Run:

```text
cargo test
cargo run --release --bin speedup
```

The benchmark must report `1`, `2`, `4`, and `8` threads with time, measured speedup, and Amdahl-predicted speedup. All parallel results must match the sequential result. Save the full terminal output as `Lab1_Terminal.txt`.

### 4. Generate the plot (10 points)

Run:

```text
cargo run --release --bin plot
```

Submit the generated `speedup_plot.png`. It must show both the measured and Amdahl-predicted curves with labeled axes and a legend.

## Part C — Explain the evidence (10 points)

Add a **Measured vs. Predicted** section to `Lab1_Analysis` containing:

1. your measured speedup at `8` threads;
2. the Amdahl-predicted value at `8` threads;
3. the size and direction of the difference; and
4. two concrete causes of that difference, tied to your implementation or machine.

End with a recommendation: if the team must reduce runtime further, should it add workers, reduce serial work, increase the workload, or change the algorithm? Defend the choice in 2–3 sentences.

## Verification checklist

Before submitting, verify all of the following:

- `cargo test` passes the public boundary and model checks. These tests cover representative cases, so also inspect the benchmark results and explain any anomaly.
- Every parallel result equals the sequential baseline.
- The table contains rows for `1`, `2`, `4`, and `8` threads.
- The final run prints `SUCCESS TOKEN: AMDAHL-VERIFIED-<your-username>-<speedup>`.
- `speedup_plot.png` opens and contains two labeled curves.
- Your analysis shows the Amdahl substitutions and explains the measured/predicted gap.

Timing results vary by machine. Correctness, reproducibility, and the quality of your explanation matter more than achieving a particular speedup.

## What to submit

Submit **one repository URL** and the following files in the Blackboard **Lab 1: The Render Deadline** item:

1. `Lab1_Analysis.pdf` **or** `Lab1_Analysis.md`
2. `src/lib.rs`
3. `src/bin/speedup.rs`
4. `src/bin/plot.rs`
5. `Cargo.toml`
6. `speedup_plot.png`
7. `Lab1_Terminal.txt`

Paste the **Success Token** into the Blackboard text-entry field and also include it at the end of `Lab1_Analysis`.

Your repository URL must point to your own repository created from the Spring 2027 template. Do not submit the instructor template URL.

## Rubric

| Criterion | Points |
| --- | ---: |
| Concurrency/parallelism distinction | 8 |
| Amdahl calculations and ceiling | 12 |
| Reverse-model algebra and engineering judgment | 12 |
| Prediction | 8 |
| Correct parallel decomposition and reduction | 18 |
| Serial-fraction measurement and explanation | 10 |
| Benchmark correctness and evidence | 12 |
| Measured vs. predicted plot | 10 |
| Evidence-based interpretation and recommendation | 10 |
| **Total** | **100** |

## AI usage and academic integrity

AI tools may be used as IDE-like assistance unless Christian gives different instructions. If you use one, add an **AI Usage** section to `Lab1_Analysis` naming the tool and listing the prompts you used. If you do not use AI, write `AI Usage: None.` You are responsible for understanding and defending every line you submit.

## Late work

Late work is accepted for up to three days with a penalty of 25% per day, following the course syllabus.
