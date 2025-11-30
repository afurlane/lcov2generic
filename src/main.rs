use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use xmlwriter::{XmlWriter, Options};

/// Risale la gerarchia fino a trovare Cargo.toml e restituisce la root del progetto
fn project_root() -> std::io::Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    while !dir.join("Cargo.toml").exists() {
        if !dir.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cargo.toml non trovato",
            ));
        }
    }
    Ok(dir)
}

/// Converte un path assoluto in relativo rispetto alla root del progetto
fn relativize(path: &str, base: &Path) -> String {
    Path::new(path)
        .strip_prefix(base)
        .map(|rel| rel.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: lcov2generic <input.lcov> <output.xml>");
        std::process::exit(1);
    }

    let input = File::open(&args[1])?;
    let reader = BufReader::new(input);
    let base_dir = project_root()?;

    let mut w = XmlWriter::new(Options::default());
    w.start_element("coverage");
    w.write_attribute("version", "1");

    let mut current_file: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("SF:") {
            if current_file.is_some() {
                w.end_element(); // chiudi file precedente
            }
            let abs_path = &line[3..];
            let rel_path = relativize(abs_path, &base_dir);
            current_file = Some(rel_path.clone());
            w.start_element("file");
            w.write_attribute("path", &rel_path);
        } else if line.starts_with("DA:") {
            let parts: Vec<&str> = line[3..].split(',').collect();
            if let [line_num, hits] = parts.as_slice() {
                let covered = if hits.parse::<i32>().unwrap_or(0) > 0 {
                    "true"
                } else {
                    "false"
                };
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

    w.end_element(); // chiudi coverage
    std::fs::write(&args[2], w.end_document())?;
    Ok(())
}
