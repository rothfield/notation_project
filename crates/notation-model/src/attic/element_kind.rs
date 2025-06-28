use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementKind {
    Composition,
    Line,
    Note,
    Trill,
    Turn,
    LeftBarline,
    RightBarline,
    FinalBarline,
    LeftSlur,
    RightSlur,
    RightRepeat,
    LeftRepeat,
    Barline,
    Dash,
    Space,
    Beat, // Added this variant
}

// Implement the Display trait to allow .to_string() calls
impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
