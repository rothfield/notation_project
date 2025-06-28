// src/constructors.rs

use crate::prelude::*;

impl Barline {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Barline,
        }
    }
}

impl Beat {
    pub fn new(elements: Vec<BeatElement>, divisions: i32) -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Beat,
            elements,
            divisions,
        }
    }
}

impl Composition {
    pub fn new(
        title: String,
        author: String,
        header: Option<String>,
        footer: Option<String>,
        last_cursor_element_id: Option<i32>,
        notation_kind: NotationKind,
        lines: Vec<Line>,
    ) -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Composition,
            title,
            author,
            header,
            footer,
            last_cursor_element_id,
            notation_kind,
            lines,
        }
    }
}

impl Dash {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Dash,
        }
    }
}

impl FinalBarline {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::FinalBarline,
        }
    }
}

impl LeftBarline {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::LeftBarline,
        }
    }
}

impl LeftRepeat {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::LeftRepeat,
        }
    }
}

impl LeftSlur {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::LeftSlur,
        }
    }
}

impl Line {
    pub fn new(
        label: String,
        line_number: usize,
        pitch_system: NotationKind,
        elements: Vec<Element>,
    ) -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Line,
            label,
            line_number,
            pitch_system,
            elements,
        }
    }
}

impl Note {
    pub fn new(
        pitch_code: PitchCode,
        pitch_system: NotationKind,
        original_text: String,
        start_pos: usize,
    ) -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Note,
            pitch_code,
            pitch_system,
            original_text,
            start_pos,
        }
    }
}

impl RightBarline {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::RightBarline,
        }
    }
}

impl RightRepeat {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::RightRepeat,
        }
    }
}

impl RightSlur {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::RightSlur,
        }
    }
}

impl Space {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Space,
        }
    }
}

impl Trill {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Trill,
        }
    }
}

impl Turn {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            element_kind: ElementKind::Turn,
        }
    }
}
