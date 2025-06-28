use crate::element_kind::ElementKind;
use crate::notation_kind::NotationKind;
use crate::element::Element;

#[derive(Debug, Clone)]
pub struct Line {
    pub id: i32,
    pub label: String,
    pub line_number: usize,
    pub pitch_system: NotationKind,
    pub element_kind: ElementKind,
    pub elements: Vec<Element>,

}
