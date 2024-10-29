# DSA Visualizer

A Data Structures and Algorithms Visualizer built with Rust and egui.

## Project Overview

This project aims to create an interactive and educational tool for visualizing various data structures and algorithms. Users will be able to select different data structures, apply algorithms, and see the operations animated in real-time.

### Features

- **Interactive UI**: Allows users to select different data structures and algorithms for visualization.
- **Step-by-Step Visualization**: Displays each step of the algorithm's execution visually.
- **Customizable Input**: Users can input their own data to see how algorithms perform.
- **Performance Metrics**: Provides insights into the time and space complexity of algorithms.
- **Educational Explanations**: Offers detailed explanations of the algorithms and data structures.

## Current State

- **UI**: Basic setup with egui.
- **Data Structures**:
  - Array: Basic implementation.
- **Algorithms**:
  - Bubble Sort: Basic visualization implemented.
- **Visualization**:
  - Array Visualization: Basic rendering of array elements.

<details>
  <summary>TODO List</summary>

### Data Structures
- [ ] Implement Linked List data structure.
- [ ] Implement Binary Tree data structure.
- [ ] Implement Graph data structure.

### Algorithms
- [ ] Implement Quick Sort algorithm.
- [ ] Implement Merge Sort algorithm.
- [ ] Implement Binary Search.
- [ ] Implement Depth-First Search for graphs.
- [ ] Implement Breadth-First Search for graphs.
- [ ] Implement Tree Traversals (In-order, Pre-order, Post-order).
- [ ] Implement Graph algorithms (Dijkstra's, A*).

### Visualizations
- [ ] Enhance Array Visualization:
  - [ ] Add step-by-step controls (pause, resume, reset).
  - [ ] Improve animation system for clearer visualization.
- [ ] Implement Linked List Visualization.
- [ ] Implement Binary Tree Visualization.
- [ ] Implement Graph Visualization.
- [ ] Implement Animations Module:
  - [ ] Create AnimationManager for controlling visualizations.
  - [ ] Add methods for starting, stopping, and resetting animations.

### UI/UX
- [ ] Improve UI design and user experience.
- [ ] Add user controls for custom input data.

### Utilities
- [ ] Implement a performance metrics module.
- [ ] Display time complexity and space complexity for each algorithm.

### Documentation
- [ ] Add educational explanations for each algorithm.
- [ ] Improve user documentation.
- [ ] Add inline code comments for clarity.

### Testing
- [ ] Add unit tests for data structures.
- [ ] Add integration tests for algorithms.
- [ ] Add UI tests for visualization components.
</details>

## Getting Started

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```sh
   git clone https://github.com/MunishMummadi/dsa_visualizer.git
   ```
3. Navigate to the project directory:
   ```sh
   cd dsa_visualizer
   ```
4. Build the project:
   ```sh
   cargo build
   ```
5. Run the application:
   ```sh
   cargo run
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
