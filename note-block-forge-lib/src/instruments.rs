use std::{fmt, sync::Arc};

use derive_more::From;
use enum_map::Enum;

use crate::midi::event::MidiNote;

pub enum Instrument {
    Builtin(BuiltinInstrument),
    Custom(CustomInstrument),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Enum, From)]
pub enum BuiltinInstrument {
    Melodic(MelodicInstrument),
    Percussion(PercussionInstrument),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Enum)]
pub enum MelodicInstrument {
    Bass,
    Bell,
    Flute,
    Chime,
    Guitar,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
    Harp,
}

impl MelodicInstrument {
    /// Which MIDI note is played by a newly placed note block of this instrument.
    pub fn base_midi_note(self) -> MidiNote {
        MidiNote::new(self.base_octave() * 12 + 18).unwrap()
    }

    /// Which octave is the F# of a newly placed note block in for this instrument.
    ///
    /// E.g. wood/bass has a base octave of 1, and thus has a note range of F#1 to F#3.
    pub fn base_octave(self) -> u8 {
        match self {
            Self::Bass => 1,
            Self::Bell => 5,
            Self::Flute => 4,
            Self::Chime => 5,
            Self::Guitar => 2,
            Self::Xylophone => 5,
            Self::IronXylophone => 3,
            Self::CowBell => 4,
            Self::Didgeridoo => 1,
            Self::Bit => 3,
            Self::Banjo => 3,
            Self::Pling => 3,
            Self::Harp => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Enum)]
pub enum PercussionInstrument {
    Snare,
    Hat,
    Bassdrum,
}

pub struct CustomInstrument(Arc<str>);

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch {
    pub ticks: i8,
}

impl Pitch {
    pub const fn supported_by_minecraft(self) -> bool {
        self.ticks >= 0 && self.ticks <= 24
    }

    pub fn speed(self) -> f32 {
        2.0f32.powf((self.ticks - 12) as f32 / 12.0)
    }

    pub const fn format(self, base_octave: u8, note_offset: NoteOffset) -> PitchFormatter {
        PitchFormatter {
            pitch: self,
            base_octave,
            note_offset,
        }
    }
}

/// One of the two accidentals # and b.
///
/// Not called `Accidental` because that would also include ♮.
#[derive(Clone, Copy, Debug)]
pub enum NoteOffset {
    Sharp,
    Flat,
}

#[derive(Clone, Copy, Debug)]
pub struct PitchFormatter {
    pitch: Pitch,
    base_octave: u8,
    note_offset: NoteOffset,
}

impl fmt::Display for PitchFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.pitch.ticks.rem_euclid(12), self.note_offset) {
            (0, NoteOffset::Sharp) => write!(f, "F#")?,
            (0, NoteOffset::Flat) => write!(f, "Gb")?,
            (1, _) => write!(f, "G")?,
            (2, NoteOffset::Sharp) => write!(f, "G#")?,
            (2, NoteOffset::Flat) => write!(f, "G#")?,
            (3, _) => write!(f, "A")?,
            (4, NoteOffset::Sharp) => write!(f, "A#")?,
            (4, NoteOffset::Flat) => write!(f, "Bb")?,
            (5, _) => write!(f, "B")?,
            (6, _) => write!(f, "C")?,
            (7, NoteOffset::Sharp) => write!(f, "C#")?,
            (7, NoteOffset::Flat) => write!(f, "Db")?,
            (8, _) => write!(f, "D")?,
            (9, NoteOffset::Sharp) => write!(f, "D#")?,
            (9, NoteOffset::Flat) => write!(f, "Eb")?,
            (10, _) => write!(f, "E")?,
            (11, _) => write!(f, "F")?,
            _ => unreachable!(),
        }

        let octave = self.base_octave + ((self.pitch.ticks as i16 + 6) / 12) as u8;
        write!(f, "{}", octave)
    }
}
