use std::{io::Cursor, iter::Iterator, sync::Arc, time::Duration};

use enum_map::EnumMap;
use rodio::{cpal, Decoder, OutputStreamHandle, Source};

use crate::instruments::{BuiltinInstrument, MelodicInstrument, PercussionInstrument};

pub struct BuiltinInstrumentSamples {
    notes: EnumMap<BuiltinInstrument, NoteSamples>,
}

impl BuiltinInstrumentSamples {
    pub fn new() -> Self {
        Self {
            notes: EnumMap::from_fn(|instrument: BuiltinInstrument| {
                NoteSamples::from_source(
                    Decoder::new_vorbis(Cursor::new(instrument.audio_bytes())).unwrap(),
                )
            }),
        }
    }

    fn get_source(&self, instrument: BuiltinInstrument, playback: NotePlayback) -> NoteSource {
        self.notes[instrument].clone().playback(playback)
    }

    pub fn play(
        &self,
        instrument: BuiltinInstrument,
        playback: NotePlayback,
        stream_handle: &OutputStreamHandle,
    ) {
        stream_handle
            .play_raw(self.get_source(instrument, playback))
            .unwrap();
    }
}

impl Default for BuiltinInstrumentSamples {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NoteSamples {
    rate: u32,
    samples: Arc<[i16]>,
}

impl NoteSamples {
    fn from_source(source: impl Source<Item = i16>) -> Self {
        Self {
            rate: source.sample_rate(),
            samples: source.collect(),
        }
    }

    fn playback(self, playback: NotePlayback) -> NoteSource {
        NoteSource {
            samples: self,
            playback,
            pos: 0.0,
            unpanned: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NotePlayback {
    volume: f32,
    speed: f32,
    pan: f32,
}

impl NotePlayback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn volume(self) -> f32 {
        self.volume
    }

    pub fn with_volume(self, volume: f32) -> Self {
        assert!((0.0..=1.0).contains(&volume));
        Self { volume, ..self }
    }

    pub fn speed(self) -> f32 {
        self.speed
    }

    pub fn with_speed(self, speed: f32) -> Self {
        assert!(speed > 0.0);
        Self { speed, ..self }
    }

    pub fn pan(self) -> f32 {
        self.pan
    }

    pub fn with_pan(self, pan: f32) -> Self {
        assert!((-1.0..=1.0).contains(&pan));
        Self { pan, ..self }
    }
}

impl Default for NotePlayback {
    fn default() -> Self {
        Self {
            volume: 1.0,
            speed: 1.0,
            pan: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NoteSource {
    samples: NoteSamples,
    playback: NotePlayback,
    pos: f32,
    unpanned: Option<f32>,
}

impl Iterator for NoteSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(unpanned) = self.unpanned.take() {
            return Some(pan(unpanned, -self.playback.pan));
        };

        let sample_index = self.pos.floor();
        let next_weight = self.pos - sample_index;
        let sample_index = sample_index as usize;

        if sample_index + 1 >= self.samples.samples.len() {
            return None;
        }

        let current_sample = cpal::Sample::to_float_sample(self.samples.samples[sample_index]);
        let next_sample = cpal::Sample::to_float_sample(self.samples.samples[sample_index + 1]);
        let sample = (current_sample * (1.0 - next_weight) + next_sample * next_weight)
            * self.playback.volume;

        self.unpanned = Some(sample);
        self.pos += self.playback.speed;

        Some(pan(sample, self.playback.pan))
    }
}

fn pan(sample: f32, amount: f32) -> f32 {
    sample * (1.0 - amount)
}

impl Source for NoteSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.samples.rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl BuiltinInstrument {
    const fn audio_bytes(self) -> &'static [u8] {
        match self {
            Self::Melodic(instrument) => match instrument {
                MelodicInstrument::Bass => include_bytes!("../assets/instruments/bass.ogg"),
                MelodicInstrument::Bell => include_bytes!("../assets/instruments/bell.ogg"),
                MelodicInstrument::Flute => include_bytes!("../assets/instruments/flute.ogg"),
                MelodicInstrument::Chime => include_bytes!("../assets/instruments/icechime.ogg"),
                MelodicInstrument::Guitar => include_bytes!("../assets/instruments/guitar.ogg"),
                MelodicInstrument::Xylophone => {
                    include_bytes!("../assets/instruments/xylobone.ogg")
                }
                MelodicInstrument::IronXylophone => {
                    include_bytes!("../assets/instruments/iron_xylophone.ogg")
                }
                MelodicInstrument::CowBell => include_bytes!("../assets/instruments/cow_bell.ogg"),
                MelodicInstrument::Didgeridoo => {
                    include_bytes!("../assets/instruments/didgeridoo.ogg")
                }
                MelodicInstrument::Bit => include_bytes!("../assets/instruments/bit.ogg"),
                MelodicInstrument::Banjo => include_bytes!("../assets/instruments/banjo.ogg"),
                MelodicInstrument::Pling => include_bytes!("../assets/instruments/pling.ogg"),
                MelodicInstrument::Harp => include_bytes!("../assets/instruments/harp2.ogg"),
            },
            Self::Percussion(instrument) => match instrument {
                PercussionInstrument::Snare => include_bytes!("../assets/instruments/snare.ogg"),
                PercussionInstrument::Hat => include_bytes!("../assets/instruments/hat.ogg"),
                PercussionInstrument::Bassdrum => include_bytes!("../assets/instruments/bd.ogg"),
            },
        }
    }
}
