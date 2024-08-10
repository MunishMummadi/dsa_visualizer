use eframe::egui;
use rand::Rng;
use std::time::Instant;
#[derive(PartialEq, Clone)]
enum DataStructure {
    Array,
    LinkedList,
    BinaryTree,
    Graph,
}

#[derive(PartialEq, Clone)]
enum Algorithm {
    BubbleSort,
    QuickSort,
    BinarySearch,
    DepthFirstSearch,
}

struct DsaVisualizer {
    current_ds: DataStructure,
    current_algo: Option<Algorithm>,
    array: Vec<i32>,
    animation_state: Vec<Vec<i32>>,
    animation_index: usize,
    last_update: Instant,
}

impl Default for DsaVisualizer {
    fn default() -> Self {
        Self {
            current_ds: DataStructure::Array,
            current_algo: None,
            array: (1..=10).collect(),
            animation_state: Vec::new(),
            animation_index: 0,
            last_update: Instant::now(),
        }
    }
}

impl DsaVisualizer {
    fn randomize_array(&mut self) {
        let mut rng = rand::thread_rng();
        self.array = (0..10).map(|_| rng.gen_range(1..=100)).collect();
        self.animation_state.clear();
        self.animation_index = 0;
    }

    fn bubble_sort(&mut self) {
        self.animation_state.clear();
        let mut array = self.array.clone();
        let n = array.len();
        
        for i in 0..n {
            for j in 0..n - i - 1 {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                    self.animation_state.push(array.clone());
                }
            }
        }
    }
}

impl eframe::App for DsaVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DSA Visualizer");
            
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_ds, DataStructure::Array, "Array");
                ui.selectable_value(&mut self.current_ds, DataStructure::LinkedList, "Linked List");
                ui.selectable_value(&mut self.current_ds, DataStructure::BinaryTree, "Binary Tree");
                ui.selectable_value(&mut self.current_ds, DataStructure::Graph, "Graph");
            });
            
            ui.add_space(10.0);
            
            match self.current_ds {
                DataStructure::Array => {
                    if ui.button("Randomize Array").clicked() {
                        self.randomize_array();
                    }
                    
                    if ui.button("Bubble Sort").clicked() {
                        self.current_algo = Some(Algorithm::BubbleSort);
                        self.bubble_sort();
                    }
                    
                    ui.add_space(20.0);
                    
                    // Display array
                    ui.horizontal(|ui| {
                        for &value in &self.array {
                            ui.label(egui::RichText::new(value.to_string()).size(20.0));
                            ui.add_space(10.0);
                        }
                    });
                    
                    // Animate if an algorithm is selected
                    if let Some(Algorithm::BubbleSort) = self.current_algo {
                        if !self.animation_state.is_empty() && self.last_update.elapsed().as_millis() > 500 {
                            self.array = self.animation_state[self.animation_index].clone();
                            self.animation_index = (self.animation_index + 1) % self.animation_state.len();
                            self.last_update = Instant::now();
                            ctx.request_repaint();
                        }
                    }
                },
                _ => {
                    ui.label("Not implemented yet");
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "DSA Visualizer",
        native_options,
        Box::new(|_cc| Ok(Box::new(DsaVisualizer::default())))
    )
}