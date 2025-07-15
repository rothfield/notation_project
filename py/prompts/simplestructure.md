# V3
# Task: Highlight Musical Runs

You will be given a line of text that may contain a run of musical notation.
Your task is to identify and extract the musical part.

A "run of music" contains:
- Notes (e.g., S, r, R, g, M, P)
- Barlines (|)
- Rhythm dashes (-)

Everything else is considered non-musical.

Your output MUST be a single YAML code block containing a single key, "highlighted_run".

## Example
### Input:
```
|: S r R g M P :|   ' ( ) % x2 Play slowly
```
### Output:
```yaml
highlighted_run: "|: S r R g M P :|"
```

---
## Input:
```
[[INPUT]]
```

## Output: