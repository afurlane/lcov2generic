use std::fs::File;
use std::io::{BufRead, BufReader};
use xmlwriter::XmlWriter;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: lcov2generic <input.lcov> <output.xml>");
        std::process::exit(1);
    }

    let input = File::open(&args[1])?;
    let reader = BufReader::new(input);

    let mut w = XmlWriter::new(xmlwriter::Options::default());
    w.start_element("testExecutions");
    w.write_attribute("version", "1");

    let mut current_file: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("SF:") {
            if current_file.is_some() {
                w.end_element(); // chiudi file precedente
            }
            current_file = Some(line[3..].to_string());
            w.start_element("file");
            w.write_attribute("path", current_file.as_ref().unwrap());
        } else if line.starts_with("DA:") {
            let parts: Vec<&str> = line[3..].split(',').collect();
            if parts.len() == 2 {
                let line_num = parts[0];
                let hits: i32 = parts[1].parse().unwrap_or(0);
                let covered = if hits > 0 { "true" } else { "false" };
                w.start_element("lineToCover");
                w.write_attribute("lineNumber", line_num);
                w.write_attribute("covered", covered);
                w.end_element();
            }
        } else if line == "end_of_record" {
            if current_file.is_some() {
                w.end_element(); // chiudi file
                current_file = None;
            }
        }
    }

    w.end_element(); // chiudi testExecutions

    std::fs::write(&args[2], w.end_document())?;
    Ok(())
}