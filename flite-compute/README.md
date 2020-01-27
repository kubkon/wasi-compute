# flite-compute

`flite-compute` demonstrates how to plug in an existing library, in this case text-to-spech (TTS) [flite],
into the `wasi-compute` experimental framework which allows for deterministic computations
in WASI.

[flite]: http://festvox.org/flite/index.html

In this example, we use `flite` purely as a TTS engine and create a deterministic `compute` function
as described in the main [README] which takes in two WASI file descriptors: `in` can point to any text
file which we'd like to convert to speech using `flite` and save it as `wav` pointed to as `out` WASI
file descriptor.

[README]: https://github.com/kubkon/wasi-compute

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

Note that for your convenience I've already precompiled parts of `flite` required by this example
and put them inside `lib/` dir.

## Running

In order to invoke the exported function, you'll need a slightly tweaked version of the 
`wasmtime` runtime available in [my fork] under the `preopen_fd` branch.

(Again, see the root of this [repo] for detailed instruction of how to build the required
version of `wasmtime`.)

[my fork]: https://github.com/kubkon/wasmtime/tree/preopen_fd
[repo]: https://github.com/kubkon/wasi-compute

Assuming you've build `wasmtime` OK, you can now invoke `compute` from this example as follows:

```
wasmtime --preopen_read=11:some_input.txt --preopen_write=12:some_output.wav \
         --invoke=compute target/wasm32-wasi/release/flite_compute.wasm -- 11 12
```