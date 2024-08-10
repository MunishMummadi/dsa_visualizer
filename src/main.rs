mod ui;
mod visualizations;
mod algorithms;
mod data_structures;
mod utils;

use eframe::egui;
use ui::layout::create_ui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DSA Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(VisualizerApp::default()))),
    );
}

// TODO: Add fields to the VisualizerApp struct to manage the state of selected data structure and algorithm.

struct VisualizerApp;

impl Default for VisualizerApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for VisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        create_ui(ctx);
        // TODO: Call the visualization functions based on the selected data structure and algorithm.
    }
}