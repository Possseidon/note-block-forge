use egui::{Context, Grid, RichText};

pub(crate) fn about_dialog(ctx: &Context, show: &mut bool) {
    egui::Window::new("About")
        .open(show)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.hyperlink_to(
                    RichText::new(" NoteBlockForge").heading(),
                    env!("CARGO_PKG_REPOSITORY"),
                );
                ui.label(env!("CARGO_PKG_DESCRIPTION"));
                ui.separator();
            });

            Grid::new("about")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Version:");
                    ui.text_edit_singleline(&mut env!("CARGO_PKG_VERSION"));
                    ui.end_row();
                    ui.label("Author:");
                    ui.hyperlink_to(" Possseidon", "https://github.com/Possseidon");
                    ui.end_row();
                    ui.label("Contact:");
                    ui.hyperlink_to(
                        "📧 xpossseidon@gmail.com",
                        "mailto:xpossseidon@gmail.com?subject=NoteBlockForge",
                    );
                    ui.end_row();
                });
        });
}
