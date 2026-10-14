#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperFormat {
    A4,    // A4 standart sayfa (11 punto, satır başına yaklaşık 76 karakter)
    Novel, // Roman / Kitap Formatı (Genişlik: 600px, Satır: ~44 karakter)
}

impl PaperFormat {
    const PAGE_LINE_CAPACITY: usize = 60;

    #[allow(dead_code)]
    pub fn all() -> Vec<PaperFormat> {
        vec![PaperFormat::A4, PaperFormat::Novel]
    }

    #[allow(dead_code)]
    pub fn label_tr(&self) -> &'static str {
        match self {
            PaperFormat::A4 => "A4 Sayfa (~76 karakter/satır)",
            PaperFormat::Novel => "Roman / Kitap (~44 Karakter)",
        }
    }

    #[allow(dead_code)]
    pub fn label_en(&self) -> &'static str {
        match self {
            PaperFormat::A4 => "A4 Page (~76 chars/line)",
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
            PaperFormat::A4 => 76,
            PaperFormat::Novel => 44,
        }
    }

    pub fn page_line_capacity(&self) -> usize {
        Self::PAGE_LINE_CAPACITY
    }

    pub fn page_capacity(&self) -> usize {
        self.line_capacity() * self.page_line_capacity()
    }

    #[allow(dead_code)]
    pub fn bell_trigger_col(&self) -> usize {
        match self {
            PaperFormat::A4 => 70, // Satır sonuna 6 karakter kala daktilo uyarı zili çalar
            PaperFormat::Novel => 38,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PaperFormat;

    #[test]
    fn a4_capacity_matches_an_11_point_page() {
        assert_eq!(PaperFormat::A4.line_capacity(), 76);
        assert_eq!(PaperFormat::A4.page_line_capacity(), 60);
        assert_eq!(PaperFormat::A4.page_capacity(), 4_560);
    }
}
