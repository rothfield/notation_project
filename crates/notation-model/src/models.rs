// src/models.rs

#![allow(dead_code)]

use std::sync::atomic::{AtomicI32, Ordering};
use std::fmt;

// --- ID Generator ---
static NEXT_ID: AtomicI32 = AtomicI32::new(1);
fn next_id() -> i32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}



// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub struct Barline { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Dash { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct FinalBarline { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct LeftBarline { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct LeftRepeat { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct LeftSlur { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct RightBarline { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct RightRepeat { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct RightSlur { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Space { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Trill { id: i32, element_kind: ElementKind }
#[derive(Debug, Clone)]
pub struct Turn { id: i32, element_kind: ElementKind }

#[derive(Debug, Clone)]
pub struct Note {
    id: i32,
    pitch_code: PitchCode,
    pitch_system: NotationKind,
    original_text: String,
    start_pos: usize,
    element_kind: ElementKind,
    octave: Octave,
    syllable: Option<String>, // Added syllable field
}

#[derive(Debug, Clone)]
pub enum BeatElement {
    Note(Note), Dash(Dash), LeftSlur(LeftSlur), RightSlur(RightSlur),
}

#[derive(Debug, Clone)]
pub struct Beat {
    id: i32,
    element_kind: ElementKind,
    elements: Vec<BeatElement>,
    divisions: i32,
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
    element_kind: ElementKind,
    label: String,
    line_number: usize,
    pitch_system: NotationKind,
    elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub line_index: usize,
    pub element_index: usize,
    pub id: i32,
}


#[derive(Debug, Clone)]
pub struct Composition {
    id: i32,
    element_kind: ElementKind,
    title: String,
    author: String,
    header: Option<String>,
    footer: Option<String>,
    pub logical_cursor: Option<Cursor>,
    notation_kind: NotationKind,
    lines: Vec<Line>,
}

// --- Constructors and Getters ---

impl Barline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Barline } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl Dash { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Dash } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl FinalBarline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::FinalBarline } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl LeftBarline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::LeftBarline } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl LeftRepeat { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::LeftRepeat } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl LeftSlur { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::LeftSlur } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl RightBarline { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::RightBarline } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl RightRepeat { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::RightRepeat } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl RightSlur { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::RightSlur } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl Space { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Space } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl Trill { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Trill } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }
impl Turn { pub fn new() -> Self { Self { id: next_id(), element_kind: ElementKind::Turn } } pub fn id(&self) -> i32 { self.id } pub fn element_kind(&self) -> ElementKind { self.element_kind } }

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
            syllable: None, // Default to no syllable
        }
    }
    // Builder method to add a syllable
    pub fn with_syllable(mut self, syllable: &str) -> Self {
        self.syllable = Some(syllable.to_string());
        self
    }
    // Getters
    pub fn id(&self) -> i32 { self.id }
    pub fn pitch_code(&self) -> PitchCode { self.pitch_code }
    pub fn syllable(&self) -> Option<&String> { self.syllable.as_ref() }
    pub fn octave(&self) -> Octave { self.octave }
}

impl Beat {
    pub fn new(elements: Vec<BeatElement>, divisions: i32) -> Self { Self { id: next_id(), element_kind: ElementKind::Beat, elements, divisions } }
    pub fn id(&self) -> i32 { self.id }
    pub fn element_kind(&self) -> ElementKind { self.element_kind }
    pub fn elements(&self) -> &Vec<BeatElement> { &self.elements }
    pub fn divisions(&self) -> i32 { self.divisions }
}

impl Line {
    pub fn new(label: String, line_number: usize, pitch_system: NotationKind, elements: Vec<Element>) -> Self { Self { id: next_id(), element_kind: ElementKind::Line, label, line_number, pitch_system, elements } }
    pub fn id(&self) -> i32 { self.id }
    pub fn element_kind(&self) -> ElementKind { self.element_kind }
    pub fn label(&self) -> &str { &self.label }
    pub fn line_number(&self) -> usize { self.line_number }
    pub fn pitch_system(&self) -> &NotationKind { &self.pitch_system }
    pub fn elements(&self) -> &Vec<Element> { &self.elements }
}

impl Composition {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Composition,
            title: "Untitled".to_string(),
            author: String::new(),
            header: None,
            footer: None,
            logical_cursor: None,
            notation_kind: NotationKind::Number,
            lines: vec![],
        }
    }
    pub fn id(&self) -> i32 { self.id }
    pub fn element_kind(&self) -> ElementKind { self.element_kind }
    pub fn title(&self) -> &str { &self.title }
    pub fn author(&self) -> &str { &self.author }
    pub fn header(&self) -> &Option<String> { &self.header }
    pub fn footer(&self) -> &Option<String> { &self.footer }
    pub fn notation_kind(&self) -> &NotationKind { &self.notation_kind }
    pub fn lines(&self) -> &Vec<Line> { &self.lines }
    pub fn set_title(&mut self, title: &str) { self.title = title.to_string(); }
    pub fn set_author(&mut self, author: &str) { self.author = author.to_string(); }
    pub fn add_line(&mut self, line: Line) { self.lines.push(line); }
}

// ===================================================================
//                        TESTS
// ===================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_happy_birthday_with_lyrics() {
        let mut composition = Composition::new();
        composition.set_title("Happy Birthday");

        // Line 1: "Happy birthday to you" (C C D C F E)
        let line1 = Line::new(
            "Line 1".to_string(),
            1,
            NotationKind::Number,
            vec![
                Element::Note(Note::new(PitchCode::N1).with_syllable("Hap-")),
                Element::Note(Note::new(PitchCode::N1).with_syllable("py")),
                Element::Note(Note::new(PitchCode::N2).with_syllable("birth-")),
                Element::Note(Note::new(PitchCode::N1).with_syllable("day")),
                Element::Note(Note::new(PitchCode::N4).with_syllable("to")),
                Element::Note(Note::new(PitchCode::N3).with_syllable("you")),
            ],
        );

        composition.add_line(line1);

        assert_eq!(composition.title(), "Happy Birthday");
        assert_eq!(composition.lines().len(), 1);
        
        let first_line_elements = composition.lines()[0].elements();
        assert_eq!(first_line_elements.len(), 6);

        // Check the first note to ensure the syllable was set correctly
        if let Element::Note(note) = &first_line_elements[0] {
            assert_eq!(note.syllable(), Some(&"Hap-".to_string()));
        } else {
            panic!("Expected the first element to be a Note");
        }
    }
}



#[test]
fn test_happy_birthday_full2() {
    let mut elements = vec![];

    // Opening barline
    elements.push(Element::LeftBarline(LeftBarline {
        id: next_id(),
        element_kind: ElementKind::LeftBarline,
    }));

    let notes = vec![
        // Phrase 1: Happy birthday to you
        (PitchCode::N5, Octave::Low, "Ha"),
        (PitchCode::N5, Octave::Low, "ppy"),
        (PitchCode::N6, Octave::Low, "birth"),
        (PitchCode::N5, Octave::Low, "day"),
        (PitchCode::N1, Octave::Middle, "to"),
        (PitchCode::N7, Octave::Low, "you"),
        // Barline
        (PitchCode::N5, Octave::Low, "Ha"),
        (PitchCode::N5, Octave::Low, "ppy"),
        (PitchCode::N6, Octave::Low, "birth"),
        (PitchCode::N5, Octave::Low, "day"),
        (PitchCode::N2, Octave::Middle, "to"),
        (PitchCode::N1, Octave::Middle, "you"),
        // Barline
        (PitchCode::N5, Octave::Low, "Ha"),
        (PitchCode::N5, Octave::Low, "ppy"),
        (PitchCode::N5, Octave::Upper, "birth"),
        (PitchCode::N3, Octave::Middle, "day"),
        (PitchCode::N1, Octave::Middle, "dear"),
        (PitchCode::N7, Octave::Low, "friend"),
        // Barline
        (PitchCode::N4, Octave::Middle, "Ha"),
        (PitchCode::N4, Octave::Middle, "ppy"),
        (PitchCode::N3, Octave::Middle, "birth"),
        (PitchCode::N1, Octave::Middle, "day"),
        (PitchCode::N2, Octave::Middle, "to"),
        (PitchCode::N1, Octave::Middle, "you"),
    ];

    for (i, (pitch_code, octave, syllable)) in notes.iter().enumerate() {
        let note = Note {
            id: next_id(),
            pitch_code: *pitch_code,
            pitch_system: NotationKind::Number,
            original_text: format!("{:?}", pitch_code),
            start_pos: i,
            element_kind: ElementKind::Note,
            octave: *octave,
            syllable: Some(syllable.to_string()),
        };
        elements.push(Element::Note(note));

        // Insert barline after every 6 notes
        if (i + 1) % 6 == 0 && i < notes.len() - 1 {
            elements.push(Element::Barline(Barline {
                id: next_id(),
                element_kind: ElementKind::Barline,
            }));
        }
    }

    // Final barline
    elements.push(Element::FinalBarline(FinalBarline {
        id: next_id(),
        element_kind: ElementKind::FinalBarline,
    }));

    let line = Line {
        id: next_id(),
        element_kind: ElementKind::Line,
        label: "verse 1".into(),
        line_number: 1,
        pitch_system: NotationKind::Number,
        elements,
    };

    let composition = Composition {
        id: next_id(),
        element_kind: ElementKind::Composition,
        title: "Happy Birthday (Corrected)".into(),
        author: "Traditional".into(),
        header: Some("🎂 Corrected pitch/octave test".into()),
        footer: Some("End of song".into()),
        notation_kind: NotationKind::Number,
        logical_cursor: None,
        lines: vec![line],
    };

    println!("{:#?}", composition);
}

