#[derive(Clone, Copy, Debug)]
pub(crate) struct Menu {
    pub(crate) entries: &'static [MenuEntry],
}

impl Menu {
    const fn new(entries: &'static [MenuEntry]) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SubMenu {
    pub(crate) name: &'static str,
    pub(crate) entries: &'static [MenuEntry],
}

impl SubMenu {
    const fn new(name: &'static str) -> Self {
        Self { name, entries: &[] }
    }

    const fn entries(self, entries: &'static [MenuEntry]) -> Self {
        Self { entries, ..self }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum MenuEntry {
    SubMenu(SubMenu),
    Action(crate::action::Action),
    Separator,
    Custom, // TODO
}

use egui::{Response, Ui, Widget};

use crate::action::Action;

pub(crate) const MAIN_MENU: Menu = {
    Menu::new(&[
        MenuEntry::SubMenu(SubMenu::new("File").entries(&[
            MenuEntry::Action(Action::New),
            MenuEntry::Separator,
            MenuEntry::Action(Action::Open),
            MenuEntry::Action(Action::Save),
            MenuEntry::Action(Action::SaveAs),
            MenuEntry::Separator,
            MenuEntry::Action(Action::CloseEditor),
            MenuEntry::Action(Action::Exit),
        ])),
        MenuEntry::SubMenu(SubMenu::new("Play").entries(&[MenuEntry::Action(Action::Play)])),
        MenuEntry::SubMenu(
            SubMenu::new("Settings").entries(&[MenuEntry::Action(Action::MidiDevices)]),
        ),
        MenuEntry::SubMenu(SubMenu::new("Help").entries(&[
            MenuEntry::Action(Action::About),
            MenuEntry::Separator,
            MenuEntry::Action(Action::FixAudio),
        ])),
    ])
};

impl Menu {
    pub(crate) fn ui(&self, ui: &mut Ui, state: &mut NoteBlockForge) {
        add_entries(self.entries, ui, state);
    }
}

pub(crate) fn add_entries(entries: &[MenuEntry], ui: &mut Ui, state: &mut NoteBlockForge) {
    for entry in entries {
        match entry {
            MenuEntry::SubMenu(sub_menu) => {
                ui.menu_button(sub_menu.name, |ui| {
                    add_entries(sub_menu.entries, ui, state);
                });
            }
            MenuEntry::Action(action) => {
                if action.ui(ui, state).clicked() {
                    ui.close_menu();
                }
            }
            MenuEntry::Separator => {
                ui.separator();
            }
            MenuEntry::Custom => todo!(),
        }
    }
}
