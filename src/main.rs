use std::fs::{create_dir_all, write};
use std::{io::Cursor, iter};
use ureq::get;
use url::Url;

static IGNORE_TAGS: &[&str] = &[
    "nav", "script", "style", "header", "footer", "img", "svg", "iframe",
];

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
    let html_bytes = html.as_bytes();
    let html_reader = Cursor::new(html_bytes);

    let current_dir = std::env::current_dir().unwrap();
    let out_dir = current_dir.join("out");
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    create_dir_all(&out_dir).unwrap();

    let html_file = out_dir.join("html.html");
    write(&html_file, &html).unwrap();

    #[cfg(feature = "readability")]
    {
        let mut reader = html_reader.clone();
        let parsed = readability::extractor::extract(&mut reader, &url)
            .unwrap()
            .text;
        let output_file = out_dir.join("readability.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "llm_readability")]
    {
        let mut reader = html_reader.clone();
        let parsed = llm_readability::extractor::extract(&mut reader, &url)
            .unwrap()
            .text;
        let output_file = out_dir.join("llm_readability.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "html2text")]
    {
        let mut reader = html_reader.clone();
        let parsed = html2text::from_read(&mut reader, 150).unwrap();
        let output_file = out_dir.join("html2text.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "htmd")]
    {
        let parser = htmd::HtmlToMarkdown::builder()
            .skip_tags(IGNORE_TAGS.to_vec())
            .build();
        let parsed = parser.convert(&html).unwrap();
        let output_file = out_dir.join("htmd.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "html2md-rs")]
    {
        use html2md_rs::structs::{NodeType, ToMdConfig};
        use html2md_rs::to_md::safe_from_html_to_md_with_config;
        let parsed = safe_from_html_to_md_with_config(
            html.clone(),
            &ToMdConfig {
                ignore_rendering: IGNORE_TAGS
                    .iter()
                    .map(|tag| NodeType::from_tag_str(*tag))
                    .chain(iter::once(NodeType::Comment))
                    .collect(),
            },
        )
        .unwrap();
        let output_file = out_dir.join("html2md-rs.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "nanohtml2text")]
    {
        let parsed = nanohtml2text::html2text(&html);
        let output_file = out_dir.join("nanohtml2text.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "readable-readability")]
    {
        let mut parser = readable_readability::Readability::new();
        parser.base_url(url.clone());
        let (node, _metadata) = parser.parse(&html);
        let parsed = node.text_contents();

        let output_file = out_dir.join("readable-readability.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "mdka")]
    {
        let parsed = mdka::from_html(&html);
        let output_file = out_dir.join("mdka.txt");
        write(&output_file, &parsed).unwrap();
    }

    #[cfg(feature = "boilerpipe")]
    {
        let parsed = boilerpipe::parse_document(&html).content().to_string();
        let output_file = out_dir.join("boilerpipe.txt");
        write(&output_file, &parsed).unwrap();
    }
}
