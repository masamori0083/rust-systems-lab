# tcp-proxy

A Rust learning experiment that will grow into a TCP proxy. The current version
is a TCP echo server: it accepts one connection, reads bytes into a reusable
1024-byte buffer, and writes the bytes back to the same client.

## Requirements and verification

- Rust and Cargo; checked with version 1.96.0 on macOS arm64.
- `nc` for the manual client commands below.
- This project has not yet been built or run on Linux.

## Run

From this directory, in terminal 1:

```bash
cargo run
```

The server listens on `127.0.0.1:9000` and waits for one connection. No startup
message is printed. The current code may emit an unused-variable warning for
`peer_addr`; the address is not used in the echo exercise.

In terminal 2 on the same machine:

```bash
printf 'hello\n' | nc 127.0.0.1 9000
```

The client receives `hello` and a newline. The server stops reading when it sees
EOF or an I/O error, then closes the connection as the stream leaves scope.
Restart the server before each example: it accepts only one connection per run.

## Check a payload larger than the buffer

Restart the server in terminal 1, then run in terminal 2:

```bash
head -c 2000 /dev/zero | nc 127.0.0.1 9000 | wc -c
```

The learner observed 1024 returned bytes with a single `read`, then 2000 returned
bytes after adding a read/write loop. The loop reuses the same array and writes
only the bytes read, using `write_all(&buffer[..n])`.

The per-call read size is not guaranteed. The 2000-byte count check measures
returned length; it does not compare the payload contents or order. See the
[experiment record](../docs/experiments/002-tcp-single-read.md) for evidence,
design reasoning, and limits of the measurement.

## Checks

```bash
cargo fmt -- --check
cargo build --locked
cargo test --locked
```

No Rust unit tests or CI workflow have been added. Runtime I/O error branches
exist but have not been verified by fault injection.

Before publishing this checkpoint, AI-assisted verification on macOS also
compared the echoed contents for a 6-byte message and a 2000-byte binary payload,
and checked that empty input followed by a send-side shutdown ended the server.
All three checks passed. The temporary client was outside the repository; these
checks are recorded in the experiment notes and are not a committed test suite.

## Current scope

- One blocking TCP connection per run, bound to the local machine.
- One 1024-byte application buffer, reused across reads.
- EOF ends the read loop; read/write errors are printed and end the loop.
- Operational errors are printed to stdout; the program does not set a nonzero
  exit status for those errors.
- No upstream connection, HTTP processing, timeout, or concurrent clients yet.

The next step is to connect to a test HTTP server and forward bytes between two
TCP connections.
