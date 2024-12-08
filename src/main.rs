use std::{fmt, path::PathBuf};

use clap::{command, Parser};

pub mod table;
pub mod table_parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// First table file path
    #[arg(index = 1, help = "Path to the first table file")]
    table1: Option<PathBuf>,

    /// Second table file path
    #[arg(index = 2, help = "Path to the second table file")]
    table2: Option<PathBuf>,

    /// Output file
    #[arg(short, long, help = "Write output to file instead of stdout")]
    output: Option<PathBuf>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arguments:")?;
        writeln!(
            f,
            "{}",
            Self::format_path(&self.table1, "Table 1", "default")
        )?;
        writeln!(
            f,
            "{}",
            Self::format_path(&self.table2, "Table 2", "default")
        )?;
        writeln!(f, "{}", Self::format_path(&self.output, "Output", "stdout"))?;
        Ok(())
    }
}

impl Args {
    fn format_path(path: &Option<PathBuf>, name: &str, default: &str) -> String {
        match path {
            Some(path) => format!("  {}: {}", name, path.display()),
            None => format!("  {}: {}", name, default),
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("Debug {}!", args);
}
