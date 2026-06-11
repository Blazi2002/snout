use std::env;
use std::fs;
use std::process;

use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: snout <folder> <word>");
        process::exit(1);
    }
    let folder = &args[1];
    let query = &args[2];

    let mut total_matches = 0;

    // WalkDir scende ricorsivamente in ogni sottocartella a qualsiasi profondità
    for entry in WalkDir::new(folder) {
        let entry = match entry {
            Ok(e) => e,
            // Una voce non accessibile (permessi, link rotti) non deve fermare la ricerca
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        // I file non leggibili come testo (binari, immagini, ecc.) vengono ignorati
        let content = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => continue,
        };

        for (i, line) in content.lines().enumerate() {
            if line.contains(query.as_str()) {
                println!("{}:{}: {}", path.display(), i + 1, line.trim());
                total_matches += 1;
            }
        }
    }

    if total_matches == 0 {
        println!("No results for '{}'.", query);
    }
}