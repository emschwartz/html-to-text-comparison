# HTML to Text Comparison

This compares various Rust crates for extracting text from HTML:

- [boilerpipe](https://crates.io/crates/boilerpipe)
- [htmd](https://crates.io/crates/htmd)
- [html2md-rs](https://crates.io/crates/html2md-rs)
- [html2text](https://crates.io/crates/html2text)
- [llm_readability](https://crates.io/crates/llm_readability)
- [mdka](https://crates.io/crates/mdka)
- [nanohtml2text](https://crates.io/crates/nanohtml2text)
- [readability](https://crates.io/crates/readability)
- [readable-readability](https://crates.io/crates/readable-readability)

## Usage

This will download the HTML from the given URL, run the HTML through the extractor from each crate, write the output to a file in `out/`, and then print a table of the results.

### Comparing Speed

```sh
cargo run --release -- <url-to-scrape>
```

### Comparing Memory Usage

We can optionally use [`dhat`](https://crates.io/crates/dhat) to track memory usage. Note that the time will be slower when using this option.

```sh
cargo run --release --features=track-memory -- <url-to-scrape>
```
