use crate::instruments::Pitch;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiNote(u8);

impl MidiNote {
    pub fn new(note: u8) -> Option<Self> {
        (note < 0x80).then_some(Self(note))
    }

    pub fn pitch(&self, base_midi_note: MidiNote) -> Pitch {
        Pitch {
            ticks: self.0 as i8 - base_midi_note.0 as i8,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiVelocity(u8);

impl MidiVelocity {
    pub fn volume(&self) -> f32 {
        self.0 as f32 / 0x7f as f32
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MidiEvent {
    NoteOn {
        note: MidiNote,
        velocity: MidiVelocity,
    },
}

impl MidiEvent {
    pub(crate) fn parse(data: &[u8]) -> Option<Self> {
        // let channel = data[0] & 0x0F;
        match (data[0] >> 4, &data[1..]) {
            (0x8, [_note, _velocity]) => None,
            // Some(Self::NoteOff {
            //     note: MidiNote::new(*note)?,
            //     velocity: MidiVelocity(*velocity),
            // }),
            (0x9, [note, velocity]) => Some(Self::NoteOn {
                note: MidiNote::new(*note)?,
                velocity: MidiVelocity(*velocity),
            }),
            (0xA, [_, _]) => None, // Polyphonic Key Pressure (Aftertouch),
            (0xB, [_, _]) => None, // Control Change
            (0xC, [_]) => None,    // Program Change
            (0xD, [_]) => None,    // Channel Pressure (Aftertouch)
            (0xE, [_, _]) => None, // Pitch Bend
            (0xF, _) => None,      // System
            _ => {
                println!("unhandled midi event: {:?}", data);
                None
            }
        }
    }
}
