#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Atmosphere {
    Silent,
    Fireplace,
    Rain,
    Night,
    Jazz,
}

impl Atmosphere {
    pub fn label(&self) -> &str {
        match self {
            Atmosphere::Silent => "Sessiz",
            Atmosphere::Fireplace => "Şömine",
            Atmosphere::Rain => "Yağmur",
            Atmosphere::Night => "Gece",
            Atmosphere::Jazz => "Jazz",
        }
    }

    #[allow(dead_code)]
    pub fn label_en(&self) -> &str {
        match self {
            Atmosphere::Silent => "Silent",
            Atmosphere::Fireplace => "Fireplace",
            Atmosphere::Rain => "Rain",
            Atmosphere::Night => "Night",
            Atmosphere::Jazz => "Jazz",
        }
    }

    pub fn css_class(&self) -> &str {
        match self {
            Atmosphere::Silent => "atmosphere-silent",
            Atmosphere::Fireplace => "atmosphere-fireplace",
            Atmosphere::Rain => "atmosphere-rain",
            Atmosphere::Night => "atmosphere-night",
            Atmosphere::Jazz => "atmosphere-jazz",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Atmosphere::Silent => "🔇",
            Atmosphere::Fireplace => "🔥",
            Atmosphere::Rain => "🌧️",
            Atmosphere::Night => "🌙",
            Atmosphere::Jazz => "🎷",
        }
    }

    pub fn all() -> Vec<Atmosphere> {
        vec![
            Atmosphere::Silent,
            Atmosphere::Fireplace,
            Atmosphere::Rain,
            Atmosphere::Night,
            Atmosphere::Jazz,
        ]
    }
}
