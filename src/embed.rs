use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

/// Carica il modello multilingue e5-small. Al primo avvio lo scarica da Hugging Face
/// e lo mette in cache locale; dalle volte successive lavora completamente offline.
pub fn load_model() -> anyhow::Result<TextEmbedding> {
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::MultilingualE5Small)
            .with_show_download_progress(true),
    )?;
    Ok(model)
}

/// Trasforma una lista di testi nei rispettivi vettori di embedding.
pub fn embed_texts(model: &mut TextEmbedding, texts: Vec<String>) -> anyhow::Result<Vec<Vec<f32>>> {
    let embeddings = model.embed(texts, None)?;
    Ok(embeddings)
}
