# setrcomp

A fast CLI tool written in Rust for quickly bootstrapping project files from templates. Select a language, pick a template and name your directory — all from an interactive terminal UI.

---

## Features

- **Interactive TUI** — navigate extensions and templates with `j`/`k` (vim-style)
- **Template preview** — press `s` on any template to preview it with [`bat`](https://github.com/sharkdp/bat)
- **Non-interactive mode** — pass all arguments directly from the command line
- **Configurable template directory** — use the `SETRCOMP_TOOLS` env var to point to your own templates
- **Auto-copies** the template into a new directory as `<dirname>/main.<ext>`

---

## Installation

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024+ 1.85.0 >=)
- [`bat`](https://github.com/sharkdp/bat) *(optional, for template previews)*

### Build from source

```bash
git clone https://github.com/AGONIXX15/setrcomp
cd setrcomp
cargo build --release
```

The binary will be at `target/release/setrcomp`. You can move it to a directory in your `$PATH`:
(the script in script/install.sh will do all of this)
```bash
cp target/release/setrcomp ~/.local/bin/
```

---

## Template directory setup

By default, setrcomp looks for templates in:

| Priority | Path |
|---|---|
| 1 | `$SETRCOMP_TOOLS/templates` |
| 2 | `~/.local/share/setrcomp/templates` (Linux) |

Templates are organized by extension:

```
templates/
└── cpp/
    ├── basic.cpp
    ├── segment_tree.cpp
    └── ...
```

You can set a custom path by exporting the env var:

```bash
export SETRCOMP_TOOLS=/path/to/your/tools
```

---

## Usage

```
setrcomp [extension] [template] [filename]
```

All arguments are optional. If any are omitted, an interactive prompt will appear.

### Interactive mode

```bash
setrcomp
```

Use `j`/`k` to move through the list and `Enter` to select. When browsing templates, press `s` to preview the selected template with `bat`.

### Non-interactive mode

```bash
setrcomp cpp basic my_solution
```

This will create a `my_solution/` directory with `main.cpp` copied from the `basic` template.

---

## Key bindings

| Key | Action |
|---|---|
| `j` | Move selection down |
| `k` | Move selection up |
| `Enter` | Confirm selection |
| `s` | Preview selected template (template list only) |
| `Ctrl+C` | Exit |

---

## Project structure

```
setrcomp/
├── src/
│   ├── main.rs       # CLI entry point and core logic
│   └── terminal.rs   # Interactive TUI (crossterm)
├── templates/
│   └── cpp/          # C++ template files
├── scripts/          # Helper scripts
├── Cargo.toml
└── Cargo.lock
```

---

## Dependencies

| Crate | Purpose |
|---|---|
| [`clap`](https://crates.io/crates/clap) | Argument parsing |
| [`crossterm`](https://crates.io/crates/crossterm) | Cross-platform terminal control |
| [`dirs`](https://crates.io/crates/dirs) | Platform-specific data directories |

---

## License

This project is personal and open source. Feel free to fork it and adapt it to your own workflow.
