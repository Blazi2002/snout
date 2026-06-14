use serde::Serialize;

#[derive(Serialize)]
struct SearchResult {
    path: String,
    score: f32,
}

#[derive(Serialize)]
struct IndexSummary {
    added: usize,
    updated: usize,
    unchanged: usize,
}

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

#[tauri::command]
fn index_folder(path: String) -> Result<IndexSummary, String> {
    match snout::indexer::index_folder(&path, true) {
        Ok((added, updated, unchanged)) => Ok(IndexSummary { added, updated, unchanged }),
        Err(e) => Err(format!("Indexing failed: {}", e)),
    }
}

/// Apre un file nell'applicazione predefinita del sistema operativo.
#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    // Usa il comando di sistema appropriato per aprire il file con l'app di default.
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(&path).spawn();

    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd").args(["/C", "start", "", &path]).spawn();

    #[cfg(target_os = "linux")]
    let result = std::process::Command::new("xdg-open").arg(&path).spawn();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Could not open file: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![search_files, index_folder, open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
