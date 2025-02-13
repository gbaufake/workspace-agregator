use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};
use terminal_size::{terminal_size, Width};

pub struct ChartGenerator {
    max_width: usize,
}

impl Default for ChartGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartGenerator {
    pub fn new() -> Self {
        let width = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(80);

        Self {
            max_width: width.saturating_sub(40), // Leave space for labels
        }
    }

    pub fn generate_bar_chart(
        &self,
        writer: &mut impl Write,
        data: &[(String, f64)],
        title: &str,
    ) -> io::Result<()> {
        writeln!(writer, "\n{}", title)?;
        writeln!(writer, "{}", "=".repeat(title.len()))?;

        let max_value = data.iter().map(|(_, value)| *value).fold(0.0, f64::max);

        let max_label_len = data.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

        for (label, value) in data {
            let bar_width = ((value / max_value) * self.max_width as f64) as usize;
            let bar = "█".repeat(bar_width);
            let percentage = (value / max_value) * 100.0;

            writeln!(
                writer,
                "{:width$} │ {:>6.1}% {}",
                label,
                percentage,
                self.colorize_bar(&bar, percentage),
                width = max_label_len
            )?;
        }

        writeln!(writer)
    }

    pub fn generate_histogram(
        &self,
        writer: &mut impl Write,
        data: &HashMap<String, usize>,
        title: &str,
    ) -> io::Result<()> {
        writeln!(writer, "\n{}", title)?;
        writeln!(writer, "{}", "=".repeat(title.len()))?;

        let total: usize = data.values().sum();
        let max_label_len = data.keys().map(|k| k.len()).max().unwrap_or(0);

        let mut items: Vec<_> = data.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1));

        for (label, &count) in items {
            let percentage = (count as f64 / total as f64) * 100.0;
            let bar_width = ((percentage / 100.0) * self.max_width as f64) as usize;
            let bar = "█".repeat(bar_width);

            writeln!(
                writer,
                "{:width$} │ {:>5} {:>6.1}% {}",
                label,
                count,
                percentage,
                self.colorize_bar(&bar, percentage),
                width = max_label_len
            )?;
        }

        writeln!(writer)
    }

    fn colorize_bar(&self, bar: &str, percentage: f64) -> colored::ColoredString {
        match percentage {
            p if p >= 75.0 => bar.red(),
            p if p >= 50.0 => bar.yellow(),
            p if p >= 25.0 => bar.green(),
            _ => bar.blue(),
        }
    }
}
