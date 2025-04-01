use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MidiConfig {
    pub devices: BTreeMap<String, Vec<MidiDeviceConfig>>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MidiDeviceConfig {
    pub id: PortId,
    /// Whether to connect to the device.
    pub connect: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortId {
    pub name: String,
    pub index: usize,
}
