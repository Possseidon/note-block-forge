use std::collections::BTreeSet;

use egui::{Context, Grid};
use note_block_forge_lib::midi::{
    config::{MidiConfig, MidiDeviceConfig},
    connection::{MidiConnections, PortConnection},
};

#[derive(Default)]
pub(crate) struct MidiDeviceWindows {
    pub(crate) main_dialog: bool,
    pub(crate) device_dialogs: BTreeSet<String>,
}

pub(crate) fn midi_devices_dialog(
    ctx: &Context,
    windows: &mut MidiDeviceWindows,
    config: &mut MidiConfig,
    connections: &mut MidiConnections,
) {
    egui::Window::new("Midi Devices")
        .open(&mut windows.main_dialog)
        .resizable(false)
        .show(ctx, |ui| {
            let mut reload = false;
            Grid::new("midi-devices").show(ui, |ui| {
                for (port, state) in connections.connections() {
                    match state {
                        PortConnection::Unavailable => {
                            ui.colored_label(ui.visuals().weak_text_color(), &port.name)
                                .on_hover_text("device unavailable");
                        }
                        PortConnection::Available => {
                            ui.colored_label(ui.visuals().text_color(), &port.name)
                                .on_hover_text("device available");
                        }
                        PortConnection::Disconnected => {
                            ui.colored_label(ui.visuals().text_color(), &port.name)
                                .on_hover_text("device disconnected");
                        }
                        PortConnection::ConnectError(error) => {
                            ui.colored_label(ui.visuals().error_fg_color, &port.name)
                                .on_hover_text(error.to_string());
                        }
                        PortConnection::Connected(_) => {
                            ui.colored_label(ui.visuals().strong_text_color(), &port.name)
                                .on_hover_text("device connected");
                        }
                    }

                    // if let Some(device_config) = config.devices.get_mut(&port.index) {
                    //     let mut remove = false;
                    //     if ui.checkbox(&mut device_config.connect, "Connect").clicked() {
                    //         reload = true;
                    //     }
                    //     if ui.button("Remove").clicked() {
                    //         remove = true;
                    //     }
                    //     if ui.button("Open Settings...").clicked() {
                    //         windows.device_dialogs.insert(port.clone());
                    //     }
                    //     if remove {
                    //         config.devices.remove(port);
                    //         reload = true;
                    //     }
                    // } else {
                    //     let mut connect = false;
                    //     if ui.checkbox(&mut connect, "Connect").clicked()
                    //         | ui.button("Add").clicked()
                    //     {
                    //         config
                    //             .devices
                    //             .insert(port.clone(), MidiDeviceConfig { connect });
                    //         reload = true;
                    //     }
                    // }
                    ui.end_row();
                }
            });
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                if reload | ui.button("Reload").clicked() {
                    connections.load_config(config); // TODO: Deal with the error
                }
            });
        });

    windows.device_dialogs.retain(|name| {
        let mut open = true;
        egui::Window::new(name)
            .open(&mut open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Settings");
            });
        open
    });
}
