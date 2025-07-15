# Prompt: tokenize_notation_spans
# Purpose: Given a plaintext paragraph and a marked span of music notation, segment it into musical tokens with accurate kind and column.

## Instructions to the LLM:
You are a music notation tokenizer.

Your task is to examine a **raw text paragraph** that includes music notation.
One line is selected with `<<<<<<` markers to the right.
A specific span within that line is highlighted with `^^^^^^` characters in the next line.

Your job is to tokenize the highlighted notation span into **musical tokens**.

### Output:
Produce a list of YAML tokens. Each token must include:
- `text`: the exact characters of the token
- `column`: the 0-based index (from start of line) where the token begins
- `kind`: one of the following kinds:
  - `pitch` (e.g., C, D, F#)
  - `barline` (e.g., `|`, `|:`, `:|`)
  - `space` (one or more space characters)
  - `dash` (used to extend duration, e.g., `-`)
  - `breath` (apostrophe: `'`)
  - `ornament` (e.g., `~` for mordent)
  - `repeat` (for repeat symbols like `:|`, `|:`)

> Do not repeat pitches.
> Sharps and flats are part of the pitch token (e.g., `F#`, `C#`, `Bb`).
> Most tokens are 1–3 characters long.
> Tokens may be adjacent (e.g., `123CDE`).

### Example Input:
```
Happy john
             DEF .
|:  CC  DD  E-F -G |  CDEF FED-  :|     <<<<<<
     .      lyric       $
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
hi-  john
```

### Example Output:
```yaml
tokens:
  - text: "|:"
    column: 0
    kind: "barline"
  - text: "  "
    column: 2
    kind: "space"
  - text: "C"
 Happy john
               DEF .
|:  CC  DD  E-F -G |  CDEF FED-  :|     <<<<<<
     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
hi-  john   column: 4
    kind: "pitch"
  - text: "C"
    column: 5
    kind: "pitch"
  - text: "  "
    column: 6
    kind: "space"
  - text: "D"
    column: 8
    kind: "pitch"
  - text: "D"
    column: 9
    kind: "pitch"
  - text: "  "
    column: 10
    kind: "space"
  - text: "E"
    column: 12
    kind: "pitch"
  - text: "-"
    column: 13
    kind: "dash"
  - text: "F"
    column: 14
    kind: "pitch"
 Gregorian chant



R--- <<<<< - text: " "
    column: 15
    kind: "space"
  - text: "-"
    column: 16
    kind: "dash"
  - text: "G"
    column: 17
    kind: "pitch"
  - text: " "
    column: 18
    kind: "space"
  - text: "|"
    column: 19
    kind: "barline"
  - text: "  "
    column: 20
    kind: "space"
  - text: "C"
    column: 22
    kind: "pitch"
  - text: "D"
    column: 23
    kind: "pitch"
  - text:Happy john
               DEF .
|:  CC  DD  E-F -G |  CDEF FED-  :|     <<<<<<
     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
hi-  john "E"
    column: 24
    kind: "pitch"
  - text: "F"
    column: 25
    kind: "pitch"
  - text: " "
    column: 26
    kind: "space"
  - text: "F"
    column: 27
    kind: "pitch"
  - text: "E"
    column: 28
    kind: "pitch"
  - text: "D"
    column: 29
    kind: "pitch"
  - text: "-"
    column: 30
    kind: "dash"
  - text: "  "
    column: 31
    kind: "space"
  - text: ":|"
    column: 33
    kind: "barline"


#### ✅ Now, parse this :

[[INPUT]]

```
```
