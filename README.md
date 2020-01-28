# wasi-compute

`wasi-compute` explores the minimal viable model for fully deterministic
computations/functions in [WASI]. The explored model is a direct result
of discussions found in [WebAssembly/WASI/issues/190] thread.

[WASI]: https://wasi.dev
[WebAssembly/WASI/issues/190]: https://github.com/WebAssembly/WASI/issues/190

## The model

![The model](/images/the_model.png)

The model is pretty simple in its assumptions. The "compute" function takes in two
arguments: an input WASI file descriptor, `in`, and an output WASI file descriptor,
`out`. Thanks to the capabilities-based security model of WASI, we are able to enforce
`in` to only have [`rights::fd_read`] rights attached to it, and `out` to only have
[`rights::fd_write`] rights. This is handled at the preopening stage of the resources
in the runtime proper (in this case, the modified version of `wasmtime`;
see [Running the examples using `wasmtime`] section).

Constraining the descriptors to reading from or writing to immediately shields us 
from any nondeterminism that might arise from utilising any syscalls operating on
raw file descriptors or paths in WASI. In fact, the only permitted syscalls are

* [`fd_read`]
* [`fd_write`]
* [`fd_close`]

Of course, other syscalls not requiring file descriptors as arguments such as
[`random_get`] or [`clock_time_get`] can still be invoked in this model; however,
with the upcoming WASI snapshot (see [ephemeral] stage in WASI repo), these syscalls
will get their own WASI modules, and hence they will require a capability to be
available at run time. Here, in order to emulate this, I've gone ahead and filtered
out those offending syscalls manually in my tweaked version of `wasmtime` (see
[Running the examples using `wasmtime`]).

[`fd_read`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#fd_read
[`fd_write`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#fd_write
[`fd_close`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#fd_close
[`random_get`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#random_get
[`clock_time_get`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#clock_time_get
[ephemeral]: https://github.com/WebAssembly/WASI/blob/master/phases/ephemeral/docs.md
[Running the examples using `wasmtime`]: #running-the-examples-using-wasmtime

### Examples of "compute" function in different languages
* Wat:

```wat
(func (export "compute") (param $in u32) (param $out u32))
```

* Rust:

```rust
#[no_mangle]
pub extern "C" fn compute(r#in: wasi::Fd, out: wasi::Fd);
```

* C/C++:

```c
void compute(__wasi_fd_t in, __wasi_fd_t out);
```

### Building the examples using `cargo-wasi`

In order to build the examples contained within this repo, you'll need two things:

* latest version of Rust nightly (I strongly recommend using [rustup] to manage your Rust
  installation) -- stable and beta channels are fine as well however, at the time of writing,
  they do not guarantee you'll be using the latest WASI ABI snapshot, aka [`snapshot1`]
* `cargo-wasi` plugin to `cargo`

[rustup]: https://rustup.rs
[`snapshot1`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md

The steps are thus as follows:

```
rustup toolchain add nightly
cargo install cargo-wasi
cargo +nightly wasi build --release
```

When executed from the root of this repo, these 3 steps will build all examples targetting
the `wasm32-wasi` target in release mode. Note that you don't have to install the target
yourself using `rustup` as `cargo-wasi` will do it for you when you build the project for
the first time.

### Running the examples using `wasmtime`

All of the examples contained within this repo require a tweaked version
of the [`wasmtime`] runtime which can be found in my fork [kubkon/wasmtime/tree/preopen_fd].
Therefore, in order to run the examples, you'll need to clone the repo and build it using the
latest version of Rust:

```
git clone https://github.com/kubkon/wasmtime
cd wasmtime
git checkout preopen_fd
cargo build --release
```

[`wasmtime`]: https://wasmtime.dev
[kubkon/wasmtime/tree/preopen_fd]: https://github.com/kubkon/wasmtime/tree/preopen_fd

The tweaked version of the runtime adds two optional arguments to `wasmtime` which are
required in order to map a WASI file descriptor to the preopened resource on the host.
There are:

* `--preopen_read=GUEST_FD:PATH_TO_PREOPEN` which will preopen and map a resource
    opened for *reading* only, or in WASI terms, with rights set to [`rights::fd_read`]

* `--preopen_write=GUEST_FD:PATH_TO_PREOPEN` which will preopen and map a resource
    opened for *writing* only, or in WASI terms, with rights set to [`rights::fd_write`]

[`rights::fd_read`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#fd_read
[`rights::fd_write`]: https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#fd_write

The examples and any code conforming to the discussed model can then be run using
the following `wasmtime` invocation:

```
wasmtime --preopen_read=READ_FD:PATH_TO_PREOPEN \
    --preopen_write=WRITE_FD:PATH_TO_PREOPEN \
    --invoke=compute \
    example.wasm -- READ_FD WRITE_FD

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

## Disclaimer

All of the examples presented here are highly experimental in nature and should
not be relied upon in any shape or form, and by using them you agree to use them
at your own risk!

