use three_d::egui::{*, self};
use open;

pub fn show_help_gui(gui_context: &egui::Context) {
    Window::new("Help").anchor(Align2::CENTER_TOP, [0.0, 0.0]).show(&gui_context, |ui| {

        ui.heading("Camera Movement:");
        ui.label("To move around, hold down the left mouse button over the vieweport and use WASD like a first person game. The Q and E keys can be used to move in the third axis.");
        ui.label("Press 5 to toggle between perspective and orthographic cameras.");
        ui.label("Use 1, 3, 5, 7 and CTRL+1, CTRL+3, CTRL+5, CTRL+7 to align your camera with an axis (like Blender).");
        ui.separator();
        ui.heading("Selecting");
        ui.label("Click on any node to select it. Hold shift while clicking to select additional nodes.");
        ui.label("Use the deselect button or press CTRL+D to deselect all nodes.");
        ui.label("Selected nodes will be red while unselected nodes will be brown.");
        ui.label("Select two nodes to select the beam between them.");
        ui.label("Select three nodes to select the triangle between them.");
        ui.separator();
        ui.heading("Moving Nodes");
        ui.label("To move the selected node(s) press G followed by X, Y or Z depending on what axis you desire.");
        ui.label("Click to confirm the movement, right click to cancel.");
        ui.separator();
        ui.heading("Changing Properties");
        ui.label("With either a node, beam or triangle selected, press TAB to toggle between the 3D viewport and property editor.");

        ui.separator();

        ui.heading("Additional Help");
        ui.label("For additional help or to report bugs please feel free to join my discord server");
        if ui.hyperlink("https://discord.gg/cqq2hPfbEC").clicked() {
            open::that("https://discord.gg/cqq2hPfbEC").unwrap_or(());
        }

    });
}