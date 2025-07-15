You are given a text document representing musical notation.

- The text is organized in vertical columns. Notes may have annotations **above or below**.
- Dashes (-) are rhythmic placeholders.
- Dots (.) above or below notes indicate octave shifts:
  - A dot directly **above** a note = Octave 1.
  - A dot directly **below** a note = Octave -1.
  - If no dot is present = Octave 0.
  - The dot only applies to the note in the **same column**.
- Syllables (lyrics) may also appear **below** the notes. They follow the same column alignment rule:
  - A syllable applies only to the note directly **above it** (±1 column tolerance).
- Beats are separated by whitespace.
- Notes and dashes form beats.
- Bar lines are marked using |, :|, and |: symbols.
- Line labels (e.g., 1)) may precede a line of music.
- Beats are sequences of adjacent notes and dashes. For example:
    - C---D is a single beat with 5 divisions.
    - S- is a beat with 2 divisions.
    - -- is a beat with 2 divisions.
- Notes are represented by the letters C D E F G A B, optionally with accidentals (e.g., C#, Bb).
- Notes are not separated by spaces. So `C#CC` is three notes: C#, C, C.
- A Note may be one or two characters long if it includes an accidental.
- Each accidental (b or #) applies only to the note immediately before it.
- Each Note token must represent exactly one pitch with its own Text and Octave.

---

Sample input

     Title

       .
1)  |: C-D EF :|

---

Note: The dot above the first C indicates Octave 1. All others are at Octave 0.

---

Sample output (YAML)

Document:
  Title: Title
  Lines:
    - Line:
        Label: "1)"
        Tokens:
          - LeftRepeatBarline
          - Space
          - Beat:
              Divisions: 3
              Tokens:
                - Note:
                    Pitch: "C"
                    Text: "C"
                    Octave: 1
                - Dash:
                    Text: "-"
                - Note:
                    Pitch: "D"
                    Text: "D"
                    Octave: 0
          - Space
          - Beat:
              Divisions: 2
              Tokens:
                - Note:
                    Pitch: "E"
                    Text: "E"
                    Octave: 0
                - Note:
                    Pitch: "F"
                    Text: "F"
                    Octave: 0
          - Space
          - RightRepeatBarline

---

✅ Second sample input

    Line with dot and lyric

2)  |  C-D E |
       .     .
     la      ti

---

Note: The dot below C means Octave -1.  
The word "la" belongs to C, "ti" belongs to E.

---

Sample output (YAML)

Document:
  Title: Line with dot and lyric
  Lines:
    - Line:
        Label: "2)"
        Tokens:
          - Barline
          - Space
          - Beat:
              Divisions: 3
              Tokens:
                - Note:
                    Pitch: "C"
                    Text: "C"
                    Octave: -1
                    Syllable: "la"
                - Dash:
                    Text: "-"
                - Note:
                    Pitch: "D"
                    Text: "D"
                    Octave: 0
          - Space
          - Beat:
              Divisions: 1
              Tokens:
                - Note:
                    Pitch: "E"
                    Text: "E"
                    Octave: -1
                    Syllable: "ti"
          - Space
          - Barline

---

✅ Now, parse this:

[[INPUT]]

