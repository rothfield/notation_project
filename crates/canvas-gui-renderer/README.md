# Canvas GUI Renderer Crate

This crate functions as a layout engine (also referred to as an engraver or renderer), responsible for displaying the musical notation model onto a JavaScript canvas for the purpose of a What You See Is What You Get (WYSIWYG) editor.

## Core Concepts

The `canvas-gui-renderer` crate will take the `Composition` data structure from the `notation-model` crate and draw it onto a 2D canvas. This will involve:

*   **Layout Model**: A distinct data structure within this crate that stores the calculated bounding boxes and coordinates for all rendered elements. This separation ensures that the core `notation-model` remains clean and focused on musical data, while this layer handles all presentation-specific layout information necessary for rendering and GUI interactions (e.g., hit-testing for clicks, cursor placement).
*   **Multi-System Rendering**: The layout engine is designed to render musical notation across various systems, including Number, Sargam, and Western. Each system may utilize its own dedicated font to ensure accurate and aesthetically pleasing representation.
*   **Rendering Logic**: Logic to draw the individual elements, such as notes, barlines, slurs, and lyrics.
*   **Styling**: The ability to apply different styles to the rendered notation.

Much of the foundational rendering and input handling logic will be adapted from the existing `notation_engine` project, specifically its `canvas` module and the `render_to_canvas` and input handling functions from its `lib.rs`.

## Functionality

Currently, this crate is a placeholder and only contains a simple `add` function. The intended functionality is to:

*   Take a `Composition` as input.
*   Render the `Composition` to a canvas.
*   Handle user input, such as mouse clicks and keyboard events, to allow for interaction with the notation.

## High-Level Algorithm

The rendering process follows these general steps:

1.  **Input**: Receive a `Composition` object, which contains the structured musical data.
2.  **Layout Calculation**: Determine the precise position and size (bounding boxes) for each musical element within the `Composition`. This involves:
    *   **Glyph Measurement**: Utilizing the `rusttype` (or similar `font`) crate to accurately measure the dimensions of each glyph. This includes understanding its ascent (distance from baseline to top of glyph), descent (distance from baseline to bottom of glyph), and overall bounding box.
    *   **Horizontal Layout**: Arranging elements horizontally, taking into account their individual widths and any necessary spacing to form lines of music.
3.  **Command Generation**: Generate a series of low-level drawing commands (e.g., `ClearRect`, `FillText`, `FillRect`) based on the calculated layout. This also includes commands for rendering the GUI cursor and any highlighted text for selection or editing purposes.
4.  **Rendering**: These commands are then sent to the JavaScript environment to be executed on the HTML Canvas, resulting in the visual representation of the musical notation.
5.  **Interactivity**: The system also handles user interactions (clicks, key presses) by mapping canvas coordinates back to musical elements, often by performing hit-testing against the bounding boxes stored in the **Layout Model**, enabling the WYSIWYG editing experience.

## WASM-JavaScript Interoperability

This crate operates as a WebAssembly (WASM) module, meaning it runs in the browser's JavaScript environment. The communication between the Rust (WASM) core and the JavaScript canvas is crucial for rendering.

### Drawing Command Protocol

The Rust side performs all the layout calculations and then generates a series of abstract drawing commands. These commands are then serialized (e.g., using `serde_wasm_bindgen`) and passed as a JavaScript object to the frontend. The JavaScript side then iterates through these commands and executes the corresponding drawing operations on the HTML Canvas API.

An example of a drawing command structure is as follows:

```rust
pub enum DrawCommand {
    ClearRect { x: f32, y: f32, w: f32, h: f32 },
    FillText { text: String, x: f32, y: f32, font: String, color: String },
    FillRect { x: f32, y: f32, w: f32, h: f32, color: String },
    // ... other drawing commands
}
```

When serialized, a command might look like this in JavaScript:

```json
{
  "ClearRect": {
    "x": 0.0,
    "y": 0.0,
    "w": 800.0,
    "h": 600.0
  }
}
```

Or for text:

```json
{
  "FillText": {
    "text": "S",
    "x": 50.0,
    "y": 100.0,
    "font": "40px Inter",
    "color": "#1c1e21"
  }
}
```

This clear command-based interface ensures a clean separation of concerns, with Rust handling the complex layout logic and JavaScript handling the direct canvas rendering.

## AI-Readable Structure

*   **Crate Name**: `canvas-gui-renderer`
*   **Description**: Renders the musical notation model onto a canvas-based GUI.
*   **Core Components**: Layout Engine, Rendering Logic, Styling.
*   **Key Functions**:
    *   `render(composition: &Composition)`: Renders a `Composition` to the canvas (intended).
