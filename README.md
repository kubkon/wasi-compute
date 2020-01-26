# wasi-compute

`wasi-compute` explores the minimal viable model for fully deterministic
computations/functions in [WASI]. The explored model is a direct result
of discussions found in [WebAssembly/WASI/issues/190] thread.

[WASI]: https://wasi.dev
[WebAssembly/WASI/issues/190]: https://github.com/WebAssembly/WASI/issues/190

## The model

TODO...

## Examples

Fully working examples demonstrating the viability of the proposed approach can
be found in this repo, and are as follows:

* [hello-compute] - demonstrates a minimal "Hello World!"-style function
* [test-compute] - verifies that *only* reading from/writing to a specified, preopened
                   WASI file descriptor are possible, and that syscalls such as
                   `random_get`, etc., are purposely filtered out at the runtime level
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

