use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Attesi due argomenti: la cartella in cui cercare e la parola da trovare
    if args.len() < 3 {
        eprintln!("Uso: snout <cartella> <parola>");
        process::exit(1);
    }
    let cartella = &args[1];
    let parola_cercata = &args[2];

    // Leggiamo l'elenco dei file contenuti nella cartella indicata
    let elementi = fs::read_dir(cartella)
        .expect("Non riesco ad aprire la cartella");

    let mut risultati_totali = 0;

    for elemento in elementi {
        let percorso = elemento.expect("Elemento di cartella non valido").path();

        // Per ora ignoriamo le sottocartelle: trattiamo solo i file
        if !percorso.is_file() {
            continue;
        }

        // Se un file non è leggibile come testo (es. binario), lo saltiamo senza bloccarci
        let contenuto = match fs::read_to_string(&percorso) {
            Ok(testo) => testo,
            Err(_) => continue,
        };

        for (i, riga) in contenuto.lines().enumerate() {
            if riga.contains(parola_cercata.as_str()) {
                println!("{}:{}: {}", percorso.display(), i + 1, riga.trim());
                risultati_totali += 1;
            }
        }
    }

    if risultati_totali == 0 {
        println!("Nessun risultato per '{}'.", parola_cercata);
    }
}