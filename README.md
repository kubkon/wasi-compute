# wasi-compute

`wasi-compute` explores the minimal viable model for fully deterministic
computations/functions in [WASI]. The explored model is a direct result
of discussions found in [WebAssembly/WASI/issues/190] thread.

[WASI]: https://wasi.dev
[WebAssembly/WASI/issues/190]: https://github.com/WebAssembly/WASI/issues/190

## The model

![The model](/images/the_model.png)

### Examples of "compute" function in different languages
* Wasm:

```wasm
(func (export "compute") (param u32) (param u32))
```

* Rust:

```rust
fn compute(r#in: wasi::Fd, out: wasi::Fd);
```

* C/C++:

```c
void compute(__wasi_fd_t in, __wasi_fd_t out);
```

## Examples

Fully working examples demonstrating the viability of the proposed approach can
be found in this repo, and are as follows:

* [hello-compute] - demonstrates a minimal "Hello World!"-style function which reads
    from the input WASI file descriptor, makes whatever it read upper-case,
    and writes the result to the output WASI file descriptor

* [test-compute] - verifies that *only* reading from/writing to a specified, preopened
    WASI file descriptor are possible

* [flite-compute] - demonstrates that it is already possible to fit a full-fledged
    library into this model by taking a text-to-speech [flite] engine
    and performing simple TTS on the input WASI file descriptor and
    saving the result to the output WASI file descriptor

[hello-compute]: hello-compute
[test-compute]: test-compute
[flite-compute]: flite-compute
[flite]: https://festvox.org/flite/index.html

**NOTE:** all of the examples contained within this repo require a tweaked version
of the [`wasmtime`] runtime which can be found in my fork [kubkon/wasmtime/tree/preopen_fd]. Therefore, in order to run the examples, you'll need to clone the repo
and build it using the latest version of Rust:

```
git clone https://github.com/kubkon/wasmtime
cd wasmtime
git checkout preopen_fd
cargo build --release
```

[`wasmtime`]: https://wasmtime.dev
[kubkon/wasmtime/tree/preopen_fd]: https://github.com/kubkon/wasmtime/tree/preopen_fd

## Disclaimer

All of the examples presented here are highly experimental in nature and should
not be relied upon in any shape or form, and by using them you agree to use them
at your own risk!

