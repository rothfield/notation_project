// src/models/element.rs

// It should import the structs it wraps...
use crate::{
    barline::Barline,
    dash::Dash,
    final_barline::FinalBarline,
    left_barline::LeftBarline,
    left_repeat::LeftRepeat,
    left_slur::LeftSlur,
    note::Note,
    right_barline::RightBarline,
    right_repeat::RightRepeat,
    right_slur::RightSlur,
    space::Space,
    trill::Trill,
    beat::Beat,
    turn::Turn,
};

// ...and then define the Element enum.
#[derive(Debug, Clone)]
pub enum Element {
    Note(Note),
    Dash(Dash),
    Space(Space),
    Barline(Barline),
    LeftBarline(LeftBarline),
    RightBarline(RightBarline),
    FinalBarline(FinalBarline),
    LeftRepeat(LeftRepeat),
    RightRepeat(RightRepeat),
    LeftSlur(LeftSlur),
    RightSlur(RightSlur),
    Trill(Trill),
    Turn(Turn),
    Beat(Beat),
}
