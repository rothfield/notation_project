# WASM Bridge Crate

This crate acts as the primary bridge between the Rust (WebAssembly) modules and the JavaScript environment in the web application. Its core responsibility is to facilitate seamless communication and data exchange between the compiled Rust logic and the browser's runtime.

## Core Concepts

*   **Interoperability Layer**: Provides the necessary mechanisms for Rust functions to be called from JavaScript, and potentially for Rust to invoke JavaScript functions or browser APIs.
*   **Data Serialization/Deserialization**: Handles the conversion of complex data structures (like the `Composition` from `notation-model` or `DrawCommand`s from `canvas-gui-renderer`) between Rust's memory representation and JavaScript's object model. This often involves libraries like `serde` and `serde_wasm_bindgen`.
*   **WebAssembly Bindings**: Utilizes `wasm-bindgen` to generate the necessary JavaScript and TypeScript bindings that allow the Rust code to be imported and used as a standard JavaScript module.

## Functionality

Currently, this crate is a placeholder with a simple `add` function. Its intended functionalities include:

*   **Exposing Public API**: Defining and exposing the main entry points and functions from the Rust application that the JavaScript frontend needs to interact with (e.g., initialization, rendering triggers, event handlers).
*   **Event Handling**: Passing user input events (like mouse clicks, keyboard presses) from JavaScript to the appropriate Rust modules for processing.
*   **Command Dispatch**: Receiving drawing commands or other instructions from Rust modules and dispatching them to the JavaScript side for execution on the canvas or other DOM elements.

## AI-Readable Structure

*   **Crate Name**: `wasm-bridge`
*   **Description**: Facilitates communication and data exchange between Rust (WASM) and JavaScript.
*   **Core Components**: Interoperability Layer, Data Serialization/Deserialization, WebAssembly Bindings.
*   **Key Functions**:
    *   `init()`: (Intended) Initializes the WASM module and sets up the environment.
    *   `process_event(event: JsValue)`: (Intended) Receives and processes events from JavaScript.
    *   `render_commands()`: (Intended) Returns drawing commands to JavaScript for rendering.
