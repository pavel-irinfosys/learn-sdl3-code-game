use sdl3::Sdl;
use sdl3::audio::{AudioFormat, AudioSpec, AudioStreamOwner};
use std::f32::consts::TAU;

const RATE: i32 = 44100;

pub struct Sound {
    stream: AudioStreamOwner,
}

impl Sound {
    pub fn new(sdl: &Sdl) -> Result<Self, Box<dyn std::error::Error>> {
        let audio = sdl.audio()?;
        let spec = AudioSpec {
            freq: Some(RATE),
            channels: Some(1),
            format: Some(AudioFormat::f32_sys()),
        };
        let device = audio.open_playback_device(&spec)?;
        let stream = device.open_device_stream(Some(&spec))?;
        stream.resume()?;
        Ok(Self { stream })
    }

    /// Plays notes one after another: (frequency Hz, seconds)
    fn play(&mut self, notes: &[(f32, f32)]) {
        let mut samples = Vec::new();
        for &(freq, secs) in notes {
            let count = (RATE as f32 * secs) as usize;
            for i in 0..count {
                let t = i as f32 / RATE as f32;
                // fade in/out so there are no clicks
                let env = (i as f32 / 200.0).min(1.0) * ((count - i) as f32 / 200.0).min(1.0);
                samples.push((t * freq * TAU).sin() * 0.25 * env);
            }
        }
        let _ = self.stream.put_data_f32(&samples);
    }

    // pub fn click(&mut self) {
    //     self.play(&[(900.0, 0.03)]);
    // }
    pub fn step(&mut self) {
        self.play(&[(440.0, 0.08)]);
    }
    pub fn blocked(&mut self) {
        self.play(&[(160.0, 0.25)]);
    }
    pub fn win(&mut self) {
        self.play(&[(523.0, 0.1), (659.0, 0.1), (784.0, 0.1), (1047.0, 0.25)]);
    }
}
