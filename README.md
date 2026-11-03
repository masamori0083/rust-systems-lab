# rust-systems-lab

Hands-on Rust experiments in process management, networking, and Linux systems.
Each experiment starts with a small working program, then explores its behavior,
design choices, and failure cases.

## Experiments

| Project | What it does | Current status |
| --- | --- | --- |
| [linux-job-agent](linux-job-agent/) | Runs a command as a child process, waits for it to finish, and forwards its exit code | Minimal CLI verified on macOS and Ubuntu Arm64 |
| [tcp-proxy](tcp-proxy/) | Forwards bytes in both directions between a client and a fixed destination, using blocking I/O and two threads | HTTP forwarding and an EOF-triggered response verified on Ubuntu Arm64 |

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

## Try the TCP proxy

The current destination is an experimental Rust server on port 8001. It waits
for request EOF before returning a fixed response. Run all three terminals on
the same machine; each `127.0.0.1` address refers to that machine.

Terminal 1, from the repository root:

```bash
cd tcp-proxy
cargo run --example eof_backend -j1
```

Terminal 2, also from the repository root:

```bash
cd tcp-proxy
cargo run --bin tcp-proxy -j1
```

In terminal 3, run the send-side shutdown client from the
[TCP proxy README](tcp-proxy/README.md#send-a-request). It sends `hello\n`,
closes only its sending direction, and reads the response until EOF.

Expected response: `b'EOF received\n'`. Both Rust programs handle one connection
per run and must be restarted for another request. The current backend is a raw
TCP test server, so ordinary HTTP requests with `curl` are not this demo's client.

## Design notes

- [Forwarding child-process exit codes](docs/decisions/001-exit-code.md)
  (Japanese): alternatives, observed results, and current limitations.
- [Blocking I/O with one thread per direction](docs/decisions/002-bidirectional-io.md)
  (Japanese): the selected design, handle ownership, and remaining limitations.

## Experiment notes

- [Process creation, waiting, and exit](docs/experiments/001-process-lifecycle.md)
  (Japanese): Linux observations using ps, /proc, and strace.
- [The limit of a single TCP read](docs/experiments/002-tcp-single-read.md)
  (Japanese): the earlier echo exercise, prediction, and before/after results.
- [Forwarding EOF without losing the response](docs/experiments/003-tcp-half-close.md)
  (Japanese): the EOF-waiting backend, reproduction steps, and observed results.

## Verification

From the repository root:

```bash
cargo fmt --manifest-path linux-job-agent/Cargo.toml -- --check
cargo test --manifest-path linux-job-agent/Cargo.toml --locked
cargo fmt --manifest-path tcp-proxy/Cargo.toml -- --check
cargo build --manifest-path tcp-proxy/Cargo.toml --all-targets --locked -j1
cargo test --manifest-path tcp-proxy/Cargo.toml --locked
```

No Rust unit tests are defined yet. The command runner has been manually checked
with successful commands, nonzero exit codes (1 and 7), and a missing executable.
The commands and expected results are documented in the project README.
The current TCP proxy has been manually checked on Ubuntu with an HTTP response
(using the earlier destination setting, port 8000) and with a six-byte request
followed by a send-side shutdown (using the current port 8001). The experiment
notes distinguish learner runs from AI support and record untested conditions.
The earlier 2000-byte echo check is historical evidence, not a test of the current
proxy's payload integrity. Two unused-item warnings remain in the proxy.

This repository contains learning experiments. Features such as timeouts,
cancellation, and resource limits are not implemented in the current job agent.
