use crate::element_kind::ElementKind;
use crate::notation_kind::NotationKind;
use crate::pitch_code::PitchCode;

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i32,
    pub pitch_code: PitchCode,
    pub pitch_system: NotationKind,
    pub original_text: String,
    pub start_pos: usize,
    pub element_kind: ElementKind,
}
