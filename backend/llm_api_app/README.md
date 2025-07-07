# Music Notation Project LLM Prompts 

This system parses monospaced text-based musical notation into structured formats, such as LilyPond or SVG. It handles letter-based notation with annotations above and below the main line. This directory contains the LLM prompts, which are used as an API.

---


### Document parser
Split document into:
- Header (optional)
- Footer (optional)
- **Blocks** (paragraphs)

Each block consists of lines that may contain:
- Main notation line (with notes, barlines, etc.)
- Annotation lines (above and below main line)
- Lyrics (below the main line)
- each block is fed into the tokenizer
---
Output: a document parsed into header and multiple blocks
IE
Document
   Header
   Block
     Line
       text
     Line
       text
 

### 3. Tokenize Block
This tokenizes a single block
- Identify the **main notation line** by detecting pitch system and barlines.
- Tokenize lines into tokens with column positions.
- Detect upper/lower annotation lines and lyrics.

**Output Example:**
```text
original input:
|: S R g m P D :|
   hello

Output

TODO
