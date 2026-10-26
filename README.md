# CS 411 Lab 1 The Render Deadline starter

The full Spring 2027 assignment is `Lab_01_The_Render_Deadline.md`. This Rust starter supplies the sequential prime-sum baseline and the benchmark and plot runners. Complete only the two TODO functions in `src/lib.rs`; the public tests check boundary limits, non-even chunks, Amdahl values, and a bounded fraction estimate.

From the repository root, with a current Rust toolchain:

```text
cargo test
cargo run --bin speedup -- --limit 200000
cargo run --release --bin speedup
cargo run --release --bin plot
```

The unchanged starter compiles, but the tests and runners stop at the intentional TODO panic. For an initial short check, use the `--limit 200000` command. The release runs use a larger default workload and will take longer. The benchmark asks for your USI username before printing a token; the token is emitted only after all parallel results equal the sequential baseline.

Save the analysis, plot, and terminal evidence specified in the handout. Do not edit the expected values in `tests/verification.rs`.

## Work in your own GitHub repository

The instructor starter is an upstream source. Keep your completed work in a repository under your own GitHub account, then submit the repository URL and the evidence listed in the assignment handout through Blackboard.

If Blackboard links a template repository, choose **Use this template**, create the new repository under your account, and clone that new repository. Check `git remote -v` before pushing; `origin` must be your repository.

For a direct clone, first create an empty repository under your account. Replace the placeholders below with the starter link supplied in Blackboard and your own empty repository URL:

```sh
git clone <instructor-starter-url>
cd <cloned-directory>
git remote rename origin upstream
git remote add origin <your-empty-repository-url>
git remote -v
git push -u origin HEAD
```

Commit and push your completed work to your own `origin`. Do not push to the instructor's upstream repository. Use the exact submission files and verification commands in the assignment handout.
