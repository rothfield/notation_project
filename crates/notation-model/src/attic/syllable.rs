#[derive(Debug, Clone)]
pub struct Syllable {
    pub id: i32,
    pub text: String,
    pub element_kind: super::element_kind::ElementKind,
}
