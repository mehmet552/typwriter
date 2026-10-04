#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperFormat {
    A4,    // A4 Standart Sayfa (Genişlik: 800px, Satır: ~60 karakter)
    Novel, // Roman / Kitap Formatı (Genişlik: 600px, Satır: ~44 karakter)
}

impl PaperFormat {
    #[allow(dead_code)]
    pub fn all() -> Vec<PaperFormat> {
        vec![PaperFormat::A4, PaperFormat::Novel]
    }

    #[allow(dead_code)]
    pub fn label_tr(&self) -> &'static str {
        match self {
            PaperFormat::A4 => "A4 Sayfa (~60 Karakter)",
            PaperFormat::Novel => "Roman / Kitap (~44 Karakter)",
        }
    }

    #[allow(dead_code)]
    pub fn label_en(&self) -> &'static str {
        match self {
            PaperFormat::A4 => "A4 Page (~60 Chars)",
            PaperFormat::Novel => "Novel / Book (~44 Chars)",
        }
    }

    pub fn width_pixels(&self) -> i32 {
        match self {
            PaperFormat::A4 => 800,
            PaperFormat::Novel => 600,
        }
    }

    pub fn line_capacity(&self) -> usize {
        match self {
            PaperFormat::A4 => 60,
            PaperFormat::Novel => 44,
        }
    }

    #[allow(dead_code)]
    pub fn bell_trigger_col(&self) -> usize {
        match self {
            PaperFormat::A4 => 54, // Satır sonuna 6 karakter kala daktilo uyarı zili çalar
            PaperFormat::Novel => 38,
        }
    }
}
