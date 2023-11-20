# axum-example

Ref: https://www.youtube.com/watch?v=XZtlD_m59sM&list=PLsQuQ3zSmVYGvrY9KNSupS7VnbOItOzT2

### cargo watch

Watch the src folder:

```bash
cargo watch -q -c -w src/ -x run
```

Watch the test folder for local development:

```bash
 cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

- `-q` quiet
- `-c` clear between each recompile
- `-w` watch on this folder(src in this case)
- `-x` execute this command(run in this case)
