# HTML to Text Comparison

This compares various Rust crates for extracting text from HTML.

## Usage

### Comparing Speed

```sh
cargo run --release -- <url-to-scrape>
```

### Comparing Memory Usage

We can optionally use [`dhat`](https://crates.io/crates/dhat) to track memory usage. Note that the time will be slower when using this option.

```sh
cargo run --release --features=track-memory -- <url-to-scrape>
```
