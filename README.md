<!--
MIT License

Copyright (c) 2023 Sophie Katz

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
-->

# Rust/Python Playground

This is me just trying out different things with integrating Rust and Python.

* [Python module written in Rust](./rust-python-module/)
* [Calling Python from within Rust code](./python-from-rust)

See the [PyO3 user guide](https://pyo3.rs/v0.20.0/getting_started) for more information.

## Setup

Install both Rust (at least stable) and Python (at least 3.7). And set this environment variable to make sure that Python will work with shared libraries:

```shell
PYTHON_CONFIGURE_OPTS="--enable-shared"
```

Install Maturin:

```shell
pip3 install maturin
```
