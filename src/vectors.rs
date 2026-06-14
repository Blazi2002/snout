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

pub fn reset() -> std::io::Result<()> {
    if Path::new(STORE_PATH).exists() {
        std::fs::remove_file(STORE_PATH)?;
    }
    Ok(())
}

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

/// Similarita' coseno tra due vettori: 1.0 = identici, 0.0 = non correlati.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for i in 0..a.len().min(b.len()) {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a.sqrt() * norm_b.sqrt())
}

/// Un risultato di ricerca semantica: percorso del file, punteggio e testo del chunk.
pub struct SemanticHit {
    pub path: String,
    pub score: f32,
    pub text: String,
}

/// Cerca i chunk semanticamente piu' vicini al vettore della query.
/// Restituisce al piu' un risultato per file (il chunk col punteggio piu' alto),
/// ordinati dal piu' rilevante.
pub fn semantic_search(query_vector: &[f32], limit: usize) -> std::io::Result<Vec<SemanticHit>> {
    let records = load_all()?;

    // Per ogni file teniamo solo il chunk col punteggio massimo.
    let mut best_per_file: std::collections::HashMap<String, SemanticHit> =
        std::collections::HashMap::new();

    for record in records {
        let score = cosine_similarity(query_vector, &record.vector);
        let entry = best_per_file.get(&record.path);
        let is_better = match entry {
            Some(existing) => score > existing.score,
            None => true,
        };
        if is_better {
            best_per_file.insert(
                record.path.clone(),
                SemanticHit {
                    path: record.path,
                    score,
                    text: record.text,
                },
            );
        }
    }

    let mut hits: Vec<SemanticHit> = best_per_file.into_values().collect();
    // Ordiniamo per punteggio decrescente.
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(limit);

    Ok(hits)
}
