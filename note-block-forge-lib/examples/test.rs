fn main() {
    let midi_input = midir::MidiInput::new("note-block-forge").unwrap();
    for port in midi_input.ports() {
        println!("{}: {:?}", port.id(), midi_input.port_name(&port));
    }
}
