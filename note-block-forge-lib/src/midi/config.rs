use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MidiConfig {
    pub devices: Vec<MidiDeviceConfig>,
    pub settings: BTreeMap<String, ()>,
}

/// Configuration for a single midi input device such as a midi keyboard.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MidiDeviceConfig {
    /// The last known [`midir::MidiInputPort::id`].
    pub id: String,
    /// The last known [`midir::MidiInput::port_name`].
    pub name: String,
    /// Whether this device was previously connected and should attempt to reconnect.
    pub connect: bool,
    pub settings: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortId {
    pub name: String,
    pub index: usize,
}
