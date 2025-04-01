use egui::{include_image, Button, Visuals};

pub fn theme_switcher(ui: &mut egui::Ui) {
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
