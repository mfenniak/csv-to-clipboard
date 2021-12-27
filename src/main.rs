#![windows_subsystem = "windows"]

use std::{fs::File, time::Instant};
use csv::StringRecord;
use clipboard::{ClipboardProvider, ClipboardContext};
use msgbox::IconType;
use std::env;
use anyhow::{Result, Context};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Required filename argument not provided")]
    MissingFileArgument,
}

fn main() -> Result<()> {
    let start = Instant::now();
    match copy_csv_to_clipboard() {
        Ok(_) => {
            let now = Instant::now();
            let elapsed = now.duration_since(start);
            msgbox::create("Success", &format!("Copied file to clipboard; took {}ms", elapsed.as_millis()), IconType::Info)?;

            Ok(())
        },
        Err(e) => {
            msgbox::create("Error!", &format!("Error occurred in rust-clipboardy.exe: {}", e), IconType::Error)?;
            Ok(()) // technically should be an error, but I don't think it really matters        
        }
    }
}

fn copy_csv_to_clipboard() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let _cmd = args.get(0).expect("arg[0] required");
    let file = args.get(1);

    if file.is_none() {
        return Err(Box::new(ApplicationError::MissingFileArgument));
    }

    let file = file.unwrap();

    let mut rdr = csv::Reader::from_reader(File::open(file).context("Failed to open file requested")?);
    let mut str = String::new();

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the error here.
        let record = result.context("failed to read line in file")?;
        str.push_str(&convert_to_tsv_line(record));
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(str)?;

    Ok(())
}

fn convert_to_tsv_line(rec: StringRecord) -> String {
    let mut str = String::new();

    for cell in &rec {
        str.push_str(
            &cell
            .replace("\\", "\\\\")
            .replace("\t", "\\t")
            .replace("\n", "\\n")
        );
        str.push('\t');
    }
    str.pop(); // ignore Option<T> return; works fine even for empty StringRecords this way
    str.push('\n');

    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_tsv_line_simple() {
        let record = StringRecord::from(vec!["a", "b", "c"]);
        assert_eq!(convert_to_tsv_line(record), String::from("a\tb\tc\n"));
    }

    #[test]
    fn convert_to_tsv_line_empty() {
        let record = StringRecord::from(Vec::<&str>::new());
        assert_eq!(convert_to_tsv_line(record), String::from("\n"));
    }

    #[test]
    fn convert_to_tsv_line_with_special_chars() {
        let record = StringRecord::from(vec!["\t", "\n", "\\"]);
        assert_eq!(convert_to_tsv_line(record), String::from("\\t\t\\n\t\\\\\n"));
    }

    #[test]
    fn convert_to_tsv_line_with_special_chars_repeating() {
        let record = StringRecord::from(vec!["\t\n\\\t\n\\"]);
        assert_eq!(convert_to_tsv_line(record), String::from("\\t\\n\\\\\\t\\n\\\\\n"));
    }
}
