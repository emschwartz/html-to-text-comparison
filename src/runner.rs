use comfy_table::{CellAlignment, Table};
use std::time::{Duration, Instant};
use std::{fs::write, path::PathBuf};

struct Stats {
    name: &'static str,
    time: Duration,
    output_size: usize,
    peak_memory: u64,
    leaked_memory: i64,
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

        let mut time = Duration::ZERO;
        let mut output_size = 0;

        let stats = allocation_counter::measure(|| {
            let start = Instant::now();

            let parsed = extractor(&self.html);

            time = start.elapsed();
            output_size = parsed.len();
            write(&output_file, &parsed).unwrap();
        });

        self.stats.push(Stats {
            name,
            time,
            output_size,
            peak_memory: stats.bytes_max,
            leaked_memory: stats.bytes_current,
        });
    }

    pub fn into_table(mut self) -> Table {
        self.stats.sort_by_key(|s| s.name);

        let mut table = Table::new();
        table.set_header(vec![
            "Name",
            "Time (microseconds)",
            "Peak Memory (bytes)",
            "Peak Memory as % of HTML Size",
            "Leaked Memory (bytes)",
            "Leaked Memory as % of HTML Size",
            "Output Size (bytes)",
            "% Reduction",
            "Output File",
        ]);
        let numeric_columns = 1..=6;
        for column in numeric_columns {
            table
                .column_mut(column)
                .unwrap()
                .set_cell_alignment(CellAlignment::Right);
        }

        for stat in &self.stats {
            table.add_row(vec![
                stat.name,
                &format!("{}", stat.time.as_micros()),
                &format!("{}", stat.peak_memory),
                &format!(
                    "{:.2}%",
                    stat.peak_memory as f64 / self.html.len() as f64 * 100.0
                ),
                &format!("{}", stat.leaked_memory),
                &format!(
                    "{:.2}%",
                    stat.leaked_memory as f64 / self.html.len() as f64 * 100.0
                ),
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
