use std::path::PathBuf;

use directories::ProjectDirs;

/// Restituisce la cartella dati di Snout nell'area standard del sistema operativo
/// (es. ~/Library/Application Support/Snout su macOS), creandola se non esiste.
/// In caso estremo di fallimento, ripiega su una cartella locale.
pub fn data_dir() -> PathBuf {
    let dir = ProjectDirs::from("com", "blazi2002", "Snout")
        .map(|p| p.data_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".snout_data"));
    std::fs::create_dir_all(&dir).ok();
    dir
}

/// Percorso della cartella dell'indice full-text (Tantivy).
pub fn index_dir() -> PathBuf {
    data_dir().join("index")
}

/// Percorso del file degli embedding semantici.
pub fn embeddings_path() -> PathBuf {
    data_dir().join("embeddings.jsonl")
}
