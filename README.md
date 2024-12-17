# failfast - `std::process::abort()` demonstration

This is a trivial Rust program, and test, that call `std::process::abort()`.

## What's the point?

To show how programs that run other programs, including test drivers, display results when a process is abruptly terminated in this way.

## Examples

### GNU/Linux

The current effect of `std::process::abort()` on GNU/Linux systems (and probably on Unix-like systems in general) is to raise `SIGABRT`:

```text
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/failfast`
Aborted (core dumped)

$ echo "$?"  # 134 - 128 = 6 (SIGABRT)
134
```

```text
$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/failfast-5e2237fc1bd80edb)

running 1 test
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `/home/ek/repos/failfast/target/debug/deps/failfast-5e2237fc1bd80edb` (signal: 6, SIGABRT: process abort signal)
```

```text
$ cargo nextest run
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
────────────
 Nextest run ID d8fa92fd-6664-46eb-8ce7-96b1f442ad42 with nextest profile: default
    Starting 1 test across 2 binaries
     SIGABRT [   0.107s] failfast test::failfast
──── STDOUT:             failfast test::failfast

running 1 test

  Cancelling due to test failure
────────────
     Summary [   0.108s] 1 test run: 0 passed, 1 failed, 0 skipped
     SIGABRT [   0.107s] failfast test::failfast
error: test run failed
```

### Windows

On Windows, as of this writing, `std::process::abort()` calls `std::intrinsics::abort()`, which currently appears equivalent to `__failfast(1282)` in a C or C++ program compiled with MSVC.

```text
> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\failfast.exe`
error: process didn't exit successfully: `target\debug\failfast.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
```

```text
> cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src\lib.rs (target\debug\deps\failfast-cb220fbd611e8336.exe)

running 1 test
error: test failed, to rerun pass `--lib`

Caused by:
  process didn't exit successfully: `C:\Users\ek\source\repos\failfast\target\debug\deps\failfast-cb220fbd611e8336.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
note: test exited abnormally; to see the full output pass --nocapture to the harness.
```

```text
> cargo nextest run
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
────────────
 Nextest run ID c6d87486-f0df-4697-bd60-321f03d8b6fb with nextest profile: default
    Starting 1 test across 2 binaries
       ABORT [   0.970s] failfast test::failfast
     Message [         ] code 0xc0000409: The system detected an overrun of a stack-based buffer in this application. This overrun could potentially allow a malicious user to gain control of this application. (os error 1282)
──── STDOUT:             failfast test::failfast

running 1 test

  Cancelling due to test failure
────────────
     Summary [   0.974s] 1 test run: 0 passed, 1 failed, 0 skipped
       ABORT [   0.970s] failfast test::failfast
     Message [         ] code 0xc0000409: The system detected an overrun of a stack-based buffer in this application. This overrun could potentially allow a malicious user to gain control of this application. (os error 1282)
error: test run failed
```

See [`STATUS_STACK_BUFFER_OVERRUN` doesn’t mean that there was a stack buffer overrun](https://devblogs.microsoft.com/oldnewthing/20190108-00/?p=100655) by Raymond Chen for further context on `__failfast`.

## License

[0BSD](LICENSE)
