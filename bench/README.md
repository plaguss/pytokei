# Benchmark against tokei

This folder contains some scripts to test the execution times
of `tokei` and `pytokei` against the cpython repo (at 30 of October 2022).

## pytokei

The script runs a Dockerfile.

```bash
$ sh run_dockerfile_py.sh
```

runs the small test for `pytokei`, and prints at the end something like:

```console
...
==================
Timing python run:
------------------
20 loops, best of 5: 180 msec per loop
==================
```

## tokei

```bash
cargo install rust-script
```