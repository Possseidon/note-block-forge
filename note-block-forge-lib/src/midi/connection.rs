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
    config::{MidiConfig, PortId},
    event::MidiEvent,
};
use crate::{
    instruments::{BuiltinInstrument, MelodicInstrument},
    playback::{BuiltinInstrumentSamples, NotePlayback},
};

pub struct MidiConnections {
    midi: MidiInput,
    /// A list of all available ports.
    ports: Vec<(PortId, MidiInputPort)>,
    /// A mirror of the config with actual connection information.
    connections: Vec<(PortId, PortConnection)>,
    stream_handle: OutputStreamHandle,
    instrument_samples: Arc<BuiltinInstrumentSamples>,
}

impl MidiConnections {
    const CLIENT_NAME: &'static str = "NoteBlockForge";
    const PORT_NAME: &'static str = "NoteBlockForge";

    pub fn new(
        stream_handle: OutputStreamHandle,
        instrument_samples: Arc<BuiltinInstrumentSamples>,
    ) -> Result<Self, InitError> {
        let mut midi = MidiInput::new(Self::CLIENT_NAME)?;
        midi.ignore(Ignore::All);
        let ports = Self::get_ports(&midi);
        Ok(Self {
            midi,
            ports,
            connections: Default::default(),
            stream_handle,
            instrument_samples,
        })
    }

    pub fn ports(&self) -> &[(PortId, MidiInputPort)] {
        &self.ports
    }

    pub fn connections(&self) -> &[(PortId, PortConnection)] {
        &self.connections
    }

    pub fn refresh_ports(&mut self) {
        self.ports = Self::get_ports(&self.midi);
    }

    pub fn load_config(&mut self, config: &MidiConfig) {
        for (port_id, port) in &mut self.connections {
            let device_config = config.devices.iter().find(|config| config.id == *port_id);
            if let Some(config) = device_config {
                if config.connect {
                } else {
                    *port = PortConnection::Disconnected;
                }
            } else {
                *port = PortConnection::Available;
            }
        }

        for config in &config.devices {
            if !self
                .connections
                .iter()
                .any(|(port_id, _)| port_id == &config.id)
            {
                self.connections
                    .push((config.id.clone(), PortConnection::Unavailable));
            }
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

pub enum PortConnection {
    /// The port exists but is not part of the config.
    Available,
    /// The port is part of the config but doesn't exist anymore.
    Unavailable,
    /// The port exists and is part of the config, but is not connected.
    Disconnected,
    /// The port exists and is part of the config, but connecting to it fails.
    ConnectError(ConnectErrorKind),
    /// The port exists, is part of the config and is connected.
    Connected(MidiConnection),
}

impl PortConnection {
    pub fn channel(&self) -> Option<&Channel> {
        if let Self::Connected(connection) = self {
            Some(&connection.channel)
        } else {
            None
        }
    }
}

/// A port is listed in the config and is connected.
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
