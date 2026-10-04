#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Turkish,
    English,
}

#[allow(dead_code)]
impl Language {
    pub fn all() -> Vec<Language> {
        vec![Language::Turkish, Language::English]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::Turkish => "Türkçe",
            Language::English => "English",
        }
    }
}

#[allow(dead_code)]
pub struct Strings {
    pub app_title: &'static str,
    pub subtitle_default: &'static str,
    pub open_tooltip: &'static str,
    pub save_tooltip: &'static str,
    pub save_as_tooltip: &'static str,
    pub fullscreen_tooltip: &'static str,
    pub settings_tooltip: &'static str,
    pub words_label: &'static str,
    pub chars_label: &'static str,
    pub saved_prefix: &'static str,
    pub opened_prefix: &'static str,
    pub save_dialog_title: &'static str,
    pub open_dialog_title: &'static str,
    pub animation_switch: &'static str,
    pub sound_switch: &'static str,
    pub language_label: &'static str,
    pub atmosphere_label: &'static str,
    pub atm_silent: &'static str,
    pub atm_fireplace: &'static str,
    pub atm_rain: &'static str,
    pub atm_night: &'static str,
    pub atm_cafe: &'static str,
}

pub fn get_strings(lang: Language) -> Strings {
    match lang {
        Language::Turkish => Strings {
            app_title: "Typwriter",
            subtitle_default: "Daktilo Odaklanma Alanı",
            open_tooltip: "Belge Aç (Ctrl+O)",
            save_tooltip: "Kaydet (Ctrl+S)",
            save_as_tooltip: "Farklı Kaydet (Ctrl+Shift+S)",
            fullscreen_tooltip: "Tam Ekran (F11)",
            settings_tooltip: "Ayarlar",
            words_label: "Kelimeler",
            chars_label: "Karakterler",
            saved_prefix: "Kaydedildi",
            opened_prefix: "Dosya",
            save_dialog_title: "Belge Kaydet",
            open_dialog_title: "Belge Aç (.docx, .txt, .md)",
            animation_switch: "Daktilo Animasyonu",
            sound_switch: "Daktilo Sesleri",
            language_label: "Dil / Language",
            atmosphere_label: "Atmosfer",
            atm_silent: "Sessiz",
            atm_fireplace: "Şömine",
            atm_rain: "Yağmur",
            atm_night: "Gece",
            atm_cafe: "Kafe",
        },
        Language::English => Strings {
            app_title: "Typwriter",
            subtitle_default: "Typewriter Focus Space",
            open_tooltip: "Open Document (Ctrl+O)",
            save_tooltip: "Save (Ctrl+S)",
            save_as_tooltip: "Save As (Ctrl+Shift+S)",
            fullscreen_tooltip: "Fullscreen (F11)",
            settings_tooltip: "Settings",
            words_label: "Words",
            chars_label: "Characters",
            saved_prefix: "Saved",
            opened_prefix: "File",
            save_dialog_title: "Save Document",
            open_dialog_title: "Open Document (.docx, .txt, .md)",
            animation_switch: "Typewriter Animation",
            sound_switch: "Typewriter Sounds",
            language_label: "Language / Dil",
            atmosphere_label: "Atmosphere",
            atm_silent: "Silent",
            atm_fireplace: "Fireplace",
            atm_rain: "Rain",
            atm_night: "Night",
            atm_cafe: "Cafe",
        },
    }
}
