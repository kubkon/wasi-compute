# test-compute

`test-compute` verifies that the preopened WASI file descriptors passed in as arguments to the
`compute` function can *only* be read from in case of the `in` file descriptor, and written to
in case of the `out` file descriptor.

In WASI terms, this example asserts that `in` contains [`rights::fd_read`] *only*, and that `out`
contains [`rights::fd_write`] *only*.

[`rights::fd_read`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#fd_read
[`rights::fd_write`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#fd_write

## Building

In order to build this example, you need:

* latest version of Rust nightly (I strongly recommend using [rustup] to manage your Rust
  installation)
* `cargo-wasi` plugin to `cargo`

(See the root of this [repo] for detailed instruction of how to get the required setup.)

[rustup]: https://rustup.rs

The steps required to build this example are then as follows:

```
cd flite-compute
cargo +nightly wasi build --release
```

We're using `+nightly` channel so that we use the latest WASI ABI snapshot, and at the time of
writing, that would be `snapshot1`.

And that's it!

## Running

In order to invoke the exported function, you'll need a slightly tweaked version of the 
`wasmtime` runtime available in [my fork] under the `preopen_fd` branch.

(Again, see the root of this [repo] for detailed instruction of how to build the required
version of `wasmtime`.)

[my fork]: https://github.com/kubkon/wasmtime/tree/preopen_fd
[repo]: https://github.com/kubkon/wasi-compute

Assuming you've build `wasmtime` OK, you can now invoke `compute` from this example as follows:

```
wasmtime --preopen_read=11:in --preopen_write=12:out \
         --invoke=compute target/wasm32-wasi/release/test_compute.wasm -- 11 12
```

In this example, it is immaterial what `in` contains, etc., as long as it exists on the host
since we only verify that the preopened WASI file descriptors contain only the required rights:
not more, not less.