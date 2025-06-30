// src/modelss

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

impl Element {
    pub fn get_id(&self) -> i32 {
        match self {
            Element::Note(n) => n.id,
            Element::Dash(d) => d.id,
            Element::Space(s) => s.id,
            Element::Barline(b) => b.id,
            Element::LeftBarline(b) => b.id,
            Element::RightBarline(b) => b.id,
            Element::FinalBarline(b) => b.id,
            Element::LeftRepeat(r) => r.id,
            Element::RightRepeat(r) => r.id,
            Element::LeftSlur(s) => s.id,
            Element::RightSlur(s) => s.id,
            Element::Trill(t) => t.id,
            Element::Turn(t) => t.id,
            Element::Beat(b) => b.id,
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
    pub cursor: Option<Cursor>,
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
            start_pos: 0,
            element_kind: ElementKind::Note,
            pitch_code,
            pitch_system: NotationKind::Number,
            original_text: String::new(),
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
    pub fn new() -> Self 
    { Self { id: next_id(), element_kind: ElementKind::Beat, 
            elements: Vec::new(),
               divisions: 0,
           }
    }
    pub fn id(&self) -> i32 { self.id }
    pub fn element_kind(&self) -> ElementKind { self.element_kind }
    pub fn elements(&self) -> &Vec<BeatElement> { &self.elements }
    pub fn divisions(&self) -> i32 { self.divisions }
}

impl Line {

  pub fn new() -> Self {
        Line {
            id: next_id(),
            element_kind: ElementKind::Line,
            label: String::new(),
            line_number: 0,
            pitch_system: NotationKind::Number,
            elements: Vec::new(),
        }
    }

    pub fn id(&self) -> i32 { self.id }
    pub fn element_kind(&self) -> ElementKind { self.element_kind }
    pub fn label(&self) -> &str { &self.label }
    pub fn line_number(&self) -> usize { self.line_number }
    pub fn pitch_system(&self) -> &NotationKind { &self.pitch_system }
    pub fn elements(&self) -> &Vec<Element> { &self.elements }
}

impl Composition {


    pub fn cursor_right(&mut self) -> bool {
        if let Some(cursor) = &mut self.cursor {
            if let Some(line) = self.lines.get(cursor.line_index) {
                if cursor.element_index + 1 < line.elements.len() {
                    cursor.element_index += 1;
                    cursor.id = line.elements[cursor.element_index].get_id();
                    return true;
                } else if cursor.line_index + 1 < self.lines.len() {
                    cursor.line_index += 1;
                    cursor.element_index = 0;
                    if let Some(next_line) = self.lines.get(cursor.line_index) {
                        cursor.id = next_line.elements.get(0).map_or(-1, |e| e.get_id());
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn cursor_left(&mut self) -> bool {
        if let Some(cursor) = &mut self.cursor {
            if cursor.element_index > 0 {
                cursor.element_index -= 1;
                if let Some(line) = self.lines.get(cursor.line_index) {
                    cursor.id = line.elements[cursor.element_index].get_id();
                    return true;
                }
            } else if cursor.line_index > 0 {
                cursor.line_index -= 1;
                if let Some(prev_line) = self.lines.get(cursor.line_index) {
                    cursor.element_index = prev_line.elements.len().saturating_sub(1);
                    cursor.id = prev_line.elements.get(cursor.element_index).map_or(-1, |e| e.get_id());
                    return true;
                }
            }
        }
        false
    }

    pub fn cursor_up(&mut self) -> bool {
        if let Some(cursor) = &mut self.cursor {
            if cursor.line_index > 0 {
                cursor.line_index -= 1;
                let new_line = &self.lines[cursor.line_index];
                cursor.element_index = cursor.element_index.min(new_line.elements.len().saturating_sub(1));
                cursor.id = new_line.elements.get(cursor.element_index).map_or(-1, |e| e.get_id());
                return true;
            }
        }
        false
    }

    pub fn cursor_down(&mut self) -> bool {
        if let Some(cursor) = &mut self.cursor {
            if cursor.line_index + 1 < self.lines.len() {
                cursor.line_index += 1;
                let new_line = &self.lines[cursor.line_index];
                cursor.element_index = cursor.element_index.min(new_line.elements.len().saturating_sub(1));
                cursor.id = new_line.elements.get(cursor.element_index).map_or(-1, |e| e.get_id());
                return true;
            }
        }
        false
    }

    pub fn cursor_home(&mut self) -> bool {
        if let Some(cursor) = &mut self.cursor {
            if let Some(line) = self.lines.get(cursor.line_index) {
                if !line.elements.is_empty() {
                    cursor.element_index = 0;
                    cursor.id = line.elements[0].get_id();
                    return true;
                }
            }
        }
        false
    }

    pub fn cursor_end(&mut self) -> bool {
        if let Some(cursor) = &mut self.cursor {
            if let Some(line) = self.lines.get(cursor.line_index) {
                if !line.elements.is_empty() {
                    cursor.element_index = line.elements.len() - 1;
                    cursor.id = line.elements[cursor.element_index].get_id();
                    return true;
                }
            }
        }
        false
    }

    pub fn cursor_next_line(&mut self) -> bool {
        if let Some(cursor) = &mut self.cursor {
            if cursor.line_index + 1 < self.lines.len() {
                cursor.line_index += 1;
                cursor.element_index = 0;
                if let Some(line) = self.lines.get(cursor.line_index) {
                    cursor.id = line.elements.get(0).map_or(-1, |e| e.get_id());
                    return true;
                }
            }
        }
        false
    }

    pub fn get_cursor(&self) -> &Cursor {
        self.cursor.as_ref().expect("cursor must exist")
    }

    pub fn get_cursor_mut(&mut self) -> &mut Cursor {
        self.cursor.as_mut().expect("cursor must exist")
    }



    pub fn append_pitch(&mut self, pitch_code: PitchCode) {
        // Step 1: Get cursor position (fallback to end of last line if None)
        let cursor = self.get_cursor();
        let line_index = cursor.line_index;
        let element_index = cursor.element_index;
        // Step 2: Construct Note with defaults
        let mut note = Note::new(pitch_code);
        note.start_pos = element_index;
        let new_id = note.id;

        // Step 3: Insert Note into line
        let line = &mut self.lines[line_index];
        line.elements.insert(element_index, Element::Note(note));

        // Step 4: Advance the cursor
        self.cursor = Some(Cursor {
            line_index,
            element_index: element_index + 1,
            id: new_id,
        });
    }

    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Composition,
            title: "Untitled".to_string(),
            author: String::new(),
            header: None,
            footer: None,
            cursor: None,
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
