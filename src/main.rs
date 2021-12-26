use std::error::Error;
use std::fs::File;
use csv::StringRecord;
use clipboard::{ClipboardProvider, ClipboardContext};

// FIXME: add argument for the file to download
// FIXME: change to win32 API or something so that console doesn't appear when running through double-click... maybe MessageBoxA it ir something instead

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader( File::open("test1.csv")?);
    let mut str = String::new();

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the error here.
        let record = result?;
        str.push_str(&convert_to_tsv_line(record));
    }

    println!("tsv: {}", str);

    let mut ctx: ClipboardContext = ClipboardProvider::new()?; // .expect("initializing ClipboardProvider");
    ctx.set_contents(str)?; // .expect("set_contents on ClipboardProvider");


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

    // fixme: empty case
    // fixme: stuff with \n and \t in it

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
