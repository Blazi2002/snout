use std::collections::HashMap;

use crate::embed;
use crate::indexer;
use crate::vectors;

/// Costante k della formula RRF.
const RRF_K: f32 = 60.0;

/// Un risultato finale fuso: percorso, punteggio RRF e un estratto di testo.
pub struct HybridHit {
    pub path: String,
    pub score: f32,
    pub preview: String,
}

pub fn hybrid_search(query: &str, limit: usize) -> anyhow::Result<Vec<HybridHit>> {
    let fulltext: Vec<String> = indexer::search(query, 50).unwrap_or_default();

    let mut model = embed::load_model()?;
    let mut qvecs = embed::embed_texts(&mut model, vec![query.to_string()])?;
    let qvec = qvecs.remove(0);
    let semantic = vectors::semantic_search(&qvec, 50)?;

    // Mappa percorso -> testo del chunk piu' rilevante, per l'anteprima.
    let mut previews: HashMap<String, String> = HashMap::new();
    for hit in &semantic {
        previews.entry(hit.path.clone()).or_insert_with(|| hit.text.clone());
    }

    let mut scores: HashMap<String, f32> = HashMap::new();

    for (rank, path) in fulltext.iter().enumerate() {
        let contribution = 1.0 / (RRF_K + rank as f32 + 1.0);
        *scores.entry(path.clone()).or_insert(0.0) += contribution;
    }

    for (rank, hit) in semantic.iter().enumerate() {
        let contribution = 1.0 / (RRF_K + rank as f32 + 1.0);
        *scores.entry(hit.path.clone()).or_insert(0.0) += contribution;
    }

    let mut hits: Vec<HybridHit> = scores
        .into_iter()
        .map(|(path, score)| {
            let preview = previews.get(&path).cloned().unwrap_or_default();
            HybridHit { path, score, preview }
        })
        .collect();
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(limit);

    Ok(hits)
}
