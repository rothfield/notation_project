
# identify_structural_elements Prompt

## Purpose:
Identify and segment the *structural elements* of a musical document.

This is the **first stage** of the pipeline, working on the raw, unprocessed document text.
It detects and classifies lines into structural categories such as:
- Notation lines (containing music notation, lyrics, or notation-related text)
- Commentary or non-musical text

This stage also applies positional rules to reclassify certain lines—such as recognizing
lyrics that visually belong to notation lines based on their position.

## Key Properties:
- Starts with a blank slate (no prior segmentation).
- Operates purely at the structural document level, not on musical content yet.
- Produces labeled structural elements that are passed to later pipeline stages.

## Why This Name:
The name “identify_structural_elements” reflects that this stage identifies the large-scale
structure of the document, rather than performing detailed musical analysis.

## Example Output:
- A document with lines tagged as "notation" or "highlight" (for text/commentary).

---


# Prompt: Highlight Runs of Music Inside Notation Blocks

## Background:
Historically, music notation has always been designed for clear communication. Musicians, scribes, and composers often arranged their music in visually structured ways—using columns, spacing, and groupings—to make it easier to read, perform, and align with other text or annotations.

Inside notation blocks, you will often find not only musical notes and rhythms, but also various supporting symbols and annotations. These symbols are typically brief and pragmatic—intended to help performers interpret or repeat music efficiently.

Common symbols that may appear inside notation blocks include:
- **Phrase markers:** Ticks or apostrophes (`'`)
- **Slurs:** Parentheses (`(` and `)`)
- **Repeat signs:** Percent signs (`%`)
- **Octave markings:** Dots (`.` or `:`), numbers, or shorthand signs
- **Barlines:** Vertical bars (`|`)
- **Spaces:** For alignment or separation
- **Other brief directions or repeat counts:** e.g., `x2`, `Play slowly`

All of these symbols may appear alongside or between runs of music.

---

## Task:
Your task is to carefully highlight the **runs of music** inside each notation block.

### A **run of music** includes:
- **Notes or pitch symbols** (letters or notation symbols representing pitches)
- **Dashes (`-`)** representing rhythmic duration or rests
- **Barlines (`|`)** separating musical sections

### Important:
- Do not ignore or remove other symbols (like phrase markers, slurs, repeat signs, or annotations).
- You do not need to highlight or process those other symbols at this stage.
- Simply **highlight the runs of music**—leaving the rest untouched.

---

## Example:

### Input Notation Block:
```
|: S r R g M P :|   ' ( ) % x2 Play slowly
```

### Expected Output:
```
Highlighted musical runs:
1. "|: S r R g M P :|" (Columns 1–15)
```

---

### Notes:
- The phrase marker (`'`), slurs (`(` and `)`), repeat sign (`%`), and the text (`x2 Play slowly`) remain untouched.
- Only the musical segment is highlighted.

---

## Summary:
Your only task is to highlight the musical runs—notes, dashes, and barlines—inside each notation block, while leaving all other content unchanged.
