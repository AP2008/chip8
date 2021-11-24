use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::time::{Duration, Instant};

const DELAY_TIME: Duration = Duration::from_millis((1000 / 60) as u64);

pub struct Sound {
    time: Instant,
    count: u8,
    audio: AudioDevice<SquareWave>,
}
impl Sound {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .unwrap();

        Sound {
            time: Instant::now(),
            count: 0,
            audio: device,
        }
    }
    pub fn check(&mut self) {
        if self.count != 0 && self.time.elapsed() >= DELAY_TIME {
            self.count -= 1;
            self.time = Instant::now();
            self.audio.resume();
        } else if self.count == 0 {
            self.audio.pause();
        }
    }
    pub fn set_count(&mut self, count: u8) {
        self.count = count;
        if count > 0 {
            self.audio.resume();
        }
        self.time = Instant::now();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
