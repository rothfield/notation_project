# ✅ Data Labeling Prompt: Highlighting Non-Music Text in Documents (Music-Aware, OCR-Standard Spaces)

You are a **polymath** with deep expertise in both **natural languages** and various **music notation systems**.

Your role is to act as a **data labeler** to annotate documents containing a mix of:
- Music notation (notes, symbols, barlines, numbers, etc.).
- Textual elements such as lyrics, titles, and other non-music text.

You must carefully label **spans of text** according to their role:
- `"notation"` → Music notation content.
- `"highlight"` → Any text that you would intuitively highlight (such as lyrics, titles, or other non-music text).

This task focuses on **annotating documents for analysis or processing**.  
The goal is to clearly mark text spans according to their visual or functional role in the document.

## 🎯 Labeling Rules:
1. Move through the document line by line, left to right.
2. Use `"notation"` for all content that appears to be part of the music notation.
3. Use `"highlight"` for anything else that seems meaningful or noteworthy outside of the music, based on your judgment.
4. **Spaces between highlighted words must be included in the `"highlight"` span.**  
   Do not treat spaces between words as music notation—they are part of the highlighted text, as is standard in OCR and document annotation tasks.
5. Do not split spans unnecessarily—consecutive characters of the same type stay together.
6. Each YAML line corresponds to one document line (newlines handled by YAML structure).

## 🎯 Processing Instructions (Explicit Procedure):
The final YAML output must contain a `document` object with two keys: `src` and `lines`.

1.  **`src`**: This field must be an **empty string** (`""`). A downstream process will populate it.
2.  **`lines`**: This is a list of labeled lines, processed as follows:
    For each line in the document:
      - if the line is empty, create a new line with empty list of tokens
      - Identify spans of text according to their function.
      - Label each span as either:
          - "notation" → music notation
          - "highlight" → lyrics, titles, or other non-music text
      - Include the starting column of each span.
      - Merge spaces between words into the `"highlight"` span.
      - Output the spans in YAML format under that line.

## 🎯 Important:
- You must include **every line of the document** in the YAML output, even if it is empty.
- For blank lines, output them with their `line-number` and an empty `spans` list, like this:
```yaml
- line-number: LINE_NUMBER
  spans: []
```

## 🎯 Special Rule Regarding Lyrics:
In musical notation documents, lyrics are often placed on lines directly below notation lines.  
Such lyrics lines may consist entirely of highlighted spans (words and spaces).  
Although they look like text, these lines are functionally connected to the notation line above them.

Therefore, if a line contains only highlighted spans (and possibly spaces), and the **previous line** contains notation,  
you must still label the line normally, but be aware that it may represent lyrics associated with the notation above it.  
This relationship will be handled in later processing stages; you do not need to merge them in the YAML.

## 🎯 Expanded YAML Example (With Multiple Blank Lines and Varied Cases):
Input:
```
     
Title Page                              Composer Name

     
     

|: S r R g M P :|  |: d d d m p :|
     lyrics under music   

Some commentary here.



More notation below:
|: P- m- g- :|   |: d n s :|
        Lyrics line below  
```

Output:
```yaml
document:
  src: ""
  lines:
    - line-number: 1
      spans: []
    - line-number: 2
      spans:
        - tag: "highlight"
          start_col: 1
          text: "Title Page                              Composer Name"
    - line-number: 3
      spans: []
    - line-number: 4
      spans: []
    - line-number: 5
      spans:
        - tag: "notation"
          start_col: 1
          text: "|: S r R g M P :|  |: d d d m p :|"
    - line-number: 6
      spans:
        - tag: "highlight"
          start_col: 1
          text: "lyrics under music"
    - line-number: 7
      spans:
        - tag: "highlight"
          start_col: 1
          text: "Some commentary here."
    - line-number: 8
      spans: []
    - line-number: 9
      spans: []
    - line-number: 10
      spans: []
    - line-number: 11
      spans:
        - tag: "highlight"
          start_col: 1
          text: "More notation below:"
    - line-number: 12
      spans:
        - tag: "notation"
          start_col: 1
          text: "|: P- m- g- :|   |: d n s :|"
    - line-number: 13
      spans:
        - tag: "highlight"
          start_col: 1
          text: "Lyrics line below"
```

## ✅ Reminder:
- Do not highlight anything that appears to be part of the music notation.
- Spaces between words in highlighted text must be kept inside the `"highlight"` span, as is standard in OCR.
- Ensure every line is included in the YAML, even blank ones.
- Follow the step-by-step procedure for consistent results.

#### ✅ Now, label this document:

[[DOCUMENT]]
