#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Atmosphere {
    Silent,
    Fireplace,
    Rain,
    Night,
    Cafe,
}

impl Atmosphere {
    pub fn label(&self) -> &str {
        match self {
            Atmosphere::Silent => "Sessiz",
            Atmosphere::Fireplace => "Şömine",
            Atmosphere::Rain => "Yağmur",
            Atmosphere::Night => "Gece",
            Atmosphere::Cafe => "Kafe",
        }
    }

    pub fn css_class(&self) -> &str {
        match self {
            Atmosphere::Silent => "atmosphere-silent",
            Atmosphere::Fireplace => "atmosphere-fireplace",
            Atmosphere::Rain => "atmosphere-rain",
            Atmosphere::Night => "atmosphere-night",
            Atmosphere::Cafe => "atmosphere-cafe",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Atmosphere::Silent => "🔇",
            Atmosphere::Fireplace => "🔥",
            Atmosphere::Rain => "🌧️",
            Atmosphere::Night => "🌙",
            Atmosphere::Cafe => "☕",
        }
    }

    pub fn all() -> Vec<Atmosphere> {
        vec![
            Atmosphere::Silent,
            Atmosphere::Fireplace,
            Atmosphere::Rain,
            Atmosphere::Night,
            Atmosphere::Cafe,
        ]
    }
}
