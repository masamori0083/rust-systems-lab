# rust-systems-lab

Hands-on Rust experiments in process management, networking, and Linux systems.
Each experiment starts with a small working program, then explores its behavior,
design choices, and failure cases.

## Experiments

| Project | What it does | Current status |
| --- | --- | --- |
| [linux-job-agent](linux-job-agent/) | Runs a command as a child process, waits for it to finish, and forwards its exit code | Minimal CLI verified on macOS and Ubuntu Arm64 |

Further experiments will explore sockets and network I/O.

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

## Design notes

- [Forwarding child-process exit codes](docs/decisions/001-exit-code.md)
  (Japanese): alternatives, observed results, and current limitations.

## Verification

From the repository root:

```bash
cargo fmt --manifest-path linux-job-agent/Cargo.toml -- --check
cargo test --manifest-path linux-job-agent/Cargo.toml --locked
```

No Rust unit tests are defined yet. The command runner has been manually checked
with successful commands, nonzero exit codes (1 and 7), and a missing executable.
The commands and expected results are documented in the project README.

This repository contains learning experiments. Features such as timeouts,
cancellation, and resource limits are not implemented in the current job agent.
