use std::{
    collections::BTreeMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
};

use midir::{
    ConnectErrorKind, Ignore, InitError, MidiIO, MidiInput, MidiInputConnection, MidiInputPort,
    MidiInputPorts,
};
use rodio::OutputStreamHandle;

use super::{
    config::{MidiConfig, MidiDeviceConfig, PortId},
    event::MidiEvent,
};
use crate::{
    instruments::{BuiltinInstrument, MelodicInstrument},
    playback::{BuiltinInstrumentSamples, NotePlayback},
};

pub struct MidiConnections {
    midi: MidiInput,
    ports: BTreeMap<String, Vec<DeviceState>>,
    stream_handle: OutputStreamHandle,
    instrument_samples: Arc<BuiltinInstrumentSamples>,
}

impl MidiConnections {
    const CLIENT_NAME: &'static str = "NoteBlockForge";

    pub fn new(
        config: &MidiConfig,
        stream_handle: OutputStreamHandle,
        instrument_samples: Arc<BuiltinInstrumentSamples>,
    ) -> Result<Self, InitError> {
        let mut midi = MidiInput::new(Self::CLIENT_NAME)?;
        midi.ignore(Ignore::All);
        Ok(Self {
            midi,
            ports: config
                .devices
                .iter()
                .map(|config| if config.connect {})
                .collect(),
            stream_handle,
            instrument_samples,
        })
    }

    pub fn config(&self) -> MidiConfig {
        MidiConfig {
            devices: self
                .ports
                .iter()
                .map(|port| MidiDeviceConfig {
                    id: port.id.clone(),
                    connect: port.device.is_connected(),
                })
                .collect(),
        }
    }

    pub fn ports(&self) -> &[MidiPort] {
        &self.ports
    }

    /// Adds newly available ports and marks no longer available
    pub fn refresh_ports(&mut self) {
        for port in self.midi.ports() {
            let name = self.midi.port_name(&port);

            self.ports.iter().find(|port| port.id.name == name)
        }
    }

    fn get_ports(midi: &MidiInput) -> Vec<(PortId, MidiInputPort)> {
        let mut ports = Vec::new();
        let mut counts = BTreeMap::<String, usize>::new();
        for port in midi.ports() {
            let Ok(name) = midi.port_name(&port) else {
                continue;
            };
            let count = counts.entry(name.clone()).or_default();
            ports.push((
                PortId {
                    name,
                    index: *count,
                },
                port,
            ));
            *count += 1;
        }
        ports
    }

    fn midi_callback(
        event_sender: Sender<MidiEvent>,
        midi_playback: Arc<Mutex<MidiPlayback>>,
        instrument_samples: Arc<BuiltinInstrumentSamples>,
        stream_handle: OutputStreamHandle,
    ) -> impl FnMut(u64, &[u8], &mut ()) {
        move |_timestamp, data, ()| {
            if let Some(event) = MidiEvent::parse(data) {
                event_sender.send(event).unwrap();

                match event {
                    MidiEvent::NoteOn { note, velocity } => {
                        let midi_playback = midi_playback.lock().unwrap();
                        let Some(instrument) = midi_playback.instrument else {
                            return;
                        };

                        instrument_samples.play(
                            BuiltinInstrument::Melodic(instrument),
                            NotePlayback::new()
                                .with_volume(velocity.volume() * midi_playback.volume)
                                .with_speed(note.pitch(instrument.base_midi_note()).speed())
                                .with_pan(midi_playback.pan),
                            &stream_handle,
                        );
                    }
                }
            }
        }
    }
}

pub enum DeviceState {
    /// There is no device with this name.
    Unavailable,
    /// There is a device with this name and runtime id.
    Available {
        runtime_id: String,
        connection: Option<MidiConnection>,
    },
    /// There is no longer a device with this runtime id.
    Removed { runtime_id: String },
}

impl DeviceState {
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { connection: .. })
    }

    pub fn channel(&self) -> Option<&Channel> {
        if let Self::Connected { connection } = self {
            Some(&connection.channel)
        } else {
            None
        }
    }
}

pub struct MidiConnection {
    pub channel: Channel,
    #[allow(dead_code)]
    connection: MidiInputConnection<()>,
}

impl std::fmt::Debug for MidiConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MidiConnection")
            .field("channel", &self.channel)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct Channel {
    pub playback: Arc<Mutex<MidiPlayback>>,
    pub events: Receiver<MidiEvent>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MidiPlayback {
    pub instrument: Option<MelodicInstrument>,
    pub volume: f32,
    pub pan: f32,
}

impl Default for MidiPlayback {
    fn default() -> Self {
        Self {
            instrument: None,
            volume: 1.0,
            pan: 0.0,
        }
    }
}
