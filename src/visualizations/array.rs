use eframe::egui::Ui;
use crate::data_structures::array::Array;  // Import Array from the ds folder

pub struct ArrayVisualizer {
    array: Array,
}

impl ArrayVisualizer {
    pub fn new(array: Array) -> Self {
        Self { array }
    }

    pub fn visualize(&self, ui: &mut Ui) {
        // Display the array elements visually (e.g., as boxes or bars).
        for &value in &self.array.data {
            ui.label(format!("{}", value));
        }
    }
    
    // TODO: Add methods for step-by-step visualization (e.g., highlighting current elements being compared).
}