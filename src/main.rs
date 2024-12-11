use comfy_table::Table;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{io::Cursor, iter};
use ureq::get;
use url::Url;

static IGNORE_TAGS: &[&str] = &[
    "nav", "script", "style", "header", "footer", "img", "svg", "iframe",
];

struct Stats {
    name: &'static str,
    time: Duration,
    output_size: usize,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = if let Some(url) = args.last() {
        Url::parse(url).expect("Invalid URL")
    } else {
        eprintln!("No URL provided");
        return;
    };

    let response = get(url.as_str()).call();
    let html = match response {
        Ok(res) => res.into_string().unwrap(),
        Err(err) => {
            eprintln!("Failed to fetch URL: {}", err);
            return;
        }
    };

    let current_dir = std::env::current_dir().unwrap();
    let out_dir = current_dir.join("out");
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    create_dir_all(&out_dir).unwrap();

    let html_file = out_dir.join("html.html");
    write(&html_file, &html).unwrap();

    let mut runner = Runner::new(out_dir, html);

    #[cfg(feature = "readability")]
    {
        runner.run_with_reader("readability", |html| {
            readability::extractor::extract(html, &url).unwrap().text
        });
    }

    #[cfg(feature = "llm_readability")]
    {
        runner.run_with_reader("llm_readability", |html| {
            llm_readability::extractor::extract(html, &url)
                .unwrap()
                .text
        });
    }

    #[cfg(feature = "html2text")]
    {
        runner.run_with_reader("html2text", |html| html2text::from_read(html, 150).unwrap());
    }

    #[cfg(feature = "htmd")]
    {
        runner.run("htmd", |html| {
            htmd::HtmlToMarkdown::builder()
                .skip_tags(IGNORE_TAGS.to_vec())
                .build()
                .convert(html)
                .unwrap()
        });
    }

    #[cfg(feature = "html2md-rs")]
    {
        use html2md_rs::structs::{NodeType, ToMdConfig};
        use html2md_rs::to_md::safe_from_html_to_md_with_config;
        runner.run("html2md-rs", |html| {
            safe_from_html_to_md_with_config(
                html.to_string(),
                &ToMdConfig {
                    ignore_rendering: IGNORE_TAGS
                        .iter()
                        .map(|tag| NodeType::from_tag_str(*tag))
                        .chain(iter::once(NodeType::Comment))
                        .collect(),
                },
            )
            .unwrap()
        });
    }

    #[cfg(feature = "nanohtml2text")]
    {
        runner.run("nanohtml2text", |html| nanohtml2text::html2text(html));
    }

    #[cfg(feature = "readable-readability")]
    {
        runner.run("readable-readability", |html| {
            let mut parser = readable_readability::Readability::new();
            parser.base_url(url.clone());
            let (node, _metadata) = parser.parse(&html);
            node.text_contents()
        });
    }

    #[cfg(feature = "mdka")]
    {
        runner.run("mdka", |html| mdka::from_html(html));
    }

    #[cfg(feature = "boilerpipe")]
    {
        runner.run("boilerpipe", |html| {
            boilerpipe::parse_document(&html).content().to_string()
        });
    }

    println!("{}", runner.into_table());
    println!("Remember to check the output files to make sure they have parsed the information you expect!");
}

struct Runner {
    out_dir: PathBuf,
    html: String,
    stats: Vec<Stats>,
}

impl Runner {
    fn new(out_dir: PathBuf, html: String) -> Self {
        Self {
            out_dir,
            html,
            stats: Vec::new(),
        }
    }

    fn run(&mut self, name: &'static str, extractor: impl Fn(&str) -> String) {
        let start = Instant::now();
        let parsed = extractor(&self.html);
        self.stats.push(Stats {
            name,
            time: start.elapsed(),
            output_size: parsed.len(),
        });
        let output_file = self.out_dir.join(format!("{}.txt", name));
        write(&output_file, &parsed).unwrap();
    }

    fn run_with_reader(
        &mut self,
        name: &'static str,
        extractor: impl Fn(&mut Cursor<&[u8]>) -> String,
    ) {
        let mut reader = Cursor::new(self.html.as_bytes());
        let start = Instant::now();
        let parsed = extractor(&mut reader);
        self.stats.push(Stats {
            name,
            time: start.elapsed(),
            output_size: parsed.len(),
        });
        let output_file = self.out_dir.join(format!("{}.txt", name));
        write(&output_file, &parsed).unwrap();
    }

    fn into_table(mut self) -> Table {
        self.stats.sort_by_key(|s| s.time);
        let mut table = Table::new();
        table.set_header(vec![
            "Name",
            "Time (ms)",
            "Output Size (bytes)",
            "% Reduction",
            "Output File",
        ]);
        for stat in &self.stats {
            table.add_row(vec![
                stat.name,
                &format!("{}", stat.time.as_millis()),
                &format!("{}", stat.output_size),
                &format!(
                    "{:.2}%",
                    100.0 - (stat.output_size as f64 / self.html.len() as f64) * 100.0
                ),
                &format!(
                    "{}",
                    self.out_dir.join(format!("{}.txt", stat.name)).display()
                ),
            ]);
        }
        table
    }
}
