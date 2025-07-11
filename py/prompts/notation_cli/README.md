# Notation CLI

A command-line tool for parsing and viewing music notation files.

## Installation

This project uses [Poetry](https://python-poetry.org/) for dependency management.

1.  **Install Poetry:**
    ```bash
    curl -sSL https://install.python-poetry.org | python3 -
    ```

2.  **Install Dependencies:**
    Navigate to the `notation_cli` directory and run:
    ```bash
    poetry install
    ```

This will create a virtual environment and install all necessary packages, including the CLI itself.

## Usage

Once installed, you can use the `notation` command from within the project's virtual environment.

### `view`

To view a labeled document:

```bash
poetry run notation view /path/to/your/labeled_document.yaml
```

This will display a side-by-side comparison of the original source text and the color-coded, labeled version.
