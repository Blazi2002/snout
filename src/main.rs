use std::env;
use std::process;

mod extract;
mod indexer;

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
                Ok(count) => println!("Indexed {} files into the index.", count),
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
    eprintln!("  snout index <folder>    Build the search index from a folder");
    eprintln!("  snout search <query>    Search the index");
}
