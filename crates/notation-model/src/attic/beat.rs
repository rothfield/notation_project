// src/models/elements/beat.rs

use super::element_kind::ElementKind;
use super::{
    left_slur::LeftSlur,
    note::Note,
    dash::Dash,
    right_slur::RightSlur,
};

#[derive(Debug, Clone)]
pub struct Beat {
    pub id: i32,
    pub element_kind: ElementKind, // Will always be ElementKind::Beat
    pub elements: Vec<BeatElement>,
    pub divisions: i32,
}

#[derive(Debug, Clone)]
pub enum BeatElement {
    Note(Note),
    Dash(Dash),
    LeftSlur(LeftSlur),
    RightSlur(RightSlur),
}

