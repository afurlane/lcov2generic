use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use xmlwriter::XmlWriter;

/// Risale la gerarchia fino a trovare Cargo.toml e restituisce la root del progetto
fn project_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap();
    loop {
        if dir.join("Cargo.toml").exists() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    panic!("Cargo.toml non trovato!");
}

/// Converte un path assoluto in relativo rispetto alla root del progetto
fn relativize(path: &str, base: &Path) -> String {
    let abs = Path::new(path);
    match abs.strip_prefix(base) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => abs.to_string_lossy().to_string(),
    }
}

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
    let base_dir = project_root();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("SF:") {
            if current_file.is_some() {
                w.end_element(); // chiudi file precedente
            }
            let abs_path = &line[3..];
            let rel_path = relativize(abs_path, &base_dir);
            current_file = Some(rel_path);
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
