use crate::runner::Runner;
use std::fs::{create_dir_all, write};
use std::{io::Cursor, iter};
use ureq::get;
use url::Url;

mod runner;

static IGNORE_TAGS: &[&str] = &[
    "nav", "script", "style", "header", "footer", "img", "svg", "iframe",
];

fn main() {
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
    println!("HTML Size (bytes): {}", html.len());

    let mut runner = Runner::new(out_dir, html);

    runner.run("readability", |html| {
        let mut html = Cursor::new(html.as_bytes());
        readability::extractor::extract(&mut html, &url)
            .unwrap()
            .text
    });

    #[cfg(feature = "llm_readability")]
    {
        runner.run("llm_readability", |html| {
            let mut html = Cursor::new(html.as_bytes());
            llm_readability::extractor::extract(&mut html, &url)
                .unwrap()
                .text
        });
    }

    #[cfg(feature = "html2text")]
    {
        runner.run("html2text", |html| {
            let mut html = Cursor::new(html.as_bytes());
            html2text::from_read(&mut html, 1000).unwrap_or_default()
        });
    }

    #[cfg(feature = "htmd")]
    {
        runner.run("htmd", |html| {
            htmd::HtmlToMarkdown::builder()
                .skip_tags(IGNORE_TAGS.to_vec())
                .build()
                .convert(html)
                .unwrap_or_default()
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
            .unwrap_or_default()
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

    #[cfg(feature = "august")]
    {
        runner.run("august", |html| august::convert(html, usize::MAX));
    }

    println!("{}", runner.into_table());
    println!("Remember to check the output files to make sure they have parsed the information you expect!");
}
