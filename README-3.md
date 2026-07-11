# Vacuum Cleaner 🧹

A simple command-line tool that deletes a file/directory you specify, or
every file and directory matching a glob pattern — always with a
confirmation step first.

> ⚠️ **Warning:** This tool deletes files **permanently** (it does not
> move them to the trash/recycle bin). A confirmation prompt is always
> shown before deleting, but use it carefully and keep backups of
> anything important.

## Features

- **Delete a specific file/directory** — enter the full path, confirm,
  and it's deleted.
- **Bulk delete by pattern** — enter a [glob](https://docs.rs/glob)
  pattern such as `C:\**\*.tmp` or `/home/user/**/*.log`; every
  matching file/directory is listed, and a single confirmation deletes
  them all.
- Always asks for confirmation before deleting anything.
- Clear error messages for missing paths and invalid patterns.

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (tested
  with the Rust 2021 edition)

## Installation

```bash
git clone https://github.com/<your-username>/vacuum-cleaner.git
cd vacuum-cleaner
cargo build --release
```

The compiled binary will be at `target/release/vacuum_cleaner` (or
`vacuum_cleaner.exe` on Windows).

## Usage

```bash
cargo run
# or, using the compiled binary:
./target/release/vacuum_cleaner
```

On startup, the program offers two options:

```
Welcome to the vacuum cleaner application
Please choose an option:
1. Clean a specific file or directory
2. Clean files or directories matching a pattern
```

### 1) Clean a specific file or directory

Enter the full path to a file or directory; the program asks for
confirmation and deletes it if you agree.

```
Enter the full path of the file or directory to delete:
/tmp/old_file.txt
Delete '/tmp/old_file.txt' permanently? [y/N]: y
Deleted: /tmp/old_file.txt
```

### 2) Clean files or directories matching a pattern

Enter a glob pattern (e.g. to match all `.tmp` files). The program
first lists every matching path, then deletes them all after a single
confirmation.

```
Enter a glob pattern to match (e.g. C:\**\*.tmp):
/tmp/test_area/*.tmp
Found 2 match(es):
  /tmp/test_area/a.tmp
  /tmp/test_area/b.tmp
Delete ALL of the above? [y/N]: y
Deleted: /tmp/test_area/a.tmp
Deleted: /tmp/test_area/b.tmp
```

## Project structure

```
vacuum-cleaner/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

## Dependencies

- [`glob`](https://crates.io/crates/glob) — file searching using glob
  patterns

## Contributing

Pull requests and issues are welcome. Before submitting changes,
please make sure `cargo build` (and `cargo test`, if tests exist)
runs cleanly.

## License

This project is licensed under the [MIT License](LICENSE).
