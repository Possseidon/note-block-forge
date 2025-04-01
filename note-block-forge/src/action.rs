use egui::{
    include_image, Button, Context, ImageSource, Key, KeyboardShortcut, Modifiers, OpenUrl,
    Response, Ui,
};
use enum_map::Enum;

use crate::app::App;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ActionInfo {
    pub(crate) name: &'static str,
    pub(crate) executor: Option<ActionExecutor>,
    pub(crate) icon: Option<&'static ImageSource<'static>>,
    pub(crate) help: Option<&'static str>,
    pub(crate) shortcut: Option<KeyboardShortcut>,
}

#[derive(Clone, Copy, Debug)]
struct ActionExecutor {
    pub(crate) execute: fn(ActionContextMut),
    pub(crate) can_execute: fn(ActionContext) -> bool,
}

struct ActionContextMut<'a> {
    app: &'a mut App,
    context: &'a Context,
}

#[derive(Clone, Copy)]
struct ActionContext<'a> {
    app: &'a App,
}

impl ActionInfo {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            executor: None,
            icon: None,
            help: None,
            shortcut: None,
        }
    }

    const fn execute(self, execute: fn(ActionContextMut)) -> Self {
        Self {
            executor: Some(ActionExecutor {
                execute,
                can_execute: |_| true,
            }),
            ..self
        }
    }

    const fn execute_if(
        self,
        can_execute: fn(ActionContext) -> bool,
        execute: fn(ActionContextMut),
    ) -> Self {
        Self {
            executor: Some(ActionExecutor {
                execute,
                can_execute,
            }),
            ..self
        }
    }

    const fn icon(self, icon: &'static ImageSource<'static>) -> Self {
        Self {
            icon: Some(icon),
            ..self
        }
    }

    const fn help(self, help: &'static str) -> Self {
        Self {
            help: Some(help),
            ..self
        }
    }

    const fn shortcut(self, modifiers: Modifiers, key: Key) -> Self {
        Self {
            shortcut: Some(KeyboardShortcut::new(modifiers, key)),
            ..self
        }
    }

    const fn on_key(self, key: Key) -> Self {
        self.shortcut(Modifiers::NONE, key)
    }

    const fn on_command(self, key: Key) -> Self {
        self.shortcut(Modifiers::COMMAND, key)
    }

    const fn on_alt(self, key: Key) -> Self {
        self.shortcut(Modifiers::ALT, key)
    }

    const fn on_command_shift(self, key: Key) -> Self {
        self.shortcut(Modifiers::COMMAND.plus(Modifiers::SHIFT), key)
    }
}

#[derive(Clone, Copy, Debug, Enum)]
pub(crate) enum Action {
    About,
    CloseEditor,
    Exit,
    FixAudio,
    MidiDevices,
    New,
    Open,
    OpenIssue,
    Play,
    Save,
    SaveAs,
}

impl Action {
    pub(crate) fn ui(self, ui: &mut Ui, app: &mut App) -> Response {
        let info = self.info();

        let button = Button::opt_image_and_text(
            info.icon.map(|icon| icon.clone().into()),
            Some(info.name.into()),
        );
        if let Some(ActionExecutor {
            execute,
            can_execute,
        }) = info.executor
        {
            let response = ui.add_enabled(can_execute(ActionContext { app }), button);
            if response.clicked() {
                execute(ActionContextMut {
                    app,
                    context: ui.ctx(),
                });
            }
            response
        } else {
            ui.add_enabled(false, button)
        }
    }

    pub(crate) const fn info(self) -> ActionInfo {
        match self {
            Action::About => ActionInfo::new("About")
                .help("Open the About Dialog")
                .icon(&include_image!(
                    "../assets/minecraft/block/redstone_torch.png"
                ))
                .on_key(Key::F1)
                .execute(about),
            Action::CloseEditor => ActionInfo::new("Close Editor")
                .help("Close the current editor tab")
                .icon(&include_image!(
                    "../assets/minecraft/block/oak_trapdoor.png"
                ))
                .on_command(Key::N)
                .execute_if(can_close_editor, close_editor),
            Action::Exit => ActionInfo::new("Exit")
                .help("Exit NoteBlockForge")
                .icon(&include_image!("../assets/minecraft/item/oak_door.png"))
                .on_alt(Key::F4),
            Action::FixAudio => ActionInfo::new("Fix Audio")
                .help("Fix audio playback issues")
                .icon(&include_image!(
                    "../assets/minecraft/item/music_disc_11.png"
                ))
                .on_key(Key::F7),
            Action::MidiDevices => ActionInfo::new("Midi Devices")
                .help("Open the Midi Devices Dialog")
                .icon(&include_image!("../assets/minecraft/item/repeater.png"))
                .on_command(Key::M)
                .execute(midi_devices),
            Action::New => ActionInfo::new("New")
                .help("Create a new NoteBlockFile")
                .icon(&include_image!("../assets/minecraft/item/paper.png"))
                .on_command(Key::N),
            Action::Open => ActionInfo::new("Open...")
                .help("Open an existing NoteBlockFile")
                .icon(&include_image!("../assets/minecraft/item/written_book.png"))
                .on_command(Key::O),
            Action::OpenIssue => ActionInfo::new("Open Issue")
                .help("Open an issue on GitHub")
                .icon(&include_image!(
                    "../assets/minecraft/block/jack_o_lantern.png"
                ))
                .execute(open_issue),
            Action::Play => ActionInfo::new("Play")
                .help("Start playback")
                .icon(&include_image!(
                    "../assets/minecraft/item/music_disc_cat.png"
                ))
                .on_key(Key::Space),
            Action::Save => ActionInfo::new("Save")
                .help("Save the current NoteBlockFile")
                .icon(&include_image!(
                    "../assets/minecraft/item/writable_book.png"
                ))
                .on_command(Key::S),
            Action::SaveAs => ActionInfo::new("Save As...")
                .help("Save the current NoteBlockFile with a new name")
                .icon(&include_image!("../assets/minecraft/item/name_tag.png"))
                .on_command_shift(Key::S),
        }
    }
}

fn about(ctx: ActionContextMut) {
    ctx.app.windows.about_dialog ^= true;
}

fn midi_devices(ctx: ActionContextMut) {
    ctx.app.windows.midi_devices.main_dialog ^= true;
}

fn can_close_editor(ctx: ActionContext) -> bool {
    true
}

fn close_editor(ctx: ActionContextMut) {}

fn open_issue(ctx: ActionContextMut) {
    ctx.context.open_url(OpenUrl {
        url: concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new").into(),
        new_tab: true,
    })
}
