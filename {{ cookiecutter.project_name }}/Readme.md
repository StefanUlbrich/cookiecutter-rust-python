# `python-rust-testbed`

A Testbed for Python and Rust code. Uses a modern tech stack based with a lot of
opinionated choice:

* Python (`^3.12`)
  * [Pyenv](https://github.com/pyenv/pyenv)
  * [Poetry](https://python-poetry.org/) (packaging and environment management).
  * [PoeThePoet](https://poethepoet.natn.io/index.html) (task runner)
  * [Ruff](https://github.com/astral-sh/ruff) (extremely fast linter)
  * [Mypy](https://mypy-lang.org/) (type checker)
  * Jupyterlab, matplotlib (`--with jupyter`)
  * [Sphinx](https://www.sphinx-doc.org/en/master/),
    [pydata theme](https://github.com/pydata/pydata-sphinx-theme),
    and markdown via [myst-parser](https://myst-parser.readthedocs.io/en/latest/))
    (`--with docs`)
  * [numpy](https://numpy.org/), [scipy](https://scipy.org/), [sklearn](https://scikit-learn.org/stable/)
  * Cuda: [cupy](https://cupy.dev/) (`-e cuda12x`, `-e cuda12x` (only cupy))
  * ~~[pytorch](https://pytorch.org/)~~ (while Python 3.12 is not [supported](https://github.com/openai/triton/issues/2707))

* Rust
  * [ndarray]() (equivalent of `numpy`)
  * [PyO3]() (Python bindings)
  * [cargo-show-asm](https://github.com/pacak/cargo-show-asm)

* Development environment (recommendations)
  * [Visual Studio Code](https://code.visualstudio.com/) or [VS Codium](https://vscodium.com/)
  * or [Helix](https://helix-editor.com/) or [Lapce](https://lapce.dev/)
  * [Fira Code Font](https://github.com/tonsky/FiraCode) (`sudo apt install fonts-firacode`)
  * [WezTerm](https://wezfurlong.org/wezterm/install/linux.html?h=ubuntu#installing-on-ubuntu-and-debian-based-systems) (Terminal with support of font ligatures)

The repository is structured as follows:

* `python` sources for the pure python project
* `rust` sources for the optimized code in rust
* `bindings` sources for wrapping the rust code for python.

  This is separated from the `rust` crate as this is required for inline testing
* `pyproject.toml`, `poetry.lock` The definition of the python project and the locked virtual environment
* `Cargo.toml`, `Cargo.lock`: The definition of the workspace and locked dependencies

## Installation

### Install cuda (on Ubuntu)

```sh
sudo apt install nvidia-cuda-toolkit # 7 GiB!
# verify (after everything else is installed)
poetry install -E cuda12x
python -c "import cupy as cp; print(cp.cuda.get_cuda_path()); x_gpu = cp.array([1, 2, 3])"
```

### Python

I recommend installing with [PyEnv](https://github.com/pyenv/pyenv).

```sh
curl https://pyenv.run | bash
pyenv install -l # choose a recent version
pyenv install 3.12.0
```

Alternatively, you can use the system's version

```sh
sudo apt install python3.12
```

#### Poetry and PoeThePoet

For Python3.12, you will need Poetry `^1.7`.

```sh
curl -sSL https://install.python-poetry.org | python3 -
poetry self add 'poethepoet[poetry_plugin]'
```

Available tasks:

```sh
poetry # will list all commands
poetry lint # runs ruff to fix all auto-fixable issues
poetry mypy # runs the mypy type checker
poetry fmt # runs ruff to format the source files
poetry test # runs test and coverage
poetry all # runs all of the above
poetry install-kernel # makes the environment available in jupyter
poetry uninstall-kernel # removes the associated jupyter kernel
poetry verify # Checks whether cuda works as expected
poetry docs # Builds the docs
poetry lab # starts a jupyter service
poetry nvidia # starts the nvidia  monitor
```

#### Install the package

```sh
# optional. when using pyenv, select your version
pyenv shell 3.12 && poetry env use python
poetry install --no-root -E cuda12x # with cuda installed
poetry install --no-root -E cuda12x --with jupyter # when you want jupyter
poetry install --no-root -E cuda12x --with jupyter,doc # when you want to build the documentation
```

or add it to another project as a dependency (assuming you have your github configured
adequately). E.g.:

```sh
poetry add git++ssh://github.com:StefanUlbrich/cupy_som-oss.git
```

#### Build documentation

```sh
poetry install --with doc # and other switches
poetry docs
firefox docs/build/html/index.html # or similar
```

### Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

```sh
cargo install bottom --locked
cargo install --locked zellij
cargo install --locked --path helix-term
```

```sh
cargo install cargo-criterion
cargo install cargo-show-asm
cargo asm -v --simplify --release --lib -p PROJECT LIBRARY::MODULE::FUNCTION 0 --rust
```
