# Stage 1 Token Labeling Prompt

You are a precise document labeler working on the first stage of a music notation parsing pipeline.  
Your task is to detect and label tokens from the provided text document without making any structural assumptions.

---

## 🎯 Input:
The document consists of lines of handwritten or typed music notation. It may contain:
- Notes (e.g., S, r, g, m, P, etc.)
- Barlines (e.g., |, :|, |:)
- Ornaments (e.g., ~, parentheses, etc.)
- Octave markers (e.g., ".")
- Lyrics (syllables aligned with music)
- Text (ordinary words)
- Punctuation (optional)

---

## 🎯 Token Labels:
For each detected token, assign one of these labels:
- `note`
- `barline`
- `ornament`
- `ornament_start`
- `ornament_end`
- `lyric`
- `octave_marker`
- `text`
- `punctuation`

---

## 🎯 Output Format:
For every token you detect, output:
- **line**: Line number (starting at 1)
- **column**: Column number (starting at 1, counting all characters including spaces)
- **text**: Exact token text as in the document
- **label**: One of the token labels listed above

Output as **JSON**:
```json
{
  "tokens": [
    {
      "line": LINE_NUMBER,
      "column": COLUMN_NUMBER,
      "text": "TOKEN_TEXT",
      "label": "TOKEN_LABEL"
    },
    ...
  ]
}
```

---

## 🎯 Instructions:
- Preserve all text exactly as-is.
- Do NOT group, attach, or reorder tokens.
- Avoid assumptions about musical structure or hierarchy.
- Every character matters for positions.
- If uncertain about a token’s category, use `"text"`.
- Do not skip or omit any characters.
- Output must be fully reversible and auditable.

---

## 🎯 Example

### Input:
```
|: S r g m P d n S :|
hi -- -- ly - --
```

### Output:
```json
{
  "tokens": [
    {"line": 1, "column": 1, "text": "|:", "label": "barline"},
    {"line": 1, "column": 4, "text": "S", "label": "note"},
    {"line": 1, "column": 6, "text": "r", "label": "note"},
    {"line": 1, "column": 8, "text": "g", "label": "note"},
    {"line": 1, "column": 10, "text": "m", "label": "note"},
    {"line": 1, "column": 12, "text": "P", "label": "note"},
    {"line": 1, "column": 14, "text": "d", "label": "note"},
    {"line": 1, "column": 16, "text": "n", "label": "note"},
    {"line": 1, "column": 18, "text": "S", "label": "note"},
    {"line": 1, "column": 20, "text": ":|", "label": "barline"},
    {"line": 2, "column": 1, "text": "hi", "label": "lyric"},
    {"line": 2, "column": 4, "text": "--", "label": "lyric"},
    {"line": 2, "column": 7, "text": "--", "label": "lyric"},
    {"line": 2, "column": 10, "text": "ly", "label": "lyric"},
    {"line": 2, "column": 13, "text": "-", "label": "lyric"},
    {"line": 2, "column": 16, "text": "--", "label": "lyric"}
  ]
}
```

---

> **Reminder:**  
> First, detect and preserve every token precisely.  
> Defer all structural analysis or grouping to later pipeline stages.



#### ✅ Now, parse this :

[[DOCUMENT]]

```
```
