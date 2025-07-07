Here is the complete prompt for the "Fold Annotations" stage, incorporating all the details and refinements we've discussed:

-----

## Prompt: Music Notation Parsing (Fold Annotations Stage)

You are an expert music notation processor, specializing in structurally integrating disparate layers of musical information. Your task in this stage is to "fold" the upper and lower annotation lines into the main notation line, creating a single, enriched sequence of musical events. This process involves associating annotation tokens (octave dots, ornament notes, lyrics) with their corresponding main line tokens (notes, dashes) based on their column positions.

The output from the previous "Line Tokenization" stage (which you will receive as input) provides a detailed, tokenized outline of a music notation paragraph, preserving `line` and `column` information for each token. Your output for this stage should be a flattened list of the main line tokens, with their newly applied annotations as nested properties.

### Input Format

You will receive the output of the "Line Tokenization" stage, which is an indented outline tree structured as follows:

```plaintext
Paragraph
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
```

Each `[token]` in the input has at least `line` and `column` properties, and may include `pitch`, `text`, `length`, `type`, etc.

-----

### Parsing Instructions

#### Step 1 — Align and Apply Annotations

For each token in the **Main Line's `Tokens` list**, attempt to find and apply relevant annotations from the **Upper Line** and **Lower Line** tokens based on column alignment.

1.  **Column Alignment Rule:** An annotation token is considered "aligned" with a main line token if its `column` falls within the range of the main line token's `column` to (`column` + `length` - 1), or within a tolerance of **± 1 character column** from the main line token's starting `column`. Prioritize exact column matches over matches within the tolerance. If multiple annotations match within the tolerance, apply all relevant ones according to their type and the rules below.

2.  **Applying Upper Annotation Tokens:**

      * **`dot` tokens (`.` or `:`):** These indicate octave shifts.
          * A single dot (`.`) increases the octave of the **aligned `note` token** by **+1**.
          * A double dot (`:`) increases the octave of the **aligned `note` token** by **+2**.
          * If a `note` token already has an `octave` property (e.g., from a previous default or explicit notation if it existed), adjust it accordingly. If no `octave` property exists, assume a base octave of `0` before applying the shift.
          * Example: A `note` token "S" at `column: 2` with an upper `dot` at `column: 2` would result in the "S" having `octave: 1` (assuming base 0).
      * **`note` tokens (representing ornaments):** These should be stored as an array of `ornament` notes within the aligned main line `note` token.
          * For a main line `note` token, if one or more `note` tokens from an `Upper Line` align with it, collect these `note` tokens. Create a new `ornaments` property on the main line `note` token, which is an array containing these collected `note` tokens (preserving their original `pitch`, `line`, `column`, etc.).
          * Example: Main line "C" at `column: 5` with upper line "D E" where "D" is at `column: 5` and "E" at `column: 6`. The "C" token would gain an `ornaments` array containing the "D" and "E" tokens.

3.  **Applying Lower Annotation Tokens:**

      * **`syllable` tokens:** These represent lyrics.
          * Attach the `text` content of the `syllable` token to the **aligned `note` or `dash` token** on the main line. Create a new `lyrics` property on the main line token, storing the syllable's `text` value. If multiple syllables align with the same note/dash (e.g., in complex rhythm), concatenate them with a **space character**.
          * Example: Main line "C" at `column: 2` with lower line "la" at `column: 2`. The "C" token would gain `lyrics: "la"`.
      * **`dot` tokens (`.` or `:`):** If present in lower lines, these signify downward octave shifts.
          * A single dot (`.`) decreases the octave of the **aligned `note` token** by **-1**.
          * A double dot (`:`) decreases the octave of the **aligned `note` token** by **-2**.
          * Apply this shift after any upper octave shifts.

#### Step 2 — Construct Flattened Output

Create a single, flattened array (or list) of **only the enriched Main Line tokens**. Each token in this output array should include all its original properties from the Line Tokenization stage, plus any newly added `octave`, `ornaments`, or `lyrics` properties.

  * `space` tokens should be preserved in the flattened output, as they contribute to horizontal spacing.
  * The output should not include `Upper Line` or `Lower Line` wrapper objects; their content has been integrated.

-----

### Output Format

A flat list of enriched main line tokens. Each token should be a dictionary/object.

```json
[
    {
        "type": "note",
        "line": 1,
        "column": 0,
        "pitch": "S",
        "octave": 0, // Added or modified
        "lyrics": "hi" // Added
    },
    {
        "type": "dash",
        "line": 1,
        "column": 1
    },
    {
        "type": "note",
        "line": 1,
        "column": 2,
        "pitch": "R",
        "ornaments": [ // Added
            {
                "type": "note",
                "line": 0,
                "column": 4,
                "pitch": "C"
            },
            {
                "type": "note",
                "line": 0,
                "column": 5,
                "pitch": "D"
            }
        ]
    }
    // ... more tokens
]
```

-----

### Example Input (Output from Line Tokenization Stage)

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
            note
                line: 0
                column: 4
                pitch: C
            note
                line: 0
                column: 5
                pitch: D
    Main Line
        Pitch System: Sargam
        Label: 
        Tokens
            note
                line: 1
                column: 0
                pitch: S
            dash
                line: 1
                column: 1
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
    Lower Line
        Tokens
            dot
                line: 2
                column: 0
            syllable
                line: 3
                column: 0
                text: "hi"
            syllable
                line: 3
                column: 1
                text: "there"
```

-----

### Example Output (JSON format for clarity)

```json
[
    {
        "type": "note",
        "line": 1,
        "column": 0,
        "pitch": "S",
        "octave": 1,
        "lyrics": "hi"
    },
    {
        "type": "dash",
        "line": 1,
        "column": 1
    },
    {
        "type": "note",
        "line": 1,
        "column": 2,
        "pitch": "R",
        "ornaments": [
            {
                "type": "note",
                "line": 0,
                "column": 4,
                "pitch": "C"
            },
            {
                "type": "note",
                "line": 0,
                "column": 5,
                "pitch": "D"
            }
        ]
    },
    {
        "type": "space",
        "line": 1,
        "column": 3,
        "length": 1
    },
    {
        "type": "note",
        "line": 1,
        "column": 4,
        "pitch": "G",
        "lyrics": "there"
    }
]
```

-----

### Notes

  * Remember the 0-based indexing for `line` and `column`.
  * Maintain all existing token properties from the previous stage, only adding or modifying `octave`, `ornaments`, and `lyrics`.
  * The `Pitch System` and `Label` from the `Main Line` itself are context for the parsing, but do not need to be carried into every individual token in the flattened output.
  * Prioritize applying annotations. If there's an ambiguity in column alignment, applying the annotation to the closest main line token is usually preferred. If multiple main line tokens are equidistant, choose the one appearing first (leftmost).
