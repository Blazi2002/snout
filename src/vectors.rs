use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Un record semantico: a quale file appartiene, il testo del chunk e il suo vettore.
#[derive(Serialize, Deserialize)]
pub struct ChunkRecord {
    pub path: String,
    pub text: String,
    pub vector: Vec<f32>,
}

const STORE_PATH: &str = ".snout_index/embeddings.jsonl";

/// Azzera il file dei vettori (usato a inizio reindicizzazione semantica completa).
pub fn reset() -> std::io::Result<()> {
    if Path::new(STORE_PATH).exists() {
        std::fs::remove_file(STORE_PATH)?;
    }
    Ok(())
}

/// Aggiunge in coda al file un gruppo di record (una riga JSON ciascuno).
pub fn append(records: &[ChunkRecord]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(STORE_PATH)?;

    for record in records {
        let line = serde_json::to_string(record)?;
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

/// Rilegge tutti i record salvati. Usato in fase di ricerca semantica.
pub fn load_all() -> std::io::Result<Vec<ChunkRecord>> {
    let mut records = Vec::new();
    if !Path::new(STORE_PATH).exists() {
        return Ok(records);
    }

    let file = File::open(STORE_PATH)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(record) = serde_json::from_str::<ChunkRecord>(&line) {
            records.push(record);
        }
    }
    Ok(records)
}
