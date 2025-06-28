#[derive(Debug, Clone)]
pub struct LowerOctave {
    pub id: i32,
    pub value: i8,
    pub element_kind: super::element_kind::ElementKind,
}
