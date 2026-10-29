# rust-systems-lab

Hands-on Rust experiments in process management, networking, and Linux systems.
Each experiment starts with a small working program, then explores its behavior,
design choices, and failure cases.

## Experiments

| Project | What it does | Current status |
| --- | --- | --- |
| [linux-job-agent](linux-job-agent/) | Runs a command as a child process, waits for it to finish, and forwards its exit code | Minimal CLI verified on macOS and Ubuntu Arm64 |
| [tcp-proxy](tcp-proxy/) | Accepts one TCP connection and echoes received bytes using a reusable buffer | Echo exercise verified on macOS arm64; upstream forwarding is the next step |

Both projects are independent Cargo packages. Run Cargo commands inside the
project directory or specify its manifest path.

## Try the job agent

Requires Rust and Cargo. The current code has been checked with version 1.96.0
on macOS arm64 and Ubuntu aarch64.

From the repository root:

```bash
cd linux-job-agent
cargo build
./target/debug/linux-job-agent /bin/echo hello world
echo $?
```

Expected output from the last two commands:

```text
hello world
0
```

For more examples, exit-code behavior, and current limitations, see the
[job agent README](linux-job-agent/README.md).

## Try the TCP echo exercise

In one terminal on the same machine:

```bash
cd tcp-proxy
cargo run
```

In another terminal:

```bash
printf 'hello\n' | nc 127.0.0.1 9000
```

The client receives `hello` followed by a newline. The server handles one
connection and exits after EOF or an I/O error. Restart it for another run.
See the [TCP experiment README](tcp-proxy/README.md) for the 2000-byte check,
platform verification, and current limitations.

## Design notes

- [Forwarding child-process exit codes](docs/decisions/001-exit-code.md)
  (Japanese): alternatives, observed results, and current limitations.

## Experiment notes

- [Process creation, waiting, and exit](docs/experiments/001-process-lifecycle.md)
  (Japanese): Linux observations using ps, /proc, and strace.
- [The limit of a single TCP read](docs/experiments/002-tcp-single-read.md)
  (Japanese): prediction, reproduction, design choice, and before/after results.

## Verification

From the repository root:

```bash
cargo fmt --manifest-path linux-job-agent/Cargo.toml -- --check
cargo test --manifest-path linux-job-agent/Cargo.toml --locked
cargo fmt --manifest-path tcp-proxy/Cargo.toml -- --check
cargo test --manifest-path tcp-proxy/Cargo.toml --locked
```

No Rust unit tests are defined yet. The command runner has been manually checked
with successful commands, nonzero exit codes (1 and 7), and a missing executable.
The commands and expected results are documented in the project README.
The TCP experiment has been manually checked with a short message and a
2000-byte payload. The experiment notes distinguish learner runs from AI
support and record what has not been verified.

This repository contains learning experiments. Features such as timeouts,
cancellation, and resource limits are not implemented in the current job agent.
