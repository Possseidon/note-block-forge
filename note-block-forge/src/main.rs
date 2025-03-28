mod about;
mod action;
mod menu;
mod midi_devices;

use std::{collections::BTreeSet, sync::Arc};

use about::about_dialog;
use eframe::{get_value, set_value, App, CreationContext, Frame, NativeOptions, Storage};
use egui::{include_image, Button, CentralPanel, Context, ImageSource, TopBottomPanel, Visuals};
use enum_map::Enum;
use menu::MAIN_MENU;
use midi_devices::{midi_devices_dialog, MidiDeviceWindows};
use note_block_forge_lib::{
    instruments::{BuiltinInstrument, MelodicInstrument, PercussionInstrument},
    midi::{
        config::{MidiConfig, MidiDeviceConfig},
        connection::MidiConnections,
    },
    playback::BuiltinInstrumentSamples,
};
use rodio::{OutputStream, OutputStreamHandle};
use serde::{Deserialize, Serialize};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        NoteBlockForge::APP_NAME,
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(NoteBlockForge::new(cc)))),
    )
}

struct NoteBlockForge {
    #[allow(dead_code)]
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    windows: OpenWindows,
    instrument_samples: Arc<BuiltinInstrumentSamples>,
    midi: MidiConnections,
    settings: Settings,
}

#[derive(Default)]
struct OpenWindows {
    about_dialog: bool,
    midi_devices: MidiDeviceWindows,
}

#[derive(Default, Serialize, Deserialize)]
struct Settings {
    midi_config: MidiConfig,
}

impl NoteBlockForge {
    const APP_NAME: &'static str = "NoteBlockForge";

    fn new(cc: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let instrument_samples = Arc::new(BuiltinInstrumentSamples::new());
        let settings: Settings = cc
            .storage
            .and_then(|storage| get_value(storage, eframe::APP_KEY))
            .unwrap_or_default();
        let mut midi =
            MidiConnections::new(stream_handle.clone(), Arc::clone(&instrument_samples)).unwrap(); // TODO: handle error
        midi.load_config(&settings.midi_config);
        Self {
            stream,
            stream_handle,
            windows: Default::default(),
            instrument_samples,
            midi,
            settings,
        }
    }
}

impl App for NoteBlockForge {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                theme_switcher(ui);
                MAIN_MENU.ui(ui, self);
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            for (port, _) in self.midi.ports() {
                ui.label(format!("{port:?}"));
            }

            ui.horizontal(|ui| {
                for instrument in 0..MelodicInstrument::LENGTH {
                    let instrument = MelodicInstrument::from_usize(instrument);
                    if ui
                        .add(Button::image(instrument_image_bytes(instrument.into())))
                        .clicked()
                    {
                        // self.midi
                        //     .ports()
                        //     .get("MPK Mini Mk II")
                        //     .unwrap()
                        //     .channel()
                        //     .unwrap()
                        //     .playback
                        //     .lock()
                        //     .unwrap()
                        //     .instrument = Some(instrument);
                    }
                }
            });
        });

        about_dialog(ctx, &mut self.windows.about_dialog);
        midi_devices_dialog(
            ctx,
            &mut self.windows.midi_devices,
            &mut self.settings.midi_config,
            &mut self.midi,
        );
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, eframe::APP_KEY, &self.settings);
    }
}

fn theme_switcher(ui: &mut egui::Ui) {
    if ui.ctx().style().visuals.dark_mode {
        if ui
            .add(
                Button::image(include_image!(
                    "../assets/minecraft/block/white_stained_glass.png"
                ))
                .frame(false),
            )
            .on_hover_text("Switch to light mode")
            .clicked()
        {
            ui.ctx().set_visuals(Visuals::light())
        }
    } else if ui
        .add(
            Button::image(include_image!("../assets/minecraft/block/tinted_glass.png"))
                .frame(false),
        )
        .on_hover_text("Switch to dark mode")
        .clicked()
    {
        ui.ctx().set_visuals(Visuals::dark())
    };
}

const fn instrument_image_bytes(instrument: BuiltinInstrument) -> ImageSource<'static> {
    match instrument {
        BuiltinInstrument::Melodic(instrument) => match instrument {
            MelodicInstrument::Bass => {
                include_image!("../assets/minecraft/block/oak_log.png")
            }
            MelodicInstrument::Bell => {
                include_image!("../assets/minecraft/block/gold_block.png")
            }
            MelodicInstrument::Flute => {
                include_image!("../assets/minecraft/block/clay.png")
            }
            MelodicInstrument::Chime => {
                include_image!("../assets/minecraft/block/packed_ice.png")
            }
            MelodicInstrument::Guitar => {
                include_image!("../assets/minecraft/block/white_wool.png")
            }
            MelodicInstrument::Xylophone => {
                include_image!("../assets/minecraft/block/bone_block_side.png")
            }
            MelodicInstrument::IronXylophone => {
                include_image!("../assets/minecraft/block/iron_block.png")
            }
            MelodicInstrument::CowBell => {
                include_image!("../assets/minecraft/block/soul_sand.png")
            }
            MelodicInstrument::Didgeridoo => {
                include_image!("../assets/minecraft/block/pumpkin_side.png")
            }
            MelodicInstrument::Bit => {
                include_image!("../assets/minecraft/block/emerald_block.png")
            }
            MelodicInstrument::Banjo => {
                include_image!("../assets/minecraft/block/hay_block_side.png")
            }
            MelodicInstrument::Pling => {
                include_image!("../assets/minecraft/block/glowstone.png")
            }
            MelodicInstrument::Harp => include_image!("../assets/minecraft/block/dirt.png"),
        },
        BuiltinInstrument::Percussion(instrument) => match instrument {
            PercussionInstrument::Snare => {
                include_image!("../assets/minecraft/block/sand.png")
            }
            PercussionInstrument::Hat => {
                include_image!("../assets/minecraft/block/glass.png")
            }
            PercussionInstrument::Bassdrum => {
                include_image!("../assets/minecraft/block/stone.png")
            }
        },
    }
}
