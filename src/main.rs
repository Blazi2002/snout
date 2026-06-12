use std::env;
use std::process;

use walkdir::WalkDir;

mod extract;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: snout <folder> <word>");
        process::exit(1);
    }
    let folder = &args[1];
    let query = &args[2];

    let mut total_matches = 0;

    for entry in WalkDir::new(folder) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        // Delega l'estrazione del testo al modulo extract, che sceglie
        // la strategia giusta in base al tipo di file (DOCX, testo, ecc.)
        let content = match extract::extract_text(path) {
            Some(text) => text,
            None => continue,
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
