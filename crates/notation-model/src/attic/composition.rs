use crate::element_kind::ElementKind;
use crate::notation_kind::NotationKind;
use crate::line::Line;

#[derive(Debug, Clone)]
pub struct Composition {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub last_cursor_element_id: Option<i32>,
    pub notation_kind: NotationKind,
    pub element_kind: ElementKind,
    pub lines: Vec<Line>,
}
