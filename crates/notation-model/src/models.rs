// src/models.rs

use std::sync::atomic::{AtomicI32, Ordering};
use std::fmt;

// --- ID Generator ---
static NEXT_ID: AtomicI32 = AtomicI32::new(1);
fn next_id() -> i32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

// --- Enums ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementKind {
    Composition, Line, Note, Trill, Turn, LeftBarline, RightBarline,
    FinalBarline, LeftSlur, RightSlur, RightRepeat, LeftRepeat,
    Barline, Dash, Space, Beat,
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum NotationKind {
    Western, Number, Sargam, Lilypond,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitchCode {
    N1, N2, N3, N4, N5, N6, N7,
    N1Sharp, N2Flat, N2Sharp, N3Flat,
    N4Sharp, N5Flat, N6Flat, N6Sharp, N7Flat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Octave {
    Lowest, Low, Middle, Upper, Highest,
}

impl Octave {
    pub fn as_i8(&self) -> i8 {
        match self {
            Octave::Lowest => -2,
            Octave::Low => -1,
            Octave::Middle => 0,
            Octave::Upper => 1,
            Octave::Highest => 2,
        }
    }
}

// --- Model Structs ---

#[derive(Debug, Clone)]
pub struct Barline { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Dash { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct FinalBarline { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct LeftBarline { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct LeftRepeat { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct LeftSlur { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Lyrics { pub syllables: Vec<String> }
#[derive(Debug, Clone)]
pub struct RightBarline { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct RightRepeat { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct RightSlur { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Space { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Trill { id: i32, pub element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Turn { id: i32, pub element_kind: ElementKind }

#[derive(Debug, Clone)]
pub struct Note {
    id: i32,
    pub pitch_code: PitchCode,
    pub pitch_system: NotationKind,
    pub original_text: String,
    pub start_pos: usize,
    pub element_kind: ElementKind,
    pub octave: Octave,
}

#[derive(Debug, Clone)]
pub enum BeatElement {
    Note(Note), Dash(Dash), LeftSlur(LeftSlur), RightSlur(RightSlur),
}

#[derive(Debug, Clone)]
pub struct Beat {
    id: i32,
    pub element_kind: ElementKind,
    pub elements: Vec<BeatElement>,
    pub divisions: i32,
}

#[derive(Debug, Clone)]
pub enum Element {
    Note(Note), Dash(Dash), Space(Space), Barline(Barline),
    LeftBarline(LeftBarline), RightBarline(RightBarline),
    FinalBarline(FinalBarline), LeftRepeat(LeftRepeat),
    RightRepeat(RightRepeat), LeftSlur(LeftSlur),
    RightSlur(RightSlur), Trill(Trill), Turn(Turn), Beat(Beat),
}

#[derive(Debug, Clone)]
pub struct Line {
    id: i32,
    pub label: String,
    pub line_number: usize,
    pub pitch_system: NotationKind,
    pub element_kind: ElementKind,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct Composition {
    id: i32,
    pub title: String,
    pub author: String,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub last_cursor_element_id: Option<i32>,
    pub notation_kind: NotationKind,
    pub element_kind: ElementKind,
    pub lines: Vec<Line>,
}

// --- Constructors and ID Getters ---

impl Barline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Barline } } pub fn id(&self) -> i32 { self.id } }
impl Dash { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Dash } } pub fn id(&self) -> i32 { self.id } }
impl FinalBarline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::FinalBarline } } pub fn id(&self) -> i32 { self.id } }
impl LeftBarline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::LeftBarline } } pub fn id(&self) -> i32 { self.id } }
impl LeftRepeat { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::LeftRepeat } } pub fn id(&self) -> i32 { self.id } }
impl LeftSlur { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::LeftSlur } } pub fn id(&self) -> i32 { self.id } }
impl RightBarline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::RightBarline } } pub fn id(&self) -> i32 { self.id } }
impl RightRepeat { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::RightRepeat } } pub fn id(&self) -> i32 { self.id } }
impl RightSlur { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::RightSlur } } pub fn id(&self) -> i32 { self.id } }
impl Space { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Space } } pub fn id(&self) -> i32 { self.id } }
impl Trill { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Trill } } pub fn id(&self) -> i32 { self.id } }
impl Turn { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Turn } } pub fn id(&self) -> i32 { self.id } }

impl Note {
    pub fn new(pitch_code: PitchCode) -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Note,
            pitch_code,
            pitch_system: NotationKind::Number,
            original_text: String::new(),
            start_pos: 0,
            octave: Octave::Middle,
        }
    }
    pub fn id(&self) -> i32 { self.id }
}
impl Beat {
    pub fn new(elements: Vec<BeatElement>, divisions: i32) -> Self { Self { id: next_id(), element_kind: ElementKind::Beat, elements, divisions } }
    pub fn id(&self) -> i32 { self.id }
}
impl Line {
    pub fn new(label: String, line_number: usize, pitch_system: NotationKind, elements: Vec<Element>) -> Self { Self { id: next_id(), element_kind: ElementKind::Line, label, line_number, pitch_system, elements } }
    pub fn id(&self) -> i32 { self.id }
}
impl Composition {
    pub fn new(title: String, author: String, header: Option<String>, footer: Option<String>, last_cursor_element_id: Option<i32>, notation_kind: NotationKind, lines: Vec<Line>) -> Self { Self { id: next_id(), element_kind: ElementKind::Composition, title, author, header, footer, last_cursor_element_id, notation_kind, lines } }
    pub fn id(&self) -> i32 { self.id }
}

// ===================================================================
//
//                        TESTS
//
// ===================================================================

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the current file (models.rs)

    #[test]
    fn test_simplified_note_constructor_and_defaults() {
        // The constructor now only requires a PitchCode
        let note = Note::new(PitchCode::N5);

        // Use the id() getter method, as the field is private
        assert!(note.id() > 0);
        assert_eq!(note.pitch_code, PitchCode::N5);
        assert_eq!(note.element_kind, ElementKind::Note);

        // Verify the new defaults
        assert_eq!(note.octave, Octave::Middle);
        assert_eq!(note.octave.as_i8(), 0);
        assert!(matches!(note.pitch_system, NotationKind::Number));
        assert_eq!(note.original_text, "");
        assert_eq!(note.start_pos, 0);
    }

    #[test]
    fn test_octave_enum_values() {
        assert_eq!(Octave::Lowest.as_i8(), -2);
        assert_eq!(Octave::Low.as_i8(), -1);
        assert_eq!(Octave::Middle.as_i8(), 0);
        assert_eq!(Octave::Upper.as_i8(), 1);
        assert_eq!(Octave::Highest.as_i8(), 2);
    }

    #[test]
    fn test_id_immutability_and_access() {
        let barline = Barline::new();
        let barline_id = barline.id();

        // This line would cause a compile error, which is what we want:
        // barline.id = 123;
        // error[E0616]: field `id` of struct `Barline` is private

        // We can only read the ID through the getter
        assert!(barline_id > 0);
    }

    #[test]
    fn test_sequential_ids_across_different_types() {
        // Create a sequence of different elements
        let note = Note::new(PitchCode::N1);
        let dash = Dash::new();
        let space = Space::new();
        let trill = Trill::new();

        // Check that the IDs are sequential and incrementing
        assert_eq!(dash.id(), note.id() + 1);
        assert_eq!(space.id(), dash.id() + 1);
        assert_eq!(trill.id(), space.id() + 1);
    }

    #[test]
    fn test_composition_with_elements() {
        let line = Line::new(
            "Verse 1".to_string(),
            1,
            NotationKind::Number,
            vec![
                Element::Note(Note::new(PitchCode::N1)),
                Element::Note(Note::new(PitchCode::N2)),
                Element::Note(Note::new(PitchCode::N3)),
            ],
        );

        let composition = Composition::new(
            "Song of Tests".to_string(),
            "A. Developer".to_string(),
            None,
            None,
            Some(line.id()),
            NotationKind::Number,
            vec![line],
        );

        assert!(composition.id() > 0);
        assert_eq!(composition.lines.len(), 1);
        assert_eq!(composition.last_cursor_element_id, Some(composition.lines[0].id()));
    }
}
