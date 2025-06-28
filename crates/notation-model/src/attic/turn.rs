use crate::element_kind::ElementKind;

#[derive(Debug, Clone)]
pub struct Turn {
    pub id: i32,
    pub element_kind: ElementKind,
}