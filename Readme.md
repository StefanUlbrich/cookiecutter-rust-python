# Mixed Rust/Python package

* Requires `cookiecutter>=2.2.0` (Ubuntu package might be outdated)
* Uses a modern tech stack based with a lot of
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
    * [Rusty hooks](https://github.com/swellaby/rusty-hook) (similar to [pre-commit](https://pre-commit.com/))

  * Development environment (recommendations)
    * [Visual Studio Code](https://code.visualstudio.com/) or [VS Codium](https://vscodium.com/)
    * or [Helix](https://helix-editor.com/) or [Lapce](https://lapce.dev/)