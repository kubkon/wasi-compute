# flite-compute

`flite-compute` demonstrates how to plug in an existing library, in this case text-to-spech (TTS) [flite],
into the `wasi-compute` experimental framework which allows for deterministic computations
in WASI.

[flite]: http://festvox.org/flite/index.html

In this example, we use `flite` purely as TTS engine and create a deterministic `compute` function
which takes in two WASI file descriptors: `in` used *only* for reading from, and `out` used
*only* for writing to. `in` can point to any text file which we'd like to convert to speech
using `flite` and save it as `wav` pointed to as `out` WASI file descriptor.

For a more in-depth look at `wasi-compute` see the root of this [repo].

[repo]: https://github.com/kubkon/wasi-compute

## Building

In order to build this example, you need a WASI compatible `clang` compiler and an up-to-date
version of [`wasi-libc`], aka the sysroot. Then, you can build the example as follows:

```
clang --sysroot=/path/to/wasi/sysroot/ --target=wasm32-wasi \
        -Iinclude -Llib -lflite_cmu_us_kal -lflite_usenglish \
        -lflite_cmulex -lflite -DDIE_ON_ERROR -DCST_NO_SOCKETS \
        -DWASM32_WASI -nostartfiles -Wl,--no-entry,--export=compute \
        -o compute.wasm compute.c
```

Note that for your convenience I've already precompiled parts of `flite` required by this example
and put them inside `lib/` dir. Also, a keen reader will note that we are actually *not* building
a runnable Wasm module (i.e., containing the `_start` export) but only a module which exports the
compute function with signature:

```c
void compute(in: __wasi_fd_t, out: __wasi_fd_t);
```

[`wasi-libc`]: https://github.com/cranestation/wasi-libc

## Running

In order to invoke the exported function, you'll need a slightly tweaked version of the 
[`wasmtime`] runtime. You can get it by building the `preopen_fd` branch of my fork of
`wasmtime`:

```
git clone https://github.com/kubkon/wasmtime.git
git checkout preopen_fd
```

[`wasmtime`]: https://wasmtime.dev

This branch includes all the necessary tweaks and hacks to be able to invoke the exported
`compute` function while enforcing determinism thanks to WASI's capability-based security
model.

Assuming you've build `wasmtime` OK, you can now invoke `compute` from this example as follows:

```
wasmtime --preopen_read=11:some_input.txt --preopen_write=12:output.wav --invoke=compute compute.wasm -- 11 12
```

So what's happening here? Firstly, the `--preopen_read` and `--preopen_write` arguments are *only*
available in my tweaked version of `wasmtime`. Secondly, the syntax is `__wasi_fd_t:path` where the
first argument points to the preopened WASI file descriptor which will point at the path encoded
as the second argument. Ultimately, the two preopened file descriptors are the arguments that we
pass in to the `compute` function.

## Disclaimer

This code is highly experimental in nature and should not be relied upon in any shape or form.
The example purposely ignores and avoids any error handling so use at your own risk!
