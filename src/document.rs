use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

pub fn load_document(path: &Path) -> Result<String, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "docx" {
        read_docx(path)
    } else {
        std::fs::read_to_string(path).map_err(|e| format!("Dosya okunamadı: {}", e))
    }
}

pub fn save_document(path: &Path, content: &str) -> Result<(), String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "docx" {
        write_docx(path, content)
    } else {
        std::fs::write(path, content).map_err(|e| format!("Dosya kaydedilemedi: {}", e))
    }
}

fn read_docx(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("DOCX açılamadı: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Geçersiz DOCX arşivi: {}", e))?;

    let mut xml = String::new();
    {
        let mut doc_file = archive
            .by_name("word/document.xml")
            .map_err(|_| "DOCX içinde word/document.xml bulunamadı".to_string())?;
        doc_file
            .read_to_string(&mut xml)
            .map_err(|e| format!("XML okunamadı: {}", e))?;
    }

    Ok(extract_text_from_word_xml(&xml))
}

fn extract_text_from_word_xml(xml: &str) -> String {
    let mut result = String::new();
    let mut in_text = false;
    let mut current_text = String::new();
    let chars: Vec<char> = xml.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '<' {
            let start = i;
            while i < chars.len() && chars[i] != '>' {
                i += 1;
            }
            if i < chars.len() {
                let tag: String = chars[start..=i].iter().collect();
                if tag.starts_with("<w:p ") || tag == "<w:p>" {
                    if !result.is_empty() && !result.ends_with('\n') {
                        result.push('\n');
                    }
                } else if tag.starts_with("<w:t") {
                    in_text = true;
                    current_text.clear();
                } else if tag == "</w:t>" {
                    in_text = false;
                    result.push_str(&unescape_xml(&current_text));
                    current_text.clear();
                } else if tag == "<w:br/>" || tag.starts_with("<w:br ") {
                    result.push('\n');
                }
            }
        } else if in_text {
            current_text.push(chars[i]);
        }
        i += 1;
    }

    result
}

fn unescape_xml(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn write_docx(path: &Path, content: &str) -> Result<(), String> {
    let file = File::create(path).map_err(|e| format!("Dosya oluşturulamadı: {}", e))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // 1. [Content_Types].xml
    zip.start_file("[Content_Types].xml", options)
        .map_err(|e| e.to_string())?;
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#,
    )
    .map_err(|e| e.to_string())?;

    // 2. _rels/.rels
    zip.start_file("_rels/.rels", options)
        .map_err(|e| e.to_string())?;
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#,
    )
    .map_err(|e| e.to_string())?;

    // 3. word/document.xml
    zip.start_file("word/document.xml", options)
        .map_err(|e| e.to_string())?;
    let mut doc_xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
"#,
    );

    for line in content.lines() {
        let escaped = escape_xml(line);
        doc_xml.push_str(&format!(
            "    <w:p><w:r><w:t xml:space=\"preserve\">{}</w:t></w:r></w:p>\n",
            escaped
        ));
    }

    if content.is_empty() {
        doc_xml.push_str("    <w:p/>\n");
    }

    doc_xml.push_str(
        r#"  </w:body>
</w:document>"#,
    );

    zip.write_all(doc_xml.as_bytes())
        .map_err(|e| e.to_string())?;
    zip.finish().map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_docx_roundtrip() {
        let test_path = Path::new("/tmp/test_typwriter.docx");
        let sample_text = "İstanbul'da bir daktilo sesi...\nŞömine çıtırtısı ve yağmur eşliğinde 1984 romanı.\nÖzel karakterler: <test> & \"alıntı\" 'tek tırnak'.";

        assert!(save_document(test_path, sample_text).is_ok());
        let loaded = load_document(test_path).expect("DOCX yüklenemedi");
        assert_eq!(sample_text, loaded);

        let _ = fs::remove_file(test_path);
    }

    #[test]
    fn test_txt_roundtrip() {
        let test_path = Path::new("/tmp/test_typwriter.txt");
        let sample_text = "Düz metin belgesi test satırı.\nİkinci satır.";

        assert!(save_document(test_path, sample_text).is_ok());
        let loaded = load_document(test_path).expect("TXT yüklenemedi");
        assert_eq!(sample_text, loaded);

        let _ = fs::remove_file(test_path);
    }
}
