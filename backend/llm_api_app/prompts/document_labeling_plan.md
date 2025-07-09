# Document Labeling Planning Session and Discussion

## Summary

This document summarizes a focused planning session on Stage 1 of a music notation parsing pipeline. The discussion aimed to define a minimal, robust, and fully reversible Stage 1 that only detects and labels tokens from handwritten or typed notation documents, preserving full positional and textual data.

---

## Core Strategy

**Stage 1 Goals:**

- Detect and label tokens without any structural assumptions.
- Fully preserve document data (line numbers, columns, original text).
- Defer musical attachment, parsing, or segmentation to later stages.

---

## Stage 1 Responsibilities

1. **Tokenize text into meaningful tokens:**

   - Notes (e.g., S, r, g)
   - Barlines (e.g., |, :|, |:)
   - Ornaments (e.g., \~, parentheses)
   - Octave markers (e.g., ".")
   - Lyrics (syllables aligned with music)
   - Text (ordinary words)
   - Punctuation (optional category)

2. **Label Each Token:**

   - Labels include: `note`, `barline`, `ornament`, `ornament_start`, `ornament_end`, `lyric`, `octave_marker`, `text`, `punctuation`.

3. **Preserve Metadata:**

   - Line number (starting from 1).
   - Column number (starting from 1).
   - Original token text.

---

## Key Design Principles

- No main-line assumptions.
- No grouping or attachment performed in this stage.
- Output must be fully reversible and auditable.
- Clean separation of detection and later structural processing.

---

## Output Format Example (YAML/JSON):

```yaml
tokens:
  - line: 2
    column: 4
    text: "S"
    label: note
  - line: 2
    column: 5
    text: "("
    label: ornament_start
  - line: 2
    column: 6
    text: "r"
    label: note
```

---

## Prompt Strategy

- LLM prompt must:
  - Accept tokenized text with line and column data.
  - Classify tokens according to the spec.
  - Output labeled tokens in structured JSON or YAML.

---

## CLI Review Tool (Preferred Human Review Method)

- Build a CLI-based tool that:
  - Reads the labeled JSON.
  - Displays document lines with color-coded tokens using ANSI terminal colors.
  - Highlights tokens based on their labels for easy human review.

### Example CLI Workflow:

```bash
cat labeled_output.json | ./colorize_tokens.py | less -R
```

- This avoids the need for complex GUIs while remaining human-friendly.
- Compatible with tools like `bat`, `fzf`, and terminal multiplexers.

---

## Advantages

- Minimal, deterministic, and reusable.
- Fully reversible and safe.
- Future stages can attach, segment, or transform tokens as needed.
- Allows fast iteration and debugging.

---

## Saved Keywords for Search and Future Retrieval

```
music notation pipeline
stage 1 token labeling
CLI colorized token viewer
limited scope, data preserved
no main line assumptions
LLM prompt vs spec
```

---

## Next Possible Steps

1. Formalize tokenizer spec.
2. Write working LLM prompt.
3. Implement CLI colorizer.
4. Validate against both simple and complex notation examples.

---

**Key Insight:**

> First, precisely detect and fully preserve data. Defer structural interpretation until later.

