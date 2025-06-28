// src/lib.rs
pub mod layout;
mod models;
pub use crate::models::composition_to_string::composition_to_string as inner_composition_to_string;
mod input;
mod canvas;
use wasm_bindgen::prelude::*;
pub use models::{TextElement, Line, FontFamily};
use rusttype::Font;
use crate::layout::assign_bounding_boxes::assign_bounding_boxes;
#[wasm_bindgen]
#[derive(Clone)]
pub struct Composition {
    title: String,
    author: String,
    pub(crate) lines: Vec<Line>,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) selected_element_id: Option<u32>,
    #[wasm_bindgen(skip)]
    pub letter_font: Font<'static>,
    #[wasm_bindgen(skip)]
    pub symbol_font: Font<'static>,
    #[wasm_bindgen(skip)]
    pub font_size: f32,
}

#[wasm_bindgen]
impl Composition {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: f32,
        height: f32,
        letter_font_data: Vec<u8>,
        symbol_font_data: Vec<u8>,
    ) -> Self {
        let letter_font = Font::try_from_vec(letter_font_data).expect("Error constructing letter font");
        let symbol_font = Font::try_from_vec(symbol_font_data).expect("Error constructing symbol font");

        Self {
            title: "Untitled Score".to_string(),
            author: "Unknown Composer".to_string(),
            lines: vec![Line { name: "Default".to_string(), elements: vec![] }],
            width,
            height,
            selected_element_id: None,
            letter_font,
            symbol_font,
            font_size: 40.0,
        }
    }

    // --- GETTERS ---
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String { self.title.clone() }

    #[wasm_bindgen(getter)]
    pub fn author(&self) -> String { self.author.clone() }

    #[wasm_bindgen(js_name = getLines)]
    pub fn get_lines(&self) -> JsValue { serde_wasm_bindgen::to_value(&self.lines).unwrap() }






    // --- RENDERERS ---
    #[wasm_bindgen(js_name = renderToCanvas)]
    pub fn render_to_canvas(&mut self) -> JsValue {
        let mut commands = Vec::new();
        // let notation = self.notation.clone(); // avoid borrowing self later
        let cloned_self = self.clone();
        let get_metrics = move |text: &str, font: &FontFamily| {
            crate::layout::metrics::get_element_metrics(&cloned_self, text, font)
        };

        for line in self.lines.iter_mut() {
            assign_bounding_boxes(line, &get_metrics);
        }

        // 1. Clear background
        canvas::draw::background(&mut commands, self);
        // 2. Draw the selection highlight BEHIND the text
        canvas::draw::selection(&mut commands, self);
        // 3. Draw content (boxes and text) on top of the selection
        canvas::draw::elements(&mut commands, self);

        serde_wasm_bindgen::to_value(&commands).unwrap()
    }





    // --- INPUT HANDLERS ---
    #[wasm_bindgen]
    pub fn on_canvas_click(&mut self, x: f32, y: f32) {
        input::handle_canvas_click(self, x, y);
    }

    #[wasm_bindgen]
    pub fn on_key_press(&mut self, key: String) {
        input::handle_key_press(self, key);
    }

    #[wasm_bindgen]
    pub fn get_selected_id(&self) -> i32 {
        self.selected_element_id.map(|id| id as i32).unwrap_or(-1)
    }
}
#[wasm_bindgen(js_name = compositionToString)]
pub fn composition_to_string(comp: &Composition) -> String {
    inner_composition_to_string(comp)
}
