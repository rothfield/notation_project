# Tokenize Block Prompt

You are a specialist in parsing lines of letter music notation into a structured parse tree.

**Task:**  
Given a single block (a paragraph, possibly multi-line) of letter notation, output a strict token-level parse in **YAML format**.

**Instructions:**

- Classify each line as `upper_lines` (annotations above), `main_line` (the main music/notation line), or `lower_lines` (annotations or lyrics below).
- Tokenize each line into a list of tokens.  
- For each token, include:
    - `type` (e.g., note, barline, dot, dash, syllable, text, space, etc.)
    - `line` (0-based index within the block)
    - `column` (character position)
    - For notes, add: `pitch`
    - For syllables or text, add: `text`
    - For spaces or dashes, add: `length`
- Output must be **valid YAML**.
- Structure:  
    - Top-level keys: `upper_lines`, `main_line`, `lower_lines`
    - Each is a list of lines.  
    - Each line is a mapping with a `tokens` list.

**Example output:**

```yaml
main_line:
  - tokens:
      - type: note
        line: 1
        column: 0
        pitch: S
      - type: space
        line: 1
        column: 1
        length: 1
      - type: note
        line: 1
        column: 2
        pitch: R
upper_lines:
  - tokens:
      - type: dot
        line: 0
        column: 2
lower_lines:
  - tokens:
      - type: syllable
        line: 3
        column: 0
        text: hi
```

**Notes:**
- The input may have zero or more annotation lines above or below the main line.
- Always preserve the physical positions of tokens (`line`, `column`).
- If a token does not apply (e.g., no text for a note), omit that key.
- Never output in outline/tree format—**only YAML is accepted**.
- Never merge tokens across lines; tokenize each line independently.

---

<!--  
(Sinatra or other logic for inserting input goes below this line; do not overwrite.)  
-->
-[Insert Raw Document Content Here]

