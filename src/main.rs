use std::env;
use std::process;

mod extract;
mod indexer;
mod embed;
mod chunker;
mod vectors;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "index" => {
            if args.len() < 3 {
                eprintln!("Usage: snout index <folder> [--semantic]");
                process::exit(1);
            }
            let folder = &args[2];
            let semantic = args.iter().any(|a| a == "--semantic");

            if semantic {
                println!("Indexing with semantic embeddings (this may take a while)...");
            }

            match indexer::index_folder(folder, semantic) {
                Ok((added, updated, unchanged)) => {
                    println!(
                        "Indexing complete: {} added, {} updated, {} unchanged.",
                        added, updated, unchanged
                    );
                    if semantic {
                        println!("Semantic embeddings generated.");
                    }
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
        "semantic" => {
            if args.len() < 3 {
                eprintln!("Usage: snout semantic <query>");
                process::exit(1);
            }
            let query = args[2].clone();
            match embed::load_model() {
                Ok(mut model) => match embed::embed_texts(&mut model, vec![query.clone()]) {
                    Ok(mut qvecs) => {
                        let qvec = qvecs.remove(0);
                        match vectors::semantic_search(&qvec, 5) {
                            Ok(hits) => {
                                if hits.is_empty() {
                                    println!("No semantic results. Did you index with --semantic?");
                                } else {
                                    for hit in hits {
                                        println!("[{:.3}] {}", hit.score, hit.path);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Semantic search failed: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Query embedding failed: {}", e),
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
    eprintln!("  snout index <folder> [--semantic]   Build or update the index");
    eprintln!("  snout search <query>                Search the index (full-text)");
    eprintln!("  snout semantic <query>              Search by meaning (semantic)");
}
