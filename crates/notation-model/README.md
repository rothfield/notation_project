# Notation Model Crate

This crate provides the data structures for representing a particular kind of letter-based, monophonic musical notation, as opposed to staff notation.

## Background

Musical notation can be represented in various ways, and this crate is designed to handle multiple pitch systems. This is crucial for representing music from different cultures and traditions. The supported systems include:

*   **Sargam**: An Indian classical music notation system (Sa, Re, Ga, Ma, Pa, Dha, Ni).
*   **Numbered Notation**: A system that uses numbers (1, 2, 3, 4, 5, 6, 7) to represent the notes of a scale.
*   **Western Notation**: The standard Western notation system (C, D, E, F, G, A, B).

This notation system implements the AACM notation system, which is based on the Bhatkande notation system developed in India. It was used to notate a large corpus of Ali Akbar Khan's compositions in manuscript form from the 1970s onward. These manuscripts are held by his students and in the archives at the AACM.

While the crate supports multiple notation systems, it uses a canonical internal representation for pitches based on a numbered system. The `PitchCode` enum defines the internal representation of each note, which allows for consistent processing and manipulation of musical data, regardless of the original notation system.

### Common Musical Symbols

The AACM system also adopts symbols from standard staff notation for elements like lyrics, slurs, and barlines. These are rendered in a similar fashion to their Western counterparts, providing a familiar visual language for musicians. This will come into play when the notation is rendered.

### Rhythm

### Octaves

In the AACM system, octaves are indicated using dots above or below the note:

*   **Middle Octave**: No dots.
*   **Upper Octave**: A single dot above the note.
*   **Highest Octave**: Two dots above the note.
*   **Low Octave**: A single dot below the note.
*   **Lowest Octave**: Two dots below the note.

This system provides a clear and concise way to represent the pitch of a note, which is essential for accurate rendering of the musical notation.

Rhythm is represented through `Beat`s and their subdivisions.

Rhythm is represented through `Beat`s and their subdivisions. A `Note` or a `Dash` each represent one subdivision of a beat. For example, a beat represented as `S--R` contains four subdivisions in total: the note 'S' is held for three subdivisions, and the note 'R' is held for one.

If we consider a `Beat` to be equivalent to a quarter note, then in the `S--R` example, 'S' would have the duration of a dotted eighth note (3/16) and 'R' would have the duration of a sixteenth note (1/16). This rhythmic representation is a key concept for converting the internal model to other formats, such as LilyPond.

## Core Concepts

The `notation-model` crate is designed to represent musical compositions in a structured and hierarchical way. The main components are:

*   **`Composition`**: The root of the data structure, representing a complete musical piece. It contains metadata like title and author, and a collection of `Line`s.
*   **`Line`**: Represents a single line of music, containing a sequence of `Element`s.
*   **`Element`**: An enum that represents the different components of a musical line, such as `Note`, `Barline`, `Slur`, etc.
*   **`Note`**: Represents a single musical note with properties like pitch, octave, and syllable.
*   **`Beat`**: Represents a beat, which can contain multiple `BeatElement`s (like `Note`s and `Dash`es).
*   **`Cursor`**: A struct that keeps track of the current position within the `Composition`.

## Functionality

The crate provides the following key functionalities:

*   **Composition Management**: Create new compositions, set titles and authors, and add lines of music.
*   **Cursor Navigation**: Move the cursor left, right, up, down, to the beginning of a line (`home`), and to the end of a line (`end`).
*   **Note Manipulation**: Append new notes to the composition at the current cursor position.
*   **Notation Systems**: Supports different notation systems like Western, Number, Sargam, and Lilypond.

## AI-Readable Structure

This README is designed to be easily parsed by AI tools. The structure is as follows:

*   **Crate Name**: `notation-model`
*   **Description**: Data structures for representing musical notation.
*   **Core Components**: `Composition`, `Line`, `Element`, `Note`, `Beat`, `Cursor`
*   **Key Functions**:
    *   `Composition::new()`: Creates a new `Composition`.
    *   `Composition::add_line()`: Adds a `Line` to a `Composition`.
    *   `Composition::cursor_right()`, `Composition::cursor_left()`, etc.: For cursor navigation.
    *   `Composition::append_pitch()`: Appends a `Note` to the `Composition`.
    *   `Note::new()`: Creates a new `Note`.

## Example Usage

```rust
use notation_model::prelude::*;

// Create a new composition
let mut composition = Composition::new();
composition.set_title("My First Song");

// Add a line of music
let mut line = Line::new();
line.elements.push(Element::Note(Note::new(PitchCode::N1)));
composition.add_line(line);
```