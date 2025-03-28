use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MidiConfig {
    pub devices: Vec<MidiDeviceConfig>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortId {
    pub name: String,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MidiDeviceConfig {
    pub id: PortId,
    /// Whether to connect to the device.
    pub connect: bool,
}
