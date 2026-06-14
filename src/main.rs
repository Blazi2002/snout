use std::env;
use std::process;

mod extract;
mod indexer;
mod embed;
mod chunker;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "index" => {
            if args.len() < 3 {
                eprintln!("Usage: snout index <folder>");
                process::exit(1);
            }
            let folder = &args[2];
            match indexer::index_folder(folder) {
                Ok((added, updated, unchanged)) => {
                    println!(
                        "Indexing complete: {} added, {} updated, {} unchanged.",
                        added, updated, unchanged
                    );
                }
                Err(e) => {
                    eprintln!("Indexing failed: {}", e);
                    process::exit(1);
                }
            }
        }
        "search" => {
            if args.len() < 3 {
                eprintln!("Usage: snout search <query>");
                process::exit(1);
            }
            let query = &args[2];
            match indexer::search(query, 10) {
                Ok(results) => {
                    if results.is_empty() {
                        println!("No results for '{}'.", query);
                    } else {
                        for path in results {
                            println!("{}", path);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Search failed: {}", e);
                    eprintln!("Did you run 'snout index <folder>' first?");
                    process::exit(1);
                }
            }
        }
        "embed-test" => {
            let text = if args.len() >= 3 {
                args[2].clone()
            } else {
                "il gatto dorme".to_string()
            };
            match embed::load_model() {
                Ok(mut model) => match embed::embed_texts(&mut model, vec![text.clone()]) {
                    Ok(vectors) => {
                        let v = &vectors[0];
                        println!("Frase: \"{}\"", text);
                        println!("Vettore di {} dimensioni.", v.len());
                        println!("Primi 5 valori: {:?}", &v[..5.min(v.len())]);
                    }
                    Err(e) => eprintln!("Embedding failed: {}", e),
                },
                Err(e) => eprintln!("Model load failed: {}", e),
            }
        }
        "chunk-test" => {
            if args.len() < 3 {
                eprintln!("Usage: snout chunk-test <text>");
                process::exit(1);
            }
            let chunks = chunker::chunk_text(&args[2]);
            println!("Il testo e' stato diviso in {} chunk:\n", chunks.len());
            for (i, c) in chunks.iter().enumerate() {
                println!("--- Chunk {} ({} caratteri) ---", i + 1, c.chars().count());
                println!("{}\n", c);
            }
        }
        other => {
            eprintln!("Unknown command: '{}'", other);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Snout - local file search");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  snout index <folder>    Build or update the search index");
    eprintln!("  snout search <query>    Search the index");
}
