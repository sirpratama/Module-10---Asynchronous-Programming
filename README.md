# Module 10 - Asynchronous Programming

This repository contains my work for Module 10: Asynchronous Programming.

## Tutorial 1: Timer

### Experiment 1.1: Original timer from the book

The first experiment implements the simple executor, spawner, task, and timer future from the Rust async book executor chapter. I changed the printed signature from the example text to `Rafi's Komputer`.

How to run:

```bash
cargo run -p timer
```

Expected output:

```text
Rafi's Komputer: howdy!
Rafi's Komputer: done!
```

### Experiment 1.2: Understanding how it works.

I added `println!("Rafi's Komputer: hey hey");` right after `spawner.spawn(...)`. The line is outside the async block, so it runs immediately while the async task has only been queued. When the executor starts, it polls the spawned task, prints `howdy!`, reaches `TimerFuture.await`, and returns `Poll::Pending`. After the timer thread sleeps for two seconds and wakes the task, the executor polls it again and the task prints `done!`.

Captured output:

```text
Rafi's Komputer: hey hey
Rafi's Komputer: howdy!
Rafi's Komputer: done!
```
