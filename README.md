1. Project overview

`rconfig` is a lightweight configuration format and parser written entirely in Rust. It lets users define application settings in a human-readable file, load that file into memory, query values, validate configuration, and inspect errors when something goes wrong.

Goal:
Your goal is not simply to make a parser that works. Your goal is to design a small, idiomatic Rust library that demonstrates why Rust's ownership model, lifetimes, enums, traits, generics, and error handling are useful in a real system.

2. technical

extension: .rcfg
components: lexer/tokenizer, parser

tools:

- CLI: validate config, get value (inspect), list

- Usage:
  **Command-Line Grammar:**

  ```
  rconfig <SUBCOMMAND> [ARGS...]
  ```

  - `rconfig`: The CLI tool entry point
  - `<SUBCOMMAND>`: Action to perform, such as `validate`, `get`, `list`
  - `[ARGS...]`: Arguments required by the subcommand

- APIs: expose public apis for external rust projects to make use of. Validate config, inspect, list

3. .rcfg design

```
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

Allow recursive model.

```
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

4. Cargo

mods: thiserror (error handling), clap (useful cli parsing),
dev: proptest(randomized test), criterion(performance tracking), tracing (observation)

5. Future plan:

- Add support for conversion to JSON, Toml, and XML
