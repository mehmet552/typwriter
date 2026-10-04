use rand::Rng;
use rodio::Source;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

// Gerçek daktilo sesleri (Kullanıcının yüklediği ses dosyasından çıkarılan orijinal sesler)
static SOUND_KEY1: &[u8] = include_bytes!("../data/sounds/key1.wav");
static SOUND_KEY2: &[u8] = include_bytes!("../data/sounds/key2.wav");
static SOUND_KEY3: &[u8] = include_bytes!("../data/sounds/key3.wav");
static SOUND_KEY4: &[u8] = include_bytes!("../data/sounds/key4.wav");
static SOUND_KEY5: &[u8] = include_bytes!("../data/sounds/key5.wav");
static SOUND_KEY6: &[u8] = include_bytes!("../data/sounds/key6.wav");
static SOUND_SPACE: &[u8] = include_bytes!("../data/sounds/space.wav");
static SOUND_BACKSPACE: &[u8] = include_bytes!("../data/sounds/backspace.wav");
static SOUND_BELL: &[u8] = include_bytes!("../data/sounds/bell.wav");
static SOUND_RETURN: &[u8] = include_bytes!("../data/sounds/return.wav");

// Gerçek Şömine Sesi (Kullanıcının yüklediği orijinal şömine çıtırtı kaydı)
static SOUND_FIREPLACE: &[u8] = include_bytes!("../data/sounds/fireplace.wav");

// Gerçek Gece Sesi (Kullanıcının yüklediği orijinal gece atmosferi kaydı)
static SOUND_NIGHT: &[u8] = include_bytes!("../data/sounds/night.wav");

// Gerçek Yağmur Sesi (Kullanıcının yüklediği orijinal yağmur atmosferi kaydı)
static SOUND_RAIN: &[u8] = include_bytes!("../data/sounds/rain.wav");

fn decode_wav(bytes: &'static [u8]) -> Vec<f32> {
    if let Ok(decoder) = rodio::Decoder::new(std::io::Cursor::new(bytes)) {
        return decoder.convert_samples::<f32>().collect();
    }
    vec![]
}

#[allow(dead_code)]
pub struct SoundManager {
    _stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    ambient_sink: RefCell<Option<rodio::Sink>>,
    key_sounds: Vec<Vec<f32>>,
    space_sound: Vec<f32>,
    enter_sound: Vec<f32>,
    backspace_sound: Vec<f32>,
    bell_sound: Vec<f32>,
    ambient_sounds: HashMap<String, Vec<f32>>,
    typing_enabled: Cell<bool>,
    ambient_volume: Cell<f32>,
    current_click: Cell<usize>,
}

#[allow(dead_code)]
impl SoundManager {
    pub fn new() -> Option<Self> {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().ok()?;

        // 1. Orijinal Daktilo Tuş Sesleri (6 varyasyon)
        let mut key_sounds = vec![
            decode_wav(SOUND_KEY1),
            decode_wav(SOUND_KEY2),
            decode_wav(SOUND_KEY3),
            decode_wav(SOUND_KEY4),
            decode_wav(SOUND_KEY5),
            decode_wav(SOUND_KEY6),
        ];
        key_sounds.retain(|s| !s.is_empty());

        // 2. Boşluk, Geri, Satır Başı (Başa kaydırma ve tik sesi), Satır Sonu Zili
        let space_sound = decode_wav(SOUND_SPACE);
        let backspace_sound = decode_wav(SOUND_BACKSPACE);
        let enter_sound = decode_wav(SOUND_RETURN);
        let bell_sound = decode_wav(SOUND_BELL);

        // 3. Atmosfer Ortam Sesleri
        let mut ambient_sounds = HashMap::new();

        // Şömine: Kullanıcının yüklediği gerçek şömine çıtırtısı (kesintisiz dikişsiz döngü)
        let fireplace_samples = decode_wav(SOUND_FIREPLACE);
        if !fireplace_samples.is_empty() {
            ambient_sounds.insert("Fireplace".to_string(), fireplace_samples);
        }

        let mut rng = rand::thread_rng();
        let sample_rate = 44100.0;
        let pi2 = 2.0 * std::f32::consts::PI;
        let loop_duration = 8.0;
        let loop_samples = (loop_duration * sample_rate) as usize;

        // Yağmur: Kullanıcının yüklediği gerçek yağmur atmosferi (kesintisiz döngü)
        let rain_samples = decode_wav(SOUND_RAIN);
        if !rain_samples.is_empty() {
            ambient_sounds.insert("Rain".to_string(), rain_samples);
        }

        // Gece: Kullanıcının yüklediği gerçek gece atmosferi (kesintisiz döngü)
        let night_samples = decode_wav(SOUND_NIGHT);
        if !night_samples.is_empty() {
            ambient_sounds.insert("Night".to_string(), night_samples);
        }

        // Kafe
        let mut cafe = Vec::with_capacity(loop_samples);
        let mut hist3 = vec![0.0; 3];
        let mut h3_idx = 0;
        for i in 0..loop_samples {
            let t = i as f32 / sample_rate;
            let white: f32 = rng.gen_range(-1.0..1.0);
            hist3[h3_idx] = white;
            h3_idx = (h3_idx + 1) % 3;
            let avg = hist3.iter().sum::<f32>() / 3.0;

            let mod1 = 0.5 + 0.3 * (pi2 * 0.3 * t).sin();
            let mod2 = 0.6 + 0.2 * (pi2 * 0.7 * t).sin();
            let mut conv = 0.0;
            if rng.gen::<f32>() < 0.00005 {
                conv = rng.gen_range(0.2..0.5);
            }
            cafe.push((avg * mod1 * mod2 + conv) * 0.08);
        }
        ambient_sounds.insert("Cafe".to_string(), cafe);

        Some(Self {
            _stream,
            stream_handle,
            ambient_sink: RefCell::new(None),
            key_sounds,
            space_sound,
            enter_sound,
            backspace_sound,
            bell_sound,
            ambient_sounds,
            typing_enabled: Cell::new(true),
            ambient_volume: Cell::new(1.0),
            current_click: Cell::new(0),
        })
    }

    fn play_oneshot(&self, samples: &[f32]) {
        if !self.typing_enabled.get() || samples.is_empty() {
            return;
        }
        let source = rodio::buffer::SamplesBuffer::new(1, 44100, samples.to_vec());
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }

    pub fn play_key_click(&self) {
        if self.key_sounds.is_empty() {
            return;
        }
        let idx = self.current_click.get();
        self.play_oneshot(&self.key_sounds[idx]);
        self.current_click
            .set((idx + 1) % self.key_sounds.len());
    }

    pub fn play_space(&self) {
        self.play_oneshot(&self.space_sound);
    }

    pub fn play_enter(&self) {
        self.play_oneshot(&self.enter_sound);
    }

    pub fn play_backspace(&self) {
        self.play_oneshot(&self.backspace_sound);
    }

    pub fn play_bell(&self) {
        self.play_oneshot(&self.bell_sound);
    }

    fn stop_ambient(&self) {
        if let Some(sink) = self.ambient_sink.borrow().as_ref() {
            sink.stop();
        }
        *self.ambient_sink.borrow_mut() = None;
    }

    pub fn set_atmosphere(&self, atm: crate::atmosphere::Atmosphere) {
        self.stop_ambient();
        let key = format!("{:?}", atm);
        if let Some(samples) = self.ambient_sounds.get(&key) {
            if let Some(sink) = rodio::Sink::try_new(&self.stream_handle).ok() {
                let source = rodio::buffer::SamplesBuffer::new(1, 44100, samples.clone());
                sink.append(source.repeat_infinite());
                sink.set_volume(self.ambient_volume.get());
                *self.ambient_sink.borrow_mut() = Some(sink);
            }
        }
    }

    pub fn set_ambient_volume(&self, vol: f32) {
        self.ambient_volume.set(vol);
        if let Some(sink) = self.ambient_sink.borrow().as_ref() {
            sink.set_volume(vol);
        }
    }

    pub fn set_typing_enabled(&self, enabled: bool) {
        self.typing_enabled.set(enabled);
    }
}
