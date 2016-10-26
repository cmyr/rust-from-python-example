# python wrapper for rust crate with rust-cpython

This is a demonstration of using [rust-cpython](https://github.com/dgrunwald/rust-cpython.git) to build a python-loadable library from rust.

This should build on stable rust out of the box.

## gotchas:

- passing the correct names to `py_module_initializer!` is important.
- use the git repo as the source for rust-cpython; the crate on crates.io is out of date.
- on macOS, rust will build a `.dylib`; you will have to manually rename this to `.so` to work with python.


## sources:

This was hacked together by looking at a bunch of other people's code, and a lot of trial and error. These resources were helpful:

- [http://ehiggs.github.io/2015/07/Python-Modules-In-Rust/](http://ehiggs.github.io/2015/07/Python-Modules-In-Rust/)
- [https://github.com/dgrunwald/rust-cpython/issues/13](https://github.com/dgrunwald/rust-cpython/issues/13)
- [https://github.com/dgrunwald/rust-cpython/blob/master/extensions/hello.rs](https://github.com/dgrunwald/rust-cpython/blob/master/extensions/hello.rs)

