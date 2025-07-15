You are given a text document representing musical notation.

- The text is organized in columns. Notes can have annotations above or below 
- Dashes (-) are rhythmic placeholders.
- Dots (.) above and below notes indicate octave shifts.
- Beats are separated by whitespace.
- Notes and dashes form beats.
- Bar lines are marked using |, :|, and |: symbols.
- Line labels (e.g., 1)) may precede a line of music.
- Beats are adjacent notes and dashes. for example, C---D is
a single beat with 5 divisions.  S- is a beat with 2 divisions. -- is a beat with 2 divisions.
- Notes are represented by the letters C D E F G A B and C# 
- Notes are not separated by spaces. so C#CC is 3 notes
---
See notation_example.yml
✅ Now, parse this:

[[INPUT]]

