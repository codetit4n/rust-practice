# Single-threaded Web Server

From the rust book: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

### Usage

1. Run the web server

```bash
cargo run
```

2. Open [`http://127.0.0.1:7878`](http://127.0.0.1:7878/) in a web browser
   - Route: `/` - Will render [hello.html](./hello.html)
   - Any other route - Will render [404.html](./404.html)
