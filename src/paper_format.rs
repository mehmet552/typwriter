#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperFormat {
    A4,    // A4 Sayfa: Standart resmi/belge kağıdı (satır sonu: ~70 karakter)
    Novel, // Roman / Kitap: Dar format edebi metin (satır sonu: ~52 karakter)
}

#[allow(dead_code)]
impl PaperFormat {
    pub fn all() -> Vec<PaperFormat> {
        vec![PaperFormat::A4, PaperFormat::Novel]
    }

    pub fn label_tr(&self) -> &'static str {
        match self {
            PaperFormat::A4 => "📄 A4 Formatı (~70 Karakter)",
            PaperFormat::Novel => "📖 Roman Formatı (~52 Karakter)",
        }
    }

    pub fn label_en(&self) -> &'static str {
        match self {
            PaperFormat::A4 => "📄 A4 Format (~70 Chars)",
            PaperFormat::Novel => "📖 Novel Format (~52 Chars)",
        }
    }

    pub fn width_pixels(&self) -> i32 {
        match self {
            PaperFormat::A4 => 800,
            PaperFormat::Novel => 600,
        }
    }

    pub fn line_margin_cols(&self) -> usize {
        match self {
            PaperFormat::A4 => 70,
            PaperFormat::Novel => 52,
        }
    }
}
