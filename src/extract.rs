use std::fs;
use std::path::Path;

/// Estrae il testo da un file in base alla sua estensione.
/// Restituisce None se il formato non è supportato o il file non è leggibile.
pub fn extract_text(path: &Path) -> Option<String> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "docx" => extract_docx(path),
        "pdf" => extract_pdf(path),
        "xlsx" | "xls" | "xlsm" | "ods" => extract_spreadsheet(path),
        // I formati testuali noti vengono letti direttamente
        "txt" | "md" | "csv" | "json" | "xml" | "html" | "log" => {
            fs::read_to_string(path).ok()
        }
        // Estensione sconosciuta: tentiamo comunque una lettura come testo,
        // così non perdiamo file di codice o formati testuali non elencati
        _ => fs::read_to_string(path).ok(),
    }
}

fn extract_pdf(path: &Path) -> Option<String> {
    // pdf-extract gestisce font, encoding e compressione interni al PDF
    pdf_extract::extract_text(path).ok()
}

/// Legge un foglio di calcolo trasformando ogni riga di celle in una riga di testo.
fn extract_spreadsheet(path: &Path) -> Option<String> {
    use calamine::{open_workbook_auto, Reader};

    let mut workbook = open_workbook_auto(path).ok()?;
    let mut output = String::new();

    // Un file può contenere più fogli: li attraversiamo tutti
    let sheet_names = workbook.sheet_names().to_owned();
    for name in sheet_names {
        if let Ok(range) = workbook.worksheet_range(&name) {
            for row in range.rows() {
                // Ogni cella diventa testo; le celle di una riga sono separate da tab
                let cells: Vec<String> = row.iter().map(|c| c.to_string()).collect();
                output.push_str(&cells.join("\t"));
                output.push('\n');
            }
        }
    }

    Some(output)
}

fn extract_docx(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let docx = docx_rs::read_docx(&bytes).ok()?;

    // Concateniamo il testo di tutti i paragrafi del documento
    let mut output = String::new();
    collect_text(&docx.document.children, &mut output);
    Some(output)
}

/// Attraversa la struttura del documento DOCX raccogliendo il testo contenuto.
fn collect_text(children: &[docx_rs::DocumentChild], output: &mut String) {
    use docx_rs::{DocumentChild, ParagraphChild, RunChild};

    for child in children {
        if let DocumentChild::Paragraph(p) = child {
            for pc in &p.children {
                if let ParagraphChild::Run(run) = pc {
                    for rc in &run.children {
                        if let RunChild::Text(t) = rc {
                            output.push_str(&t.text);
                        }
                    }
                }
            }
            output.push('\n');
        }
    }
}
