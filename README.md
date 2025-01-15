# Rust HTML to Text Comparison

This compares various Rust crates for extracting text from HTML:

- [august](https://crates.io/crates/august) ![august](https://img.shields.io/crates/d/august)
- [boilerpipe](https://crates.io/crates/boilerpipe) ![boilerpipe](https://img.shields.io/crates/d/boilerpipe)
- [dom_smoothie](https://crates.io/crates/dom_smoothie) ![dom_smoothie](https://img.shields.io/crates/d/dom_smoothie)
- [fast_html2md](https://crates.io/crates/fast_html2md) ![fast_html2md](https://img.shields.io/crates/d/fast_html2md)
- [htmd](https://crates.io/crates/htmd) ![htmd](https://img.shields.io/crates/d/htmd)
- [html2md-rs](https://crates.io/crates/html2md-rs) ![html2md-rs](https://img.shields.io/crates/d/html2md-rs)
- [html2text](https://crates.io/crates/html2text) ![html2text](https://img.shields.io/crates/d/html2text)
- [llm_readability](https://crates.io/crates/llm_readability) ![llm_readability](https://img.shields.io/crates/d/llm_readability)
- [mdka](https://crates.io/crates/mdka) ![mdka](https://img.shields.io/crates/d/mdka)
- [nanohtml2text](https://crates.io/crates/nanohtml2text) ![nanohtml2text](https://img.shields.io/crates/d/nanohtml2text)
- [readability](https://crates.io/crates/readability) ![readability](https://img.shields.io/crates/d/readability)
- [readable-readability](https://crates.io/crates/readable-readability) ![readable-readability](https://img.shields.io/crates/d/readable-readability)

These crates are used for a variety of different purposes, ranging from displaying websites in a terminal to extracting text for use in LLMs.

I am particularly interested in picking one to use for [Scour](https://scour.ing), where I need to run websites through an LLM embedding model for semantic search.

## Usage

```sh
cargo run --release -- <url-to-scrape>
```

This will:

1. Download the page from the given URL
2. Run the HTML through the extractor from each crate once to measure the memory usage
3. Run the HTML through the extractor from each crate once more to measure the time
4. Write the output to a file in `out/`
5. Print a table of the results

### Warning: check the output!

It is important to check the output files to ensure they contain the information you expect!

For example, the various readability crates seem to miss most of the content from Github repositories.
In the [mozilla/readability example](#https://github.com/mozilla/readability), the output is just the string `"mozilla/readability"` for the `llm_readability` crate and `" You can’t perform that action at this time."` for `readability` and `readable-readability`.

## Examples

### `https://example.com`

```sh
cargo run --release -- https://example.com
    Finished `release` profile [optimized] target(s) in 0.12s
     Running `target/release/html-to-text-comparison 'https://example.com'`
HTML Size (bytes): 1256
+----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------+
| Name                 | Time (microseconds) | Peak Memory (bytes) | Peak Memory as % of HTML Size | Output Size (bytes) | % Reduction | Output File                  |
+=========================================================================================================================================+
| august               |                  58 |               33239 |                      2646.42% |                 228 | 81.85%      | out/august.txt               |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| boilerpipe           |                  52 |               65702 |                      5231.05% |                 171 | 86.39%      | out/boilerpipe.txt           |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| dom_smoothie         |                 478 |               61156 |                      4869.11% |                 256 | 79.62%      | out/dom_smoothie.txt         |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| fast_html2md         |                  61 |                3260 |                       259.55% |                 229 | 81.77%      | out/fast_html2md.txt         |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| htmd                 |                  92 |                1948 |                       155.10% |                 247 | 80.33%      | out/htmd.txt                 |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| html2md-rs           |                   3 |                 275 |                        21.89% |                   0 | 100.00%     | out/html2md-rs.txt           |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| html2text            |                 101 |                1767 |                       140.68% |                 240 | 80.89%      | out/html2text.txt            |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| llm_readability      |                 156 |               35049 |                      2790.53% |                 189 | 84.95%      | out/llm_readability.txt      |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| mdka                 |                  57 |                1585 |                       126.19% |                 241 | 80.81%      | out/mdka.txt                 |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| nanohtml2text        |                  14 |                 308 |                        24.52% |                 250 | 80.10%      | out/nanohtml2text.txt        |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| readability          |                  91 |               35034 |                      2789.33% |                 175 | 86.07%      | out/readability.txt          |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------|
| readable-readability |                  73 |               43369 |                      3452.95% |                 175 | 86.07%      | out/readable-readability.txt |
+----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+------------------------------+
Remember to check the output files to make sure they have parsed the information you expect!
```

### `https://github.com/mozilla/readability`

```sh
cargo run --release -- https://github.com/mozilla/readability
    Finished `release` profile [optimized] target(s) in 0.40s
     Running `target/release/html-to-text-comparison 'https://github.com/mozilla/readability'`
HTML Size (bytes): 351119
+----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------+
| Name                 | Time (microseconds) | Peak Memory (bytes) | Peak Memory as % of HTML Size | Output Size (bytes) | % Reduction | Output File                   |
+======================================================================================================================================================================+
| august               |                6664 |              218347 |                        62.19% |               13498 | 96.16%      | out/august.txt                |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| boilerpipe           |                6773 |              342658 |                        97.59% |                 266 | 99.92%      | out/boilerpipe.txt            |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| dom_smoothie         |               12795 |              504790 |                       143.77% |               20976 | 94.03%      | out/dom_smoothie.txt          |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| fast_html2md         |                3787 |                8707 |                         2.48% |               16082 | 95.42%      | out/fast_html2md.txt          |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| htmd                 |                6442 |              165595 |                        47.16% |               14513 | 95.87%      | out/htmd.txt                  |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| html2md-rs           |                4406 |              246373 |                        70.17% |               18209 | 94.81%      | out/html2md-rs.txt            |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| html2text            |                7752 |              250147 |                        71.24% |               29458 | 91.61%      | out/html2text.txt             |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| llm_readability      |                5013 |              149639 |                        42.62% |                  19 | 99.99%      | out/llm_readability.txt       |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| mdka                 |                5985 |              210674 |                        60.00% |                6896 | 98.04%      | out/mdka.txt                  |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| nanohtml2text        |                2775 |               86376 |                        24.60% |               19126 | 94.55%      | out/nanohtml2text.txt         |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| readability          |                6062 |              156207 |                        44.49% |                  53 | 99.98%      | out/readability.txt           |
|----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------|
| readable-readability |                5983 |              216618 |                        61.69% |                  53 | 99.98%      | out/readable-readability.txt  |
+----------------------+---------------------+---------------------+-------------------------------+---------------------+-------------+-------------------------------+
```

## Optional: Jina `reader-lm`

You can also compare the results using the [jinaai/reader-lm](https://huggingface.co/jinaai/reader-lm-1.5b) model via their hosted API.

This isn't included by default because it isn't a totally fair comparison.
It's not running locally and requires an API key. The model has open weights so you could run it locally, but it is licensed under a non-commercial license.

To enable the comparison with the Jina Reader:

1. Get a free API key from https://jina.ai/reader
2. Build the crate with the `reader-lm-api` feature enabled
3. Set the `JINA_API_KEY` environment variable

```sh
JINA_API_KEY=... cargo run --release --features=reader-lm-api -- <url-to-scrape>
```
