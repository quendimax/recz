# `recz-bench`

This crate provides benchmarks for the `recz_*` crates.

## Issues with running `valgrind`

Some distributions (as CachyOS) use latest ISA (as AVX512) for **x86-64**
architecture for their packages. As `valgrind` does not support these new
instructions, running it on such distributions may result in errors. To work
around this, use the following command to run this benchmark:

```bash
RUSTFLAGS="-Ctarget-cpu=x86-64-v3" cargo bench --target=x86_64-unknown-linux-musl -p recz_bench
```
