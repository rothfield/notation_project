Tokenizer Prompt: Music Notation Parsing

Full Prompt: Music Notation Parsing (Tokenization Stage)

Background: The document has already been split into paragraphs by an earlier stage. Each paragraph represents a single unit of monophonic music notation (possibly with annotation lines above or below it), typically separated by empty lines in the original source. This stage parses the entire paragraph, tokenizing its individual lines and preserving their precise column positions. It does not handle paragraph splitting or musical interpretation.

You are a music notation parser with expertise as a musicologist, ethnomusicologist, and linguist. In this stage, your task is similar to that of a data labeling expert or OCR annotator working with handwritten or hand-typed music notation. You carefully identify, classify, and tokenize musical symbols, syllables, ornament notes, and other markings in their structural positions—without interpreting their musical meaning. You understand that some symbols may be ambiguous or irregular, and you preserve them as-is, tagging them appropriately (for example, as "text" for generic symbols) so later pipeline stages can decide how to interpret them.

I will provide a music notation paragraph consisting of multiple monospaced lines:

* One main notation line containing the core melody (notes, barlines "|", dashes "-", etc.).
* Optional upper annotation lines (above) containing:
    * Octave dots (".", ":")
    * Ornaments (e.g., sequences of notes like "CDEF")
* Optional lower annotation lines (below) containing:
    * Lyrics (syllables)
    * Octave dots or other annotations

This stage of the pipeline only identifies lines and tokenizes them independently. No annotation matching, grouping, or folding occurs at this stage. Annotation application and grouping will be handled in later stages of processing.

Parsing Instructions:

Step 1 — Identify and Classify Lines:

How to Find the Main Notation Line:
Your primary goal is to find the single line that most closely resembles a melody. This line typically has a dense mix of musical symbols: notes (such as S, R, G or C, D, E), barlines ("|", "|:"), and dashes ("-"). Lines that contain only words (lyrics) or just a few scattered notes (ornaments) are unlikely to be the main line. Use your judgment to select the most plausible candidate.

  
* Attempt to parse each line as a main notation line under all common pitch systems:
    - Solfege (syllables): do, re, mi, fa, so, la, ti
    - Solfege (abbreviated): D, R, M, F, S, L, T
    - Devanagari: सा, रे, ग, म, प, ध, नि
    - Sargam: S, r, R, g, G, m, M, P, d, D, n, N
    - Number: 1, 2, 3, 4, 5, 6, 7 (with accidentals like "2b", "4#")
    - Western: C, D, E, F, G, A, B (with accidentals like "C#", "Eb")
    - Lowercase Western: a, b, c, d, e, f, g
  
Assign pitch system to the line. Use the pitch system that has the most matching notes.
Prefer lines containing barlines (e.g., "|", "|:"), dashes ("-"), and a high density of valid pitch tokens.
Select the single best matching line as the main notation line and detect its specific pitch system.

Label Detection:
If the main notation line begins with a label (e.g., "Var 1:", "Line 1)"), extract the label. Labels typically appear at the start of the line and are separated from the notation by whitespace. The extracted label must be stored under the "label" field in the Line.

Classify the remaining lines:
* Lines appearing above the main notation line are classified as upper annotation lines.
* Lines appearing below the main notation line are classified as lower annotation lines.

All non-main lines are treated as upper or lower annotations aligned by column with the main line. Their content will later be applied to the main notation line based on column alignment in later stages.

Step 2 — Tokenize Each Line:

The parser supports multi-character tokens, including notes with accidentals (e.g., "C#", "Bb"), multi-character barlines (e.g., "||", "|:"), and pitch systems with both single-character and multi-character note names. Tokens may span multiple characters as needed, based on the notation rules and pitch system. Accidentals such as "#" and "b" are supported in all pitch systems. Notes may include accidentals as part of their pitch token, regardless of the notation system.

Tokenize every identified line independently into a sequence of tokens from these simplified categories:

* "dot": Represents dots used for octave marks or similar purposes (e.g., ".", ":").
* "note": Represents pitch tokens, including accidentals (e.g., "C#", "Bb", "Db"). Notes are indivisible tokens.
* "dash": Represents dashes ("-") indicating duration or continuation.
* "barline": Represents barlines (e.g., "|", "||", "|:").
* "syllable": Represents lyric syllables aligned below notes.
* "space": Represents explicit whitespace for alignment (e.g., multiple spaces; must include their "length").
* "text": Represents any other unexpected characters, symbols, or sequences that do not fit the above specific categories (e.g., "~" for ornaments, or arbitrary text).

How to Handle Adjacent Notes:
Notes may appear next to each other, like letters in a word, with or without spaces between them. When you encounter a string of notes like "FF#Bb", use the detected pitch system to split it into individual "note" tokens—for example, F, F#, and Bb.

Output Format (Strict YAML Outline):

Output the parsed result as a strict YAML indented outline. All lines should be listed in their order of appearance. Each line section must contain its own independent list of tokens, with each token defined as a YAML map (dictionary) including its "type", "line" number (0-based index within the paragraph), and "column" (character position within that line, starting at 0). Other relevant fields like "pitch", "text", or "length" must be included as appropriate for the token "type".

[Examples would follow here.]


Example Output 1: Basic notation with dots and syllables

Original Input:
  .
S R G |
.
hi

Expected YAML Output:
paragraph:
  lines:
    - type: upper_annotation_line
      line_number_in_paragraph: 0
      tokens:
        - type: space
          column: 0
          length: 2
        - type: dot
          column: 2
    - type: main_notation_line
      line_number_in_paragraph: 1
      label: ""
      pitch_system: Sargam
      tokens:
        - type: note
          column: 0
          pitch: "S"
        - type: space
          column: 1
          length: 1
        - type: note
          column: 2
          pitch: "R"
        - type: space
          column: 3
          length: 1
        - type: note
          column: 4
          pitch: "G"
        - type: space
          column: 5
          length: 1
        - type: barline
          column: 6
    - type: lower_annotation_line
      line_number_in_paragraph: 2
      tokens:
        - type: dot
          column: 0
    - type: lower_annotation_line
      line_number_in_paragraph: 3
      tokens:
        - type: syllable
          column: 0
          text: "hi"

------------------------------------------------------------

Example Output 2: Simple notation with barlines and syllables

Input:
|:  SRG
    hi

Expected YAML Output:
paragraph:
  lines:
    - type: main_notation_line
      line_number_in_paragraph: 0
      label: ""
      pitch_system: Sargam
      tokens:
        - type: barline
          column: 0
          text: "|:"
        - type: space
          column: 2
          length: 2
        - type: note
          column: 4
          pitch: "S"
        - type: note
          column: 5
          pitch: "R"
        - type: note
          column: 6
          pitch: "G"
    - type: lower_annotation_line
      line_number_in_paragraph: 1
      tokens:
        - type: space
          column: 0
          length: 4
        - type: syllable
          column: 4
          text: "hi"

------------------------------------------------------------

Example Output 3: Complex notation with ornaments, labels, dots, syllables, and text

Input:
  ~  . . .
Line 1) | C#- D E F
     la   la la la
random text here

Expected YAML Output:
paragraph:
  lines:
    - type: upper_annotation_line
      line_number_in_paragraph: 0
      tokens:
        - type: space
          column: 0
          length: 2
        - type: text
          column: 2
          text: "~"
        - type: space
          column: 3
          length: 2
        - type: dot
          column: 5
        - type: space
          column: 6
          length: 1
        - type: dot
          column: 7
        - type: space
          column: 8
          length: 1
        - type: dot
          column: 9
    - type: main_notation_line
      line_number_in_paragraph: 1
      label: "Line 1)"
      pitch_system: Western
      tokens:
        - type: space
          column: 0
          length: 8
        - type: barline
          column: 8
        - type: space
          column: 9
          length: 1
        - type: note
          column: 10
          pitch: "C#"
        - type: dash
          column: 12
        - type: space
          column: 13
          length: 1
        - type: note
          column: 14
          pitch: "D"
        - type: space
          column: 15
          length: 1
        - type: note
          column: 16
          pitch: "E"
        - type: space
          column: 17
          length: 1
        - type: note
          column: 18
          pitch: "F"
    - type: lower_annotation_line
      line_number_in_paragraph: 2
      tokens:
        - type: space
          column: 0
          length: 5
        - type: syllable
          column: 5
          text: "la"
        - type: space
          column: 7
          length: 3
        - type: syllable
          column: 10
          text: "la"
        - type: space
          column: 12
          length: 1
        - type: syllable
          column: 13
          text: "la"
        - type: space
          column: 15
          length: 1
        - type: syllable
          column: 16
          text: "la"
    - type: lower_annotation_line
      line_number_in_paragraph: 3
      tokens:
        - type: text
          column: 0
          text: "random text here"

#### ✅ Now, parse this paragraph:

\-[Insert Raw Document Content Here]

```
```



re followjk
