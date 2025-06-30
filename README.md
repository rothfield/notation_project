# Musical Notation Editor (WIP)

This project aims to create a WebAssembly-powered What You See Is What You Get (WYSIWYG) editor for a unique letter-based, monophonic musical notation system, primarily influenced by the Ali Akbar College of Music (AACM) notation. WebAssembly (WASM) is chosen for its ability to execute high-performance logic directly in the browser, and to allow the core application logic to be written in Rust, minimizing the need for extensive JavaScript development.

## Project Architecture
...
## Future Renderers

The modular design of this project allows for the integration of additional rendering capabilities as separate crates. Planned future renderers include:

*   **LilyPond Renderer**: A dedicated crate to convert the `notation-model` into LilyPond syntax, enabling high-quality, typeset musical scores. This will leverage the detailed rhythmic and pitch information within the model.
*   **Print Output Renderer**: Another potential crate focused on generating print-ready output (e.g., PDF) directly from the `notation-model`, independent of the interactive GUI. This would ensure optimal formatting for physical copies.

## Key Technologies

*   **Rust**: For core logic, data modeling, and rendering engine.
*   **WebAssembly (WASM)**: To compile Rust code for execution in the web browser.
*   **JavaScript**: For the frontend application logic and interaction with the HTML Canvas API.
*   **HTML Canvas**: For rendering the musical notation visually.
*   **`wasm-bindgen`**: For seamless interoperability between Rust and JavaScript.
*   **`rusttype` (or similar font crate)**: For accurate glyph measurement and layout.

## Getting Started

*(Instructions on how to build, run, and develop the project will be added here.)*

## AI-Readable Structure

*   **Project Name**: Musical Notation Editor
*   **Purpose**: WYSIWYG editor for letter-based, monophonic musical notation (AACM system).
*   **Main Components**: `notation-model`, `canvas-gui-renderer`, `wasm-bridge`, JavaScript/HTML Canvas.
*   **Technologies**: Rust, WASM, JavaScript, HTML Canvas, `wasm-bindgen`, `rusttype`.
