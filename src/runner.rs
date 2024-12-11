use comfy_table::{CellAlignment, Table};
use std::time::{Duration, Instant};
use std::{fs::write, path::PathBuf};

struct Stats {
    name: &'static str,
    time: Duration,
    output_size: usize,
    #[cfg(feature = "track-memory")]
    peak_memory: usize,
    #[cfg(feature = "track-memory")]
    leaked_memory: usize,
}

pub struct Runner {
    out_dir: PathBuf,
    html: String,
    stats: Vec<Stats>,
}

impl Runner {
    pub fn new(out_dir: PathBuf, html: String) -> Self {
        Self {
            out_dir,
            html,
            stats: Vec::new(),
        }
    }

    pub fn run(&mut self, name: &'static str, extractor: impl Fn(&str) -> String) {
        let output_file = self.out_dir.join(format!("{}.txt", name));

        #[cfg(feature = "track-memory")]
        let _profiler = dhat::Profiler::builder().testing().build();
        let start = Instant::now();

        let parsed = extractor(&self.html);

        let time = start.elapsed();
        let output_size = parsed.len();
        write(&output_file, &parsed).unwrap();

        #[cfg(feature = "track-memory")]
        let stats = {
            drop(parsed);
            drop(extractor);
            std::thread::sleep(Duration::from_millis(10));
            dhat::HeapStats::get()
        };

        self.stats.push(Stats {
            name,
            time,
            output_size,
            #[cfg(feature = "track-memory")]
            peak_memory: stats.max_bytes,
            #[cfg(feature = "track-memory")]
            leaked_memory: stats.curr_bytes,
        });
    }

    pub fn into_table(mut self) -> Table {
        self.stats.sort_by_key(|s| s.name);

        let mut table = Table::new();
        table.set_header(vec![
            "Name",
            #[cfg(feature = "track-memory")]
            "Peak Memory (bytes)",
            #[cfg(feature = "track-memory")]
            "Peak Memory as % of HTML Size",
            #[cfg(feature = "track-memory")]
            "Leaked Memory (bytes)",
            #[cfg(feature = "track-memory")]
            "Leaked Memory as % of HTML Size",
            "Time (microseconds)",
            "Output Size (bytes)",
            "% Reduction",
            "Output File",
        ]);
        #[cfg(feature = "track-memory")]
        let numeric_columns = 1..=6;
        #[cfg(not(feature = "track-memory"))]
        let numeric_columns = 1..=3;
        for column in numeric_columns {
            table
                .column_mut(column)
                .unwrap()
                .set_cell_alignment(CellAlignment::Right);
        }

        for stat in &self.stats {
            table.add_row(vec![
                stat.name,
                #[cfg(feature = "track-memory")]
                &format!("{}", stat.peak_memory),
                #[cfg(feature = "track-memory")]
                &format!(
                    "{:.2}%",
                    stat.peak_memory as f64 / self.html.len() as f64 * 100.0
                ),
                #[cfg(feature = "track-memory")]
                &format!("{}", stat.leaked_memory),
                #[cfg(feature = "track-memory")]
                &format!(
                    "{:.2}%",
                    stat.leaked_memory as f64 / self.html.len() as f64 * 100.0
                ),
                &format!("{}", stat.time.as_micros()),
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
