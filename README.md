# HTML to Text Comparison

This compares various Rust crates for extracting text from HTML:

- [boilerpipe](https://crates.io/crates/boilerpipe) ![boilerpipe](https://img.shields.io/crates/d/boilerpipe)
- [htmd](https://crates.io/crates/htmd) ![htmd](https://img.shields.io/crates/d/htmd)
- [html2md-rs](https://crates.io/crates/html2md-rs) ![html2md-rs](https://img.shields.io/crates/d/html2md-rs)
- [html2text](https://crates.io/crates/html2text) ![html2text](https://img.shields.io/crates/d/html2text)
- [llm_readability](https://crates.io/crates/llm_readability) ![llm_readability](https://img.shields.io/crates/d/llm_readability)
- [mdka](https://crates.io/crates/mdka) ![mdka](https://img.shields.io/crates/d/mdka)
- [nanohtml2text](https://crates.io/crates/nanohtml2text) ![nanohtml2text](https://img.shields.io/crates/d/nanohtml2text)
- [readability](https://crates.io/crates/readability) ![readability](https://img.shields.io/crates/d/readability)
- [readable-readability](https://crates.io/crates/readable-readability) ![readable-readability](https://img.shields.io/crates/d/readable-readability)

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
❯ cargo run --release -- https://example.com
    Finished `release` profile [optimized] target(s) in 0.10s
     Running `target/release/html-to-text-comparison 'https://example.com'`
HTML Size (bytes): 1256
+----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------+
| Name                 | Time (microseconds) | Peak Memory (bytes) | Peak Memory as % of HTML Size | Leaked Memory (bytes) | Leaked Memory as % of HTML Size | Output Size (bytes) | % Reduction | Output File                  |
+===============================================================================================================================================================================================================================+
| boilerpipe           |                  97 |              657025 |                     52310.91% |                331376 |                       26383.44% |                 171 | 86.39%      | out/boilerpipe.txt           |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| htmd                 |                 142 |               19481 |                      1551.04% |                    64 |                           5.10% |                 247 | 80.33%      | out/htmd.txt                 |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| html2md-rs           |                   5 |                2755 |                       219.35% |                     0 |                           0.00% |                   0 | 100.00%     | out/html2md-rs.txt           |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| html2text            |                 156 |               17671 |                      1406.93% |                     0 |                           0.00% |                 240 | 80.89%      | out/html2text.txt            |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| llm_readability      |                 233 |              350491 |                     27905.33% |                 41746 |                        3323.73% |                 189 | 84.95%      | out/llm_readability.txt      |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| mdka                 |                  83 |               15856 |                      1262.42% |                     0 |                           0.00% |                 241 | 80.81%      | out/mdka.txt                 |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| nanohtml2text        |                  27 |                3080 |                       245.22% |                     0 |                           0.00% |                 250 | 80.10%      | out/nanohtml2text.txt        |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| readability          |                 170 |              350341 |                     27893.39% |                 41688 |                        3319.11% |                 175 | 86.07%      | out/readability.txt          |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| readable-readability |                  99 |              433695 |                     34529.86% |                163578 |                       13023.73% |                 175 | 86.07%      | out/readable-readability.txt |
+----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------+
Remember to check the output files to make sure they have parsed the information you expect!
```

### `https://github.com/mozilla/readability`

```sh
cargo run --release -- https://github.com/mozilla/readability
    Finished `release` profile [optimized] target(s) in 0.11s
     Running `target/release/html-to-text-comparison 'https://github.com/mozilla/readability'`
HTML Size (bytes): 346565
+----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------+
| Name                 | Time (microseconds) | Peak Memory (bytes) | Peak Memory as % of HTML Size | Leaked Memory (bytes) | Leaked Memory as % of HTML Size | Output Size (bytes) | % Reduction | Output File                  |
+===============================================================================================================================================================================================================================+
| boilerpipe           |                6795 |             3413897 |                       985.07% |                362958 |                         104.73% |                 266 | 99.92%      | out/boilerpipe.txt           |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| htmd                 |                7319 |             1643114 |                       474.11% |                    64 |                           0.02% |               14071 | 95.94%      | out/htmd.txt                 |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| html2md-rs           |                4793 |             2437013 |                       703.19% |                     0 |                           0.00% |               17687 | 94.90%      | out/html2md-rs.txt           |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| html2text            |                9665 |             2479874 |                       715.56% |                     0 |                           0.00% |               28646 | 91.73%      | out/html2text.txt            |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| llm_readability      |                6675 |             1482504 |                       427.77% |                166599 |                          48.07% |                  19 | 99.99%      | out/llm_readability.txt      |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| mdka                 |                6125 |             2079408 |                       600.01% |                     0 |                           0.00% |                6895 | 98.01%      | out/mdka.txt                 |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| nanohtml2text        |                2987 |              859805 |                       248.09% |                     0 |                           0.00% |               18741 | 94.59%      | out/nanohtml2text.txt        |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| readability          |                9989 |             1548184 |                       446.72% |                232135 |                          66.98% |                  53 | 99.98%      | out/readability.txt          |
|----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------|
| readable-readability |                6335 |             2149347 |                       620.19% |                250318 |                          72.23% |                  53 | 99.98%      | out/readable-readability.txt |
+----------------------+---------------------+---------------------+-------------------------------+-----------------------+---------------------------------+---------------------+-------------+------------------------------+
Remember to check the output files to make sure they have parsed the information you expect!
```
