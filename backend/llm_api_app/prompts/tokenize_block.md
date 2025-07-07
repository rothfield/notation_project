### **Project Overview (High-Level):**

This project processes hand-typed or monospaced digital music notation in multiple pipeline stages:

* **Paragraph Extraction:** This stage typically involves the user entering notation manually (via a text editor or interactive input) or ingesting scanned notation using OCR or transcription tools. It splits the input into **paragraphs**, where each paragraph represents a single line of monophonic music notation, possibly with annotations above and below.
* **Line Tokenization (This Stage):** Parses a single paragraph, parsing and tokenizing the lines of the paragraph, preserving their exact positions for later stages.
* **Later Stages:** Apply annotations, group tokens, calculate durations, and generate output formats such as LilyPond or MusicXML.

This document describes **Line Tokenization: Tokenizing a paragraph of music notation**.

## 📝 Full Prompt: Music Notation Parsing (Tokenization Stage)

**Background:** The document has already been split into **paragraphs** by an earlier stage. Each paragraph represents a **single line of monophonic music notation** (possibly with annotation lines above or below it). This stage parses the aforementioned paragraph—tokenizing its lines and preserving their column positions. It does not handle paragraph splitting or musical interpretation.

You are a music notation parser with expertise as a musicologist, ethnomusicologist, and linguist. In this stage, your task is similar to that of a data labeling expert or OCR annotator working with handwritten or hand-typed music notation. You carefully identify, classify, and tokenize musical symbols, syllables, ornament notes, and other markings in their structural positions—without interpreting their musical meaning. You understand that some symbols may be ambiguous or irregular, and you preserve them as-is, tagging them appropriately (for example, as generic symbols) so later pipeline stages can decide how to interpret them.

I will provide a **music notation paragraph** consisting of multiple monospaced lines:

* One **main notation line** containing the core melody (notes, barlines `|`, dashes `-`, etc.).
* Optional **upper annotation lines** (above) containing:

  * Octave dots (`.` or `:`).
  * Ornaments (e.g., sequences of notes like `CDEF`).
* Optional **lower annotation lines** (below) containing:

  * Lyrics (syllables).
  * Octave dots or other annotations.

This stage of the pipeline only identifies lines and tokenizes them independently. No annotation matching, grouping, or folding occurs at this stage. Annotation application and grouping will be handled in later stages of processing.

---

### **Parsing Instructions:**

#### **Step 1 — Identify and Classify Lines:**

**How to Find the Main Notation Line:**
Your primary goal is to find the single line that most closely resembles a melody. This line typically has a dense mix of musical symbols: notes (such as S, R, G or C, D, E), barlines (`|`), and dashes (`-`). Lines that contain only words (lyrics) or just a few scattered notes (ornaments) are unlikely to be the main line. Use your judgment to select the most plausible candidate.

* Attempt to parse each line as a **main notation line** under all pitch systems:

  * **Solfege (syllables):** do, re, mi, fa, so, la, ti
  * **Solfege (syllables):** do, re, mi, fa, so, la, ti
  * **Solfege (abbreviated):** D, R, M, F, S, L, T
  * **Devanagari:** सा, रे, ग, म, प, ध, नि
  * **Sargam:** S, r, R, g, G, m, M, P, d, D, n, N
  * **Number:** 1, 2, 3, 4, 5, 6, 7 (with accidentals like `2b`, `4#`)
  * **Western:** C, D, E, F, G, A, B (with accidentals like `C#`, `Eb`)
  * **Lowercase Western:** a, b, c, d, e, f, g
* Prefer lines containing **barlines (**\`\`**)** and valid pitch tokens.
* Select the best matching line as the **main notation line** and detect its pitch system.

**Label Detection:**
If the main notation line begins with a label (e.g., `Var 1:`, `Line 1)`), extract the label. Labels typically appear at the start of the line and are separated from the notation by whitespace. The label must be stored under the `Label` field in the output.

Classify the remaining lines:

* Lines above the main notation line are classified as **upper annotation lines**.
* Lines below the main notation line are classified as **lower annotation lines**.

All non-main lines are treated as upper or lower annotations aligned by column with the main line. Their content will later be applied to the main notation line based on column alignment in later stages.

---

#### **Step 2 — Tokenize Each Line:**

The parser supports multi-character tokens, including notes with accidentals (e.g., `C#`, `Bb`), multi-character barlines (e.g., `||`, `|:`), and pitch systems with both single-character and multi-character note names. Tokens may span multiple characters as needed, based on the notation rules and pitch system. Accidentals such as `#` and `b` are supported in all pitch systems. Notes may include accidentals as part of their pitch token, regardless of the notation system.

Tokenization depends on the type of line:

* **Upper Annotation Lines:** Parse dots (`.`, `:`), ornament notes, whitespace, and any unexpected symbols. Sequences of notes in upper lines are often ornaments and are tokenized as `note` tokens here.
* **Main Notation Line:** Parse using the simplified token categories.
* **Lower Annotation Lines:** Parse syllables, dots, whitespace, and any unexpected symbols.

All tokenization follows these simplified categories:

* `dot` — dots used for octave marks or similar purposes.
* `note` — pitch tokens, including accidentals (e.g., `C#`, `Bb`). Notes are indivisible.
* `dash` — dashes (`-`) indicating duration.
* `syllable` — lyric syllables aligned below notes.
* `space` — explicit spaces for alignment (must include their `length`).
* `text` — other unexpected characters.

This stage does not perform any annotation matching, grouping, or musical interpretation. In upper annotation lines, sequences of notes are typically ornament symbols. These are tokenized as `note` tokens in this stage, and their ornament function is identified later.

* **Main Notation Line:** Parse using the full set of notation tokens, including `note`, `barline`, `dash`, `whitespace`, and others.
* **Lower Annotation Lines:** Parse syllables (lyrics), octave dots, whitespace, and any unexpected symbols as `symbol` tokens.
* **Main Notation Line:** Parse using the full set of notation tokens, including `note`, `barline`, `dash`, `whitespace`, and others.
* **Lower Annotation Lines:** Parse syllables (lyrics), octave dots, whitespace, and any unexpected symbols as `symbol` tokens.

Tokenize every line independently into a sequence of tokens from these simplified categories:

* `dot` — dots used for octave marks or similar purposes (e.g., `.` or `:`).
* `note` — pitch tokens, including accidentals (e.g., `C#`, `Bb`). Notes are indivisible tokens.
* `dash` — dashes (`-`) indicating duration or continuation.
* `syllable` — lyric syllables aligned below notes.
* `space` — explicit whitespace for alignment (spaces; must be tokenized with their `length`).

Other tokens can be captured as `text` when necessary.

**How to Handle Adjacent Notes:**
Notes may appear next to each other, like letters in a word, with or without spaces between them. When you encounter a string of notes like `FF#Bb`, use the detected pitch system to split it into individual notes—for example, F, F#, and Bb.

---

#### **Output Parse Tree (Outline Format):**

Output the parsed result as an indented outline tree, listing all lines in their order of appearance. Each line should show its classification and contain its own independent list of tokens with their column positions.

```plaintext
Paragraph
    Upper Line
        Tokens
            [token]
    Upper Line
        Tokens
            [token]
    Main Line
        Pitch System: [pitch system]
        Label: [label or blank]
        Tokens
            [token]
    Lower Line
        Tokens
            [token]
    Lower Line
        Tokens
            [token]
```

* All tokens must include `column` (0-based index).
* No annotation matching or grouping is performed at this stage.

---

#### Example Outputs:

The following examples show parsed outputs for paragraphs of music notation, progressing from simple to more complex cases:

---

#### Example Output 1: Basic notation with dots and syllables

````plaintext
Output:
Original Input:
  .
S R G |
.
hi

Paragraph:
Paragraph
    Upper Line
        Tokens
            space
                line: 0
                column: 0
                length: 2
            dot
                line: 0
                column: 2
    Main Line
        Pitch System: Sargam
        Label: 
        Tokens
            note
                line: 1
                column: 0
                pitch: S
            space
                line: 1
                column: 1
                length: 1
            note
                line: 1
                column: 2
                pitch: R
            space
                line: 1
                column: 3
                length: 1
            note
                line: 1
                column: 4
                pitch: G
            space
                line: 1
                column: 5
                length: 1
            barline
                line: 1
                column: 6
    Lower Line
        Tokens
            dot
                line: 2
                column: 0
    Lower Line
        Tokens
            syllable
                line: 3
                column: 0
                text: "hi"
```plaintext
  .
S R G |
.
hi
````

**Output:**

```plaintext
Paragraph
    Upper Line
        Tokens
            space
                line: 0
                column: 0
                length: 2
            dot
                line: 0
                column: 2
    Main Line
        Pitch System: Sargam
        Label: 
        Tokens
            note
                line: 1
                column: 0
                pitch: S
            space
                line: 1
                column: 1
                length: 1
            note
                line: 1
                column: 2
                pitch: R
            space
                line: 1
                column: 3
                length: 1
            note
                line: 1
                column: 4
                pitch: G
            space
                line: 1
                column: 5
                length: 1
            barline
                line: 1
                column: 6
    Lower Line
        Tokens
            dot
                line: 2
                column: 0
    Lower Line
        Tokens
            syllable
                line: 3
                column: 0
                text: "hi"
```

---

#### Example 2: Simple notation with barlines and syllables

**Input:**

```plaintext
|:  SRG
    hi
```

**Output:**

```plaintext
Paragraph
    Main Line
        Pitch System: Sargam
        Label: 
        Tokens
            barline
                line: 0
                column: 0
            space
                line: 0
                column: 2
                length: 2
            note
                line: 0
                column: 4
                pitch: S
            note
                line: 0
                column: 5
                pitch: R
            note
                line: 0
                column: 6
                pitch: G
    Lower Line
        Tokens
            space
                line: 1
                column: 0
                length: 4
            syllable
                line: 1
                column: 4
                text: "hi"
```

---

#### Example 3: Complex notation with ornaments, labels, dots, syllables, and text

**Input:**

```plaintext
  ~  . . .
Line 1) | C#- D E F
     la   la la la
random text here
```

**Output:**

```plaintext
Paragraph
    Upper Line
        Tokens
            text
                line: 0
                column: 2
                text: "~"
            space
                line: 0
                column: 3
                length: 2
            dot
                line: 0
                column: 5
            space
                line: 0
                column: 6
                length: 1
            dot
                line: 0
                column: 7
            space
                line: 0
                column: 8
                length: 1
            dot
                line: 0
                column: 9
    Main Line
        Pitch System: Western
        Label: Line 1)
        Tokens
            barline
                line: 1
                column: 8
            space
                line: 1
                column: 9
                length: 1
            note
                line: 1
                column: 10
                pitch: C#
            dash
                line: 1
                column: 12
            space
                line: 1
                column: 13
                length: 1
            note
                line: 1
                column: 14
                pitch: D
            space
                line: 1
                column: 15
                length: 1
            note
                line: 1
                column: 16
                pitch: E
            space
                line: 1
                column: 17
                length: 1
            note
                line: 1
                column: 18
                pitch: F
    Lower Line
        Tokens
            syllable
                line: 2
                column: 5
                text: "la"
            space
                line: 2
                column: 7
                length: 3
            syllable
                line: 2
                column: 10
                text: "la"
            space
                line: 2
                column: 12
                length: 1
            syllable
                line: 2
                column: 13
                text: "la"
            space
                line: 2
                column: 15
                length: 1
            syllable
                line: 2
                column: 16
                text: "la"
    Lower Line
        Tokens
            text
                line: 3
                column: 0
                text: "random text here"
```

#### **Example Output:**

````plaintext
Paragraph
    Upper Line
        Tokens
            space
                line: 0
                column: 0
                length: 2
            dot
                line: 0
                column: 2
    Main Line
        Pitch System: Sargam
        Label: 
        Tokens
            note
                line: 1
                column: 0
                pitch: S
            space
                line: 1
                column: 1
                length: 1
            note
                line: 1
                column: 2
                pitch: R
            space
                line: 1
                column: 3
                length: 1
            note
                line: 1
                column: 4
                pitch: G
            space
                line: 1
                column: 5
                length: 1
            barline
                line: 1
                column: 6
    Lower Line
        Tokens
            dot
                line: 2
                column: 0
    Lower Line
        Tokens
            syllable
                line: 3
                column: 0
                text: "hi"
```plaintext
Paragraph
    Main Line
        Pitch System: Sargam
        Label: 
        Tokens
            barline
                line: 0
                column: 0
            space
                line: 0
                column: 2
                length: 2
            note
                line: 0
                column: 4
                pitch: S
            note
                line: 0
                column: 5
                pitch: R
            note
                line: 0
                column: 6
                pitch: G
    Lower Line
        Tokens
            space
                line: 1
                column: 0
                length: 4
            syllable
                line: 1
                column: 4
                text: "hi"
````

#### **Example Output:**

```plaintext
Paragraph
    Upper Line
        Tokens
            symbol
                line: 0
                column: 2
                text: "~"
            whitespace
                line: 0
                column: 3
                length: 1
            text
                line: 0
                column: 4
                text: "."
            whitespace
                line: 0
                column: 5
                length: 1
            text
                line: 0
                column: 6
                text: "."
            whitespace
                line: 0
                column: 7
                length: 1
            text
                line: 0
                column: 8
                text: "."
    Main Line
        Pitch System: Western
        Label: Line 1)
        Tokens
            barline
                line: 1
                column: 8
            whitespace
                line: 1
                column: 9
                length: 1
            note
                line: 1
                column: 10
                pitch: C#
            dash
                line: 1
                column: 12
            whitespace
                line: 1
                column: 13
                length: 1
            note
                line: 1
                column: 14
                pitch: D
            whitespace
                line: 1
                column: 15
                length: 1
            note
                line: 1
                column: 16
                pitch: E
            whitespace
                line: 1
                column: 17
                length: 1
            note
                line: 1
                column: 18
                pitch: F
    Lower Line
        Tokens
            syllable
                line: 2
                column: 5
                text: "la"
            whitespace
                line: 2
                column: 7
                length: 3
            syllable
                line: 2
                column: 10
                text: "la"
            whitespace
                line: 2
                column: 12
                length: 1
            syllable
                line: 2
                column: 13
                text: "la"
            whitespace
                line: 2
                column: 15
                length: 1
            syllable
                line: 2
                column: 16
                text: "la"
    Lower Line
        Tokens
            text
                line: 3
                column: 0
                text: "random text here"
```

#### **Example Output:**

```plaintext
Paragraph
    Upper Line
        Tokens
            note
                line: 0
                column: 4
                pitch: C
            note
                line: 0
                column: 5
                pitch: D
            note
                line: 0
                column: 6
                pitch: E
            note
                line: 0
                column: 7
                pitch: F
    Main Line
        Pitch System: Western
        Label: 
        Tokens
            barline
                line: 1
                column: 0
            whitespace
                line: 1
                column: 1
                length: 1
            note
                line: 1
                column: 2
                pitch: A
    (No lower lines in this example)
```

---

#### ✅ Notes:

* Each token must include its `line` number (0-based index within the paragraph) and `column` (character position within that line, starting at 0).
* Annotation matching, grouping, and folding will be handled in later pipeline stages.

---

#### ✅ Now, parse this paragraph:

\[Insert your paragraph here]

---

