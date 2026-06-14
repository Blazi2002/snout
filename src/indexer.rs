use std::collections::HashMap;
use std::fs;
use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Field, STORED, STRING, TEXT, Value};
use tantivy::{doc, Index, TantivyDocument, Term};
use walkdir::WalkDir;

use crate::extract;
use crate::chunker;
use crate::embed;
use crate::vectors::{self, ChunkRecord};

const INDEX_DIR: &str = ".snout_index";

struct Fields {
    path: Field,
    body: Field,
    hash: Field,
}

fn build_schema() -> (Schema, Fields) {
    let mut b = Schema::builder();
    let path = b.add_text_field("path", STRING | STORED);
    let body = b.add_text_field("body", TEXT);
    let hash = b.add_text_field("hash", STRING | STORED);
    let schema = b.build();
    (schema, Fields { path, body, hash })
}

fn open_or_create() -> tantivy::Result<(Index, Fields)> {
    let index_path = Path::new(INDEX_DIR);
    let (schema, fields) = build_schema();

    let index = if index_path.exists() {
        Index::open_in_dir(index_path)?
    } else {
        fs::create_dir_all(index_path).ok();
        Index::create_in_dir(index_path, schema)?
    };

    Ok((index, fields))
}

fn file_hash(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    Some(blake3::hash(&bytes).to_hex().to_string())
}

fn existing_hashes(index: &Index, fields: &Fields) -> tantivy::Result<HashMap<String, String>> {
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let mut map = HashMap::new();

    for segment_reader in searcher.segment_readers() {
        let store = segment_reader.get_store_reader(0)?;
        for doc_result in store.iter::<TantivyDocument>(segment_reader.alive_bitset()) {
            let doc = doc_result?;
            let path = doc.get_first(fields.path).and_then(|v| v.as_str());
            let hash = doc.get_first(fields.hash).and_then(|v| v.as_str());
            if let (Some(p), Some(h)) = (path, hash) {
                map.insert(p.to_string(), h.to_string());
            }
        }
    }

    Ok(map)
}

/// Indicizza la cartella. Se `semantic` e' true, genera e salva anche gli embedding
/// dei chunk di ogni file (ricostruendo da zero lo store vettoriale).
pub fn index_folder(folder: &str, semantic: bool) -> anyhow::Result<(usize, usize, usize)> {
    let (index, fields) = open_or_create()?;
    let already = existing_hashes(&index, &fields)?;

    let mut writer = index.writer(50_000_000)?;

    // Per la parte semantica: azzeriamo lo store dei vettori e carichiamo il modello una volta.
    let mut model = if semantic {
        vectors::reset()?;
        Some(embed::load_model()?)
    } else {
        None
    };

    let mut added = 0;
    let mut updated = 0;
    let mut unchanged = 0;
    let mut seen_paths = Vec::new();

    for entry in WalkDir::new(folder) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let path_str = path.display().to_string();

        let current_hash = match file_hash(path) {
            Some(h) => h,
            None => continue,
        };
        let content = match extract::extract_text(path) {
            Some(text) => text,
            None => continue,
        };

        seen_paths.push(path_str.clone());

        let is_unchanged = already.get(&path_str) == Some(&current_hash);

        // La parte full-text usa l'indicizzazione incrementale.
        if is_unchanged {
            unchanged += 1;
        } else {
            if already.contains_key(&path_str) {
                writer.delete_term(Term::from_field_text(fields.path, &path_str));
                updated += 1;
            } else {
                added += 1;
            }

            writer.add_document(doc!(
                fields.path => path_str.clone(),
                fields.body => content.clone(),
                fields.hash => current_hash,
            ))?;
        }

        // La parte semantica rigenera i vettori per ogni file leggibile (store ricostruito).
        if let Some(ref mut m) = model {
            let chunks = chunker::chunk_text(&content);
            if !chunks.is_empty() {
                let vecs = embed::embed_texts(m, chunks.clone())?;
                let records: Vec<ChunkRecord> = chunks
                    .into_iter()
                    .zip(vecs.into_iter())
                    .map(|(text, vector)| ChunkRecord {
                        path: path_str.clone(),
                        text,
                        vector,
                    })
                    .collect();
                vectors::append(&records)?;
            }
        }
    }

    for old_path in already.keys() {
        if !seen_paths.contains(old_path) {
            writer.delete_term(Term::from_field_text(fields.path, old_path));
        }
    }

    writer.commit()?;

    Ok((added, updated, unchanged))
}

pub fn search(query_text: &str, limit: usize) -> tantivy::Result<Vec<String>> {
    let index_path = Path::new(INDEX_DIR);
    let index = Index::open_in_dir(index_path)?;

    let schema = index.schema();
    let path_field = schema.get_field("path").unwrap();
    let body_field = schema.get_field("body").unwrap();

    let reader = index.reader()?;
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![body_field]);
    let query = query_parser.parse_query(query_text)?;

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
