# linux-job-agent

A small CLI tool that runs a command as a child process, waits for it to finish,
and returns the child's exit code to the caller.

This is the first working version of a Rust project for learning Linux process
management. It currently runs one command at a time.

## Build and run

Install Rust and Cargo, then run these commands from the directory containing
`Cargo.toml`:

```bash
cargo build
./target/debug/linux-job-agent /bin/sh -c 'exit 7'
echo $?
```

Expected output from `echo $?`:

```text
7
```

The example starts a shell that exits with code 7. The agent waits for that child
and returns the same code to its caller.

Run `echo $?` immediately after the agent, on a separate line. It displays the
exit code of the command that just finished.

Usage:

```text
linux-job-agent <command> [args...]
```

For example:

```bash
./target/debug/linux-job-agent /bin/echo hello world
echo $?
```

Expected output:

```text
hello world
0
```

## Exit codes

- When the child exits with an exit code, the agent returns that same code.
- If the command cannot be started, the agent prints an error and currently
  returns 1. Errors while waiting for the child also return 1.
- If no command is provided, the agent prints a usage message and returns 1.
- If the child terminates without an exit code, such as through a terminating
  signal, the agent currently returns 1.

The following cases have been manually checked on macOS and Ubuntu. Run each command with
`./target/debug/linux-job-agent` and check its result using `echo $?`:

| Command and arguments | Agent exit code |
| --- | --- |
| `/bin/echo hello world` | 0 |
| `/usr/bin/false` | 1 |
| `/bin/sh -c 'exit 7'` | 7 |
| `/no/such/command` | 1, with an error message |

Testing code 7 as well as code 1 checks that the agent preserves the child's
exit code rather than converting every failure to 1.

## Verified environment

| Environment | Architecture | Rust | Cargo |
| --- | --- | --- | --- |
| macOS | arm64 | 1.96.0 | 1.96.0 |
| Ubuntu 24.04.5 LTS on Azure | aarch64 (Arm64) | 1.96.0 | 1.96.0 |

The Ubuntu VM uses `Standard_B2pts_v2` with 2 vCPUs and 1 GiB of memory.
The recorded Linux kernel is `7.0.0-1017-azure`.

## Current limitations

- Child exit code 1 and an agent error both produce exit code 1, so the exit code
  alone does not distinguish them.
- Signal termination is mapped to 1; the original signal is not preserved.
- Error and usage messages currently go to standard output.
- The current version has no timeout, cancellation, or resource limits.
