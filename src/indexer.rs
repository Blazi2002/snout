use std::fs;
use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Field, STORED, TEXT, Value};
use tantivy::{doc, Index, TantivyDocument};
use walkdir::WalkDir;

use crate::extract;

/// Cartella, relativa alla working directory, dove Snout salva l'indice su disco.
const INDEX_DIR: &str = ".snout_index";

/// Definisce la struttura dei documenti nell'indice: percorso e contenuto testuale.
pub fn build_schema() -> (Schema, Field, Field) {
    let mut schema_builder = Schema::builder();
    // STORED: il valore viene salvato e può essere restituito nei risultati.
    // TEXT: il valore viene analizzato e indicizzato per la ricerca full-text.
    let path_field = schema_builder.add_text_field("path", STORED);
    let body_field = schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    (schema, path_field, body_field)
}

/// Scansiona la cartella indicata, estrae il testo da ogni file e costruisce l'indice.
pub fn index_folder(folder: &str) -> tantivy::Result<usize> {
    let (schema, path_field, body_field) = build_schema();

    let index_path = Path::new(INDEX_DIR);
    if index_path.exists() {
        fs::remove_dir_all(index_path).ok();
    }
    fs::create_dir_all(index_path).ok();

    let index = Index::create_in_dir(index_path, schema)?;
    let mut writer = index.writer(50_000_000)?;

    let mut indexed = 0;

    for entry in WalkDir::new(folder) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        let content = match extract::extract_text(path) {
            Some(text) => text,
            None => continue,
        };

        writer.add_document(doc!(
            path_field => path.display().to_string(),
            body_field => content,
        ))?;
        indexed += 1;
    }

    writer.commit()?;

    Ok(indexed)
}

/// Interroga l'indice esistente cercando la query e restituisce i percorsi trovati.
pub fn search(query_text: &str, limit: usize) -> tantivy::Result<Vec<String>> {
    let index_path = Path::new(INDEX_DIR);
    let index = Index::open_in_dir(index_path)?;

    // Recuperiamo i campi dallo schema dell'indice aperto.
    let schema = index.schema();
    let path_field = schema.get_field("path").unwrap();
    let body_field = schema.get_field("body").unwrap();

    let reader = index.reader()?;
    let searcher = reader.searcher();

    // Il QueryParser interpreta il testo cercato sul campo "body".
    let query_parser = QueryParser::for_index(&index, vec![body_field]);
    let query = query_parser.parse_query(query_text)?;

    // TopDocs raccoglie i migliori risultati ordinati per rilevanza (punteggio BM25).
    let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved: TantivyDocument = searcher.doc(doc_address)?;
        if let Some(value) = retrieved.get_first(path_field) {
            if let Some(path_str) = value.as_str() {
                results.push(path_str.to_string());
            }
        }
    }

    Ok(results)
}
