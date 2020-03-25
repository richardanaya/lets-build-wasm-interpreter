# Let's build a wasm interpreter

This is a lesson plan to help teach web assembly by building a fully complaint wasm interpeter. Ultimately we'll be building up to an interpreter that:

* uses only `#[no_std]` and `alloc`
* works on nightly for `#[no_std]` error handling
* no complicated data structures
* runs `async`
