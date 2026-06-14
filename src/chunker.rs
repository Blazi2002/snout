/// Dimensione target di un chunk, in caratteri. I chunk possono superarla leggermente
/// per non spezzare una frase a meta'.
const TARGET_CHUNK_SIZE: usize = 500;

/// Quanti caratteri di coda del chunk precedente vengono ripetuti all'inizio del successivo.
/// L'overlap evita di perdere concetti che cadono a cavallo tra due chunk.
const OVERLAP: usize = 80;

/// Divide un testo in chunk rispettando i confini di frase, con overlap tra i chunk.
pub fn chunk_text(text: &str) -> Vec<String> {
    let normalized: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return Vec::new();
    }

    let sentences = split_into_sentences(&normalized);

    let mut chunks = Vec::new();
    let mut current = String::new();

    for sentence in sentences {
        if current.chars().count() + sentence.chars().count() > TARGET_CHUNK_SIZE
            && !current.is_empty()
        {
            chunks.push(current.trim().to_string());
            current = overlap_tail(&current);
        }

        current.push(' ');
        current.push_str(&sentence);
    }

    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }

    chunks
}

/// Restituisce la coda del chunk da usare come overlap, facendola iniziare sempre
/// all'inizio di una parola intera (mai a meta' di una parola).
fn overlap_tail(chunk: &str) -> String {
    let tail: String = chunk
        .chars()
        .rev()
        .take(OVERLAP)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    // Se la coda comincia a meta' di una parola, scartiamo i caratteri fino al primo spazio.
    match tail.find(' ') {
        Some(pos) => tail[pos + 1..].to_string(),
        None => tail,
    }
}

/// Divide il testo in frasi, mantenendo la punteggiatura finale come parte della frase.
fn split_into_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        current.push(ch);
        if ch == '.' || ch == '!' || ch == '?' {
            let trimmed = current.trim();
            if !trimmed.is_empty() {
                sentences.push(trimmed.to_string());
            }
            current = String::new();
        }
    }

    let trimmed = current.trim();
    if !trimmed.is_empty() {
        sentences.push(trimmed.to_string());
    }

    sentences
}
