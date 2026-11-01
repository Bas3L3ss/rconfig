# rconfig

**A lightweight configuration language and toolkit written in Rust.**

`rconfig` is a configuration language implementation and Rust library designed to make configuration files easy to write, parse, validate, inspect, and format.

Built entirely in Rust, the project provides a complete processing pipeline consisting of a lexer, parser, validator, and formatter. It also exposes a public API for integration into external Rust applications.

The project additionally serves as the core engine behind the [rconfig VS Code Extension](https://github.com/PhamThienHung/rconfig-vscode), bringing language support and developer tooling into the editor.

## Features

- **Lexer / Tokenizer** – Converts source text into a sequence of tokens while preserving source locations.
- **Parser** – Transforms tokens into a structured representation of configuration data.
- **Validator** – Detects invalid configuration structures, duplicate keys, unexpected tokens, and other syntax errors.
- **Formatter** – Produces consistently formatted configuration files.
- **CLI** – Provides commands to validate, inspect, list, and format configuration files.
- **Public Rust API** – Allows external applications and libraries to integrate `rconfig` directly.
- **VS Code Integration** – Powers the [rconfig VS Code Extension](https://github.com/PhamThienHung/rconfig-vscode).

## Configuration Format

Configuration files use the `.rcfg` extension.

The format supports common data types, key-value assignments, collections, comments, and recursively nested structures.

### Basic configuration

```rcfg
# Application configuration

app_name = "rconfig-demo"
debug = true
port = 8080
timeout = 2.5

database {
    host = "localhost"
    port = 5432
    enabled = true
} # Inline comment

features = ["logging", "metrics", "cache"]
```

### Nested structures

Structures can be nested recursively to represent more complex configuration hierarchies.

```rcfg
app_name = "rconfig"

database {
    host = "localhost"

    connection {
        pool_size = 10

        retry {
            max_attempts = 3
        }
    }
}
```

## CLI

The command-line interface provides utilities for working with `.rcfg` files.

### Command structure

```bash
rconfig <SUBCOMMAND> [ARGS...]
```

### Available commands

| Command    | Description                                      |
| ---------- | ------------------------------------------------ |
| `validate` | Validate a configuration file and report errors. |
| `get`      | Retrieve a specific configuration value.         |
| `list`     | List configuration entries.                      |
| `format`   | Format a configuration file.                     |

For detailed command usage, run:

```bash
rconfig --help
```

## Rust Library

`rconfig` is designed to be used as both a standalone CLI application and a reusable Rust library.

Its public API exposes functionality for:

- Parsing and validating configuration files.
- Loading configuration data into memory.
- Retrieving values by key or path.
- Listing configuration entries.
- Formatting configuration source.

This allows other Rust applications to use the configuration engine without depending on the CLI.

## Architecture

The project is organized around four primary components:

| Component         | Responsibility                                               |
| ----------------- | ------------------------------------------------------------ |
| Lexer / Tokenizer | Converts source text into tokens and tracks their locations. |
| Parser            | Builds structured configuration data from tokens.            |
| Validator         | Checks configuration correctness and reports diagnostics.    |
| Formatter         | Produces normalized, consistently formatted source code.     |

These components form the foundation of the language tooling and are also used by the VS Code extension.

## Technology Stack

| Dependency                                      | Purpose                        |
| ----------------------------------------------- | ------------------------------ |
| [thiserror](https://crates.io/crates/thiserror) | Structured error handling.     |
| [clap](https://crates.io/crates/clap)           | Command-line argument parsing. |

## Editor Integration

`rconfig` also provides the underlying functionality for the **rconfig VS Code Extension**.

The extension integrates the configuration engine into Visual Studio Code, bringing language tooling closer to the development workflow.

**Related project:** [rconfig VS Code Extension](https://github.com/PhamThienHung/rconfig-vscode)

## Roadmap

- [ ] Improve parser diagnostics and source-location reporting.
- [ ] Expand formatter capabilities and formatting stability.
- [ ] Add syntax highlighting and richer language support.
- [ ] Support conversion between `.rcfg`, JSON, TOML, and XML.
- [ ] Improve performance through benchmarking and optimization.
- [ ] Expand property-based testing and edge-case coverage.

## License

See [LICENSE](LICENSE) for details.
