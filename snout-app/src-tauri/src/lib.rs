use serde::Serialize;

/// Risultato di ricerca pronto per essere inviato al front-end (serializzabile in JSON).
#[derive(Serialize)]
struct SearchResult {
    path: String,
    score: f32,
}

/// Command invocabile dal front-end: esegue la ricerca ibrida e restituisce i risultati.
#[tauri::command]
fn search_files(query: String) -> Result<Vec<SearchResult>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Chiamiamo il motore (ricerca ibrida: full-text + semantica).
    match snout::hybrid::hybrid_search(&query, 20) {
        Ok(hits) => {
            let results = hits
                .into_iter()
                .map(|h| SearchResult {
                    path: h.path,
                    score: h.score,
                })
                .collect();
            Ok(results)
        }
        // Convertiamo l'errore in stringa, cosi' il front-end puo' mostrarlo.
        Err(e) => Err(format!("Search failed: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
