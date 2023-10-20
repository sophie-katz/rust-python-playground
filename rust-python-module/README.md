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

# Python module written in Rust

I followed [these instructions](https://github.com/PyO3/maturin) to create this sample project.

## Developing

Follow these instructions to set up a Python venv with the Rust module installed.

```shell
# Run this from the repository root

# Create new venv
python3 -m venv .venv

# Activate venv
source .venv/bin/activate

# Build module into venv
maturin develop -m rust-python-module/Cargo.toml
```

## Deploying

To create a wheel, run:

```shell
# Run this from the repository root

maturin build -m rust-python-module/Cargo.toml
```

The wheel will be stored under [`target/wheels/`](./target/wheels/).
