use rand::Rng;
use rodio::Source;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

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

        let mut rng = rand::thread_rng();
        let sample_rate = 44100.0;
        let pi2 = 2.0 * std::f32::consts::PI;

        // Key Click (4 variations)
        let mut key_sounds = Vec::new();
        let variations = [
            (60.0, 3000.0),
            (80.0, 4000.0),
            (70.0, 3500.0),
            (90.0, 4500.0),
        ];

        for (decay_rate, freq) in variations.iter() {
            let duration = 0.04;
            let num_samples = (duration * sample_rate) as usize;
            let mut samples = Vec::with_capacity(num_samples);
            for i in 0..num_samples {
                let t = i as f32 / sample_rate;
                let noise: f32 = rng.gen_range(-1.0..1.0);
                let env = (-t * decay_rate).exp();
                let osc = (pi2 * freq * t).sin().abs().max(0.3);
                samples.push(noise * env * osc * 0.4);
            }
            key_sounds.push(samples);
        }

        // Space bar sound
        let space_duration = 0.06;
        let space_samples = (space_duration * sample_rate) as usize;
        let mut space_sound = Vec::with_capacity(space_samples);
        for i in 0..space_samples {
            let t = i as f32 / sample_rate;
            let noise: f32 = rng.gen_range(-1.0..1.0);
            let env = (-t * 40.0).exp();
            let osc = (pi2 * 2000.0 * t).sin().abs().max(0.3);
            space_sound.push(noise * env * osc * 0.5);
        }

        // Enter/Carriage Return
        let enter_duration = 0.25;
        let enter_samples = (enter_duration * sample_rate) as usize;
        let mut enter_sound = Vec::with_capacity(enter_samples);
        for i in 0..enter_samples {
            let t = i as f32 / sample_rate;
            if t < 0.15 {
                let noise: f32 = rng.gen_range(-1.0..1.0);
                let freq = 2000.0 - (1500.0 * (t / 0.15));
                let osc = (pi2 * freq * t).sin();
                enter_sound.push(noise * osc * 0.3);
            } else {
                let local_t = t - 0.15;
                let noise: f32 = rng.gen_range(-1.0..1.0);
                let env = (-local_t * 50.0).exp();
                let osc = (pi2 * 200.0 * local_t).sin();
                enter_sound.push(noise * env * osc * 0.4);
            }
        }

        // Backspace
        let bs_duration = 0.025;
        let bs_samples = (bs_duration * sample_rate) as usize;
        let mut backspace_sound = Vec::with_capacity(bs_samples);
        for i in 0..bs_samples {
            let t = i as f32 / sample_rate;
            let noise: f32 = rng.gen_range(-1.0..1.0);
            let env = (-t * 100.0).exp();
            let osc = (pi2 * 5000.0 * t).sin().abs().max(0.3);
            backspace_sound.push(noise * env * osc * 0.2);
        }

        // Bell
        let bell_duration = 0.8;
        let bell_samples = (bell_duration * sample_rate) as usize;
        let mut bell_sound = Vec::with_capacity(bell_samples);
        for i in 0..bell_samples {
            let t = i as f32 / sample_rate;
            let osc1 = (pi2 * 2000.0 * t).sin();
            let osc2 = (pi2 * 4000.0 * t).sin() * 0.3;
            let env = (-t * 4.0).exp();
            bell_sound.push((osc1 + osc2) * env * 0.25);
        }

        let mut ambient_sounds = HashMap::new();
        let loop_duration = 8.0;
        let loop_samples = (loop_duration * sample_rate) as usize;

        // Fireplace
        let mut fireplace = Vec::with_capacity(loop_samples);
        let mut prev_brown = 0.0;
        for i in 0..loop_samples {
            let t = i as f32 / sample_rate;
            let white: f32 = rng.gen_range(-1.0..1.0);
            prev_brown = (prev_brown + white) * 0.5;
            let mut crackle = 0.0;
            if rng.gen::<f32>() < 0.0001 {
                crackle = rng.gen_range(0.5..1.0);
            }
            let mod_vol = 0.7 + 0.3 * (pi2 * 0.2 * t).sin();
            fireplace.push((prev_brown + crackle) * mod_vol * 0.15);
        }
        ambient_sounds.insert("Fireplace".to_string(), fireplace);

        // Rain
        let mut rain = Vec::with_capacity(loop_samples);
        let mut history = vec![0.0; 10];
        let mut hist_idx = 0;
        for i in 0..loop_samples {
            let t = i as f32 / sample_rate;
            let white: f32 = rng.gen_range(-1.0..1.0);
            history[hist_idx] = white;
            hist_idx = (hist_idx + 1) % 10;
            let avg: f32 = history.iter().sum::<f32>() / 10.0;
            let pinkish = (white + avg) * 0.5;
            let mod_vol = 0.85 + 0.15 * (pi2 * 0.1 * t).sin();
            rain.push(pinkish * mod_vol * 0.12);
        }
        ambient_sounds.insert("Rain".to_string(), rain);

        // Night
        let mut night = Vec::with_capacity(loop_samples);
        let mut chirp_env = 0.0;
        for i in 0..loop_samples {
            let t = i as f32 / sample_rate;
            let white: f32 = rng.gen_range(-1.0..1.0) * 0.02;
            
            if i % (sample_rate as usize / 2) == 0 && rng.gen::<f32>() < 0.4 {
                chirp_env = 1.0;
            }
            
            let mut chirp = 0.0;
            if chirp_env > 0.0 {
                chirp = (pi2 * 4500.0 * t).sin() * chirp_env * 0.08;
                chirp_env -= 1.0 / (sample_rate * 0.03); // 30ms decay
                if chirp_env < 0.0 { chirp_env = 0.0; }
            }
            
            night.push((white + chirp) * 0.05);
        }
        ambient_sounds.insert("Night".to_string(), night);

        // Cafe
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
        if !self.typing_enabled.get() { return; }
        let source = rodio::buffer::SamplesBuffer::new(1, 44100, samples.to_vec());
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }

    pub fn play_key_click(&self) {
        let idx = self.current_click.get();
        self.play_oneshot(&self.key_sounds[idx]);
        self.current_click.set((idx + 1) % self.key_sounds.len());
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
