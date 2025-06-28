// src/id_generator.rs

use std::sync::atomic::{AtomicI32, Ordering};
use crate::element_kind::ElementKind;
use crate::pitch_code::PitchCode;

// A simple sequential ID generator
static NEXT_ID: AtomicI32 = AtomicI32::new(1);

pub fn next_id() -> i32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

// A "pretty" ID generator that includes the ElementKind and, for notes, the PitchCode
pub fn pretty_id(element_kind: &ElementKind, pitch_code: Option<&PitchCode>) -> String {
    let prefix = element_kind.to_string().to_lowercase();
    let random_component = rand::random::<u32>();
    let sequential_id = next_id();

    if let (ElementKind::Note, Some(pc)) = (element_kind, pitch_code) {
        format!(
            "{}-{:?}-{}#{}",
            prefix,
            pc,
            sequential_id,
            random_component
        )
    } else {
        format!(
            "{}-{}#{}",
            prefix,
            sequential_id,
            random_component
        )
    }
}
