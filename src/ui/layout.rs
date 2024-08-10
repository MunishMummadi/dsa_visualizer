use eframe::egui::{self, CentralPanel};

pub fn create_ui(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Data Structures and Algorithms Visualizer");
        ui.label("Select a data structure and an algorithm to visualize.");
        // TODO: Add dropdowns for selecting data structures and algorithms.
        // TODO: Add a start button to begin visualization.
        // TODO: Add space for displaying performance metrics (e.g., execution time).
    });
}