use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

use crate::paths;

/// Carica il modello multilingue e5-small. Al primo avvio lo scarica da Hugging Face
/// nella cartella dati dell'applicazione (scrivibile e stabile a prescindere da dove
/// viene lanciata l'app); dalle volte successive lavora completamente offline.
pub fn load_model() -> anyhow::Result<TextEmbedding> {
    let cache_dir = paths::data_dir().join("models");
    std::fs::create_dir_all(&cache_dir).ok();

    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::MultilingualE5Small)
            .with_cache_dir(cache_dir)
            .with_show_download_progress(true),
    )?;
    Ok(model)
}

/// Trasforma una lista di testi nei rispettivi vettori di embedding.
pub fn embed_texts(model: &mut TextEmbedding, texts: Vec<String>) -> anyhow::Result<Vec<Vec<f32>>> {
    let embeddings = model.embed(texts, None)?;
    Ok(embeddings)
}
