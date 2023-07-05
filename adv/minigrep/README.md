# minigrep - Minimal version of grep

From the rust book: https://doc.rust-lang.org/book/ch12-00-an-io-project.html

### Usage

- Case sensitive search

```bash
cargo run -- [search_string] [file.txt]
```

- Case insensitive search

```bash
IGNORE_CASE=1 cargo run -- [search_string] [file.txt]
```

> search_string : string to search.

> file.txt : file to search in.
