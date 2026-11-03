# tcp-proxy

A small Rust TCP proxy that forwards bytes in both directions for one client
connection. It uses blocking I/O, an additional request thread, and the main
thread for responses.

The current destination is `127.0.0.1:8001`. The included
[EOF-waiting backend](examples/eof_backend.rs) is an experimental Rust server
that sends `EOF received\n` only after it receives EOF from the proxy.

```text
test client <-> proxy :9000 <-> EOF-waiting backend :8001
```

## Requirements and verification

- Rust and Cargo; checked with version 1.96.0.
- Python 3 for the short test client below; no extra Python packages are needed.
- Run all processes on the same host. When using a Linux VM, run the client in
  that VM too, since `127.0.0.1` is local to each host.
- The learner verified HTTP forwarding and the EOF-triggered response on
  Ubuntu 24.04.5 LTS / aarch64. The earlier HTTP check used a destination on
  port 8000; the committed code now targets port 8001.
- AI separately verified the backend on macOS arm64 with empty, six-byte, and
  2000-byte inputs. That backend-only check is separate from proxy verification.
- Before publishing this checkpoint, AI also ran the README client through the
  current proxy on macOS arm64: the six-byte request produced the expected
  response and EOF, and both Rust programs exited with code 0.

## Start the backend and proxy

In terminal 1, from this package directory:

```bash
cargo run --example eof_backend -j1
```

Wait for `待ち受け中: 127.0.0.1:8001`.

In terminal 2, from the same package directory:

```bash
cargo run --bin tcp-proxy -j1
```

The proxy listens on `127.0.0.1:9000`. It prints no startup message while waiting
for its client. Its destination stream is printed after the client connects.

Both programs accept only one connection. Restart both before repeating the
demo. A connection probe such as `nc -vz` also consumes that one connection.

## Send a request

In terminal 3 on the same host:

```bash
python3 - <<'PY'
import socket

with socket.create_connection(("127.0.0.1", 9000), timeout=5) as client:
    client.sendall(b"hello\n")
    # Finish sending while keeping the receiving direction open.
    client.shutdown(socket.SHUT_WR)

    response = b""
    while True:
        data = client.recv(1024)
        if not data:
            break
        response += data

    print(f"response: {response!r}")
PY
```

Expected output:

```text
response: b'EOF received\n'
```

The backend logs six received bytes, then EOF, then the response write. The
client finishes after receiving EOF, and the proxy returns to its prompt after
joining its request thread. The backend also exits.

This backend uses a raw TCP exchange rather than HTTP. An ordinary `curl`
request does not close its sending direction after its HTTP headers, so it
cannot trigger this backend's EOF-based response.

## How forwarding and EOF work

| Execution context | Reads from | Writes to |
| --- | --- | --- |
| Additional thread | `stream` (client) | `upstream` (destination) |
| Main thread | `upstream_reader` (destination) | `client_writer` (client) |

`try_clone()` creates another owned handle to the same TCP connection; it does
not establish another connection. The original handles move into the request
thread, and the cloned handles remain in the main thread.

Each direction reuses a 1024-byte application buffer. A successful read can
return fewer bytes than its capacity. `write_all(&buffer[..n])` writes the bytes
that were read. The two arrays do not represent total process or kernel memory.

On EOF, each direction calls `shutdown(Shutdown::Write)` on its destination:
client EOF finishes sending to the backend, and backend EOF finishes sending
to the client. This preserves the opposite direction for any remaining data.
The main thread calls `join()` after its response loop.

## Checks

```bash
cargo fmt -- --check
cargo build --all-targets --locked -j1
cargo test --locked -j1
```

No Rust unit tests or CI workflow have been added. `cargo test` currently runs
zero tests. The main binary has two warnings for the unused `std::error` import
and `peer_addr` binding. They are retained in this learning checkpoint.

For the learner's observations, AI support, and limits of the evidence, see the
[half-close experiment](../docs/experiments/003-tcp-half-close.md). The earlier
[echo experiment](../docs/experiments/002-tcp-single-read.md) describes a previous
implementation, not the current forwarding behavior.

## Current limitations

- One client connection per run, fixed addresses, and no reconnect or retry.
- No HTTP parsing, TLS termination, or message framing.
- No timeout or coordinated cancellation. An I/O error in one direction can
  leave the other blocked in `read`, `write_all`, or `join`.
- A peer that never finishes sending can keep the program waiting indefinitely.
  The demo client's five-second timeout is not a proxy timeout.
- I/O errors are printed; they are not propagated out of the worker. A
  successful `join()` means no thread panic, not successful data transfer.
- Operational errors are printed to stdout and do not set a nonzero exit code.
- Reset connections, stalled peers, and thread-start failures have not been
  tested. The comparison with EOF forwarding disabled is still pending.
- Current checks do not establish throughput, total memory use, or arbitrary
  payload integrity through the proxy. The test backend discards request bytes.
