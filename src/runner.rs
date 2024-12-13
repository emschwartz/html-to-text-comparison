use comfy_table::{CellAlignment, Table};
use std::hint::black_box;
use std::time::{Duration, Instant};
use std::{fs::write, path::PathBuf};

#[derive(Debug, Default)]
pub struct Stats {
    name: &'static str,
    time: Duration,
    output_size: usize,
    peak_memory: u64,
    leaked_memory_single_run: i64,
    leaked_memory_average: i64,
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
        let num_runs: usize = 10;
        let mut stats = (0..num_runs)
            .map(|_| self.calculate_statistics(name, &extractor))
            .fold(Stats::default(), |acc, s| Stats {
                name,
                time: acc.time + s.time,
                output_size: acc.output_size + s.output_size,
                peak_memory: acc.peak_memory.max(s.peak_memory),
                leaked_memory_single_run: s.leaked_memory_single_run,
                leaked_memory_average: acc.leaked_memory_average + s.leaked_memory_single_run,
            });
        stats.time /= num_runs as u32;
        stats.output_size /= num_runs;
        stats.peak_memory /= num_runs as u64;
        stats.leaked_memory_average /= num_runs as i64;
        self.stats.push(stats);
        self.write_to_file(name, extractor);
    }

    pub fn calculate_statistics(
        &mut self,
        name: &'static str,
        extractor: &impl Fn(&str) -> String,
    ) -> Stats {
        let mut time = Duration::ZERO;
        let mut output_size = 0;
        let allocation_info = allocation_counter::measure(|| {
            let start = Instant::now();
            let parsed = black_box(extractor(&self.html));
            output_size = parsed.len();
            drop(parsed);
            time += start.elapsed();
        });

        Stats {
            name,
            time,
            output_size,
            peak_memory: allocation_info.bytes_max,
            leaked_memory_single_run: allocation_info.bytes_current,
            leaked_memory_average: allocation_info.bytes_current,
        }
    }

    pub fn write_to_file(&self, name: &'static str, extractor: impl Fn(&str) -> String) {
        let output_file = self.out_dir.join(format!("{}.txt", name));
        let output = extractor(&self.html);
        write(&output_file, &output).unwrap();
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
            "Leaked Memory Average (bytes)",
            "Leaked Memory Average as % of HTML Size",
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
                &format!("{}", stat.leaked_memory_single_run),
                &format!(
                    "{:.2}%",
                    stat.leaked_memory_single_run as f64 / self.html.len() as f64 * 100.0
                ),
                &format!("{}", stat.leaked_memory_average),
                &format!(
                    "{:.2}%",
                    stat.leaked_memory_average as f64 / self.html.len() as f64 * 100.0
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
