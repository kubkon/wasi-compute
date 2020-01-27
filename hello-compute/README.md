# hello-compute

`hello-compute` contains the minimal useful example of how to build your own `compute` function. It
reads in whatever was passed in from the `in` WASI file descriptor, raises all characters to uppercase,
and outputs the result to the `out` WASI file descriptor

## Building

In order to build this example, you need:

* latest version of Rust nightly (I strongly recommend using [rustup] to manage your Rust
  installation)
* `cargo-wasi` plugin to `cargo`

(See the root of this [repo] for detailed instruction of how to get the required setup.)

[rustup]: https://rustup.rs

The steps required to build this example are then as follows:

```
cd hello-compute
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
wasmtime --preopen_read=11:some_input.txt --preopen_write=12:some_output.txt \
         --invoke=compute target/wasm32-wasi/release/hello_compute.wasm -- 11 12
```