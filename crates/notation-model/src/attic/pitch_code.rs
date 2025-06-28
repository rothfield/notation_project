#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitchCode {
    // Diatonic scale degrees (N for "Number" or "Note")
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,

    N1Sharp,
    N2Flat,
    N2Sharp,
    N3Flat,
    N4Sharp,
    N5Flat,
    N6Flat,
    N6Sharp,
    N7Flat,
}
