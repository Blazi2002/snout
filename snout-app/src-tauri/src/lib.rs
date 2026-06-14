use serde::Serialize;

/// Risultato di ricerca pronto per il front-end (serializzabile in JSON).
#[derive(Serialize)]
struct SearchResult {
    path: String,
    score: f32,
}

/// Esito dell'indicizzazione, da mostrare all'utente.
#[derive(Serialize)]
struct IndexSummary {
    added: usize,
    updated: usize,
    unchanged: usize,
}

/// Esegue la ricerca ibrida e restituisce i risultati.
#[tauri::command]
fn search_files(query: String) -> Result<Vec<SearchResult>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    match snout::hybrid::hybrid_search(&query, 20) {
        Ok(hits) => {
            let results = hits
                .into_iter()
                .map(|h| SearchResult { path: h.path, score: h.score })
                .collect();
            Ok(results)
        }
        Err(e) => Err(format!("Search failed: {}", e)),
    }
}

/// Indicizza la cartella indicata, con embedding semantici, e restituisce un riepilogo.
#[tauri::command]
fn index_folder(path: String) -> Result<IndexSummary, String> {
    match snout::indexer::index_folder(&path, true) {
        Ok((added, updated, unchanged)) => Ok(IndexSummary { added, updated, unchanged }),
        Err(e) => Err(format!("Indexing failed: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![search_files, index_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
