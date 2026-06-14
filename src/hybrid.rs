use std::collections::HashMap;

use crate::embed;
use crate::indexer;
use crate::vectors;

/// Costante k della formula RRF. Valore standard in letteratura: smorza il peso
/// delle prime posizioni evitando che un singolo motore domini il risultato.
const RRF_K: f32 = 60.0;

/// Un risultato finale fuso: percorso del file e punteggio combinato RRF.
pub struct HybridHit {
    pub path: String,
    pub score: f32,
}

/// Esegue ricerca full-text e semantica sulla stessa query e fonde i ranking con
/// Reciprocal Rank Fusion. RRF usa la posizione (rank) in ciascuna lista, non i
/// punteggi grezzi, cosi' scale diverse (BM25 e coseno) si combinano in modo robusto.
pub fn hybrid_search(query: &str, limit: usize) -> anyhow::Result<Vec<HybridHit>> {
    // 1. Ricerca full-text: lista di percorsi ordinati per rilevanza BM25.
    let fulltext: Vec<String> = indexer::search(query, 50).unwrap_or_default();

    // 2. Ricerca semantica: generiamo il vettore della query e cerchiamo i chunk vicini.
    let mut model = embed::load_model()?;
    let mut qvecs = embed::embed_texts(&mut model, vec![query.to_string()])?;
    let qvec = qvecs.remove(0);
    let semantic = vectors::semantic_search(&qvec, 50)?;

    // 3. Accumuliamo il punteggio RRF di ogni file sommando i contributi delle due liste.
    let mut scores: HashMap<String, f32> = HashMap::new();

    for (rank, path) in fulltext.iter().enumerate() {
        let contribution = 1.0 / (RRF_K + rank as f32 + 1.0);
        *scores.entry(path.clone()).or_insert(0.0) += contribution;
    }

    for (rank, hit) in semantic.iter().enumerate() {
        let contribution = 1.0 / (RRF_K + rank as f32 + 1.0);
        *scores.entry(hit.path.clone()).or_insert(0.0) += contribution;
    }

    // 4. Ordiniamo per punteggio combinato decrescente.
    let mut hits: Vec<HybridHit> = scores
        .into_iter()
        .map(|(path, score)| HybridHit { path, score })
        .collect();
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(limit);

    Ok(hits)
}
