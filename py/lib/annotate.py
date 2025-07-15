def add_annotations_to_notation_spans(document, src_text):
    lines = document["lines"]
    notation_lines = []
    for i, line in enumerate(lines):
        for span in line.get("spans", []):
            if span["tag"] != "notation": continue
            tokens = []
            text = span["text"]
            start_col = span["start_col"]
            for offset, char in enumerate(text):
                if char == " ": continue
                kind = classify_char(char)
                tokens.append({
                    "kind": kind,
                    "text": char,
                    "column": start_col + offset
                })
            for neighbor_offset in [-1, 1]:
                j = i + neighbor_offset
                if j < 0 or j >= len(lines): continue
                for ann in lines[j].get("spans", []):
                    if ann.get("tag") != "highlight": continue
                    for t in tokens:
                        if abs(t["column"] - ann["start_col"]) <= 1:
                            t.setdefault("annotation", []).append(ann["text"])
                            ann["attached"] = True
            notation_lines.append({
                "line_number": line["line-number"],
                "tokens": tokens
            })
    return {
        "notation_document": {
            "src": src_text,
            "lines": notation_lines
        }
    }

def classify_char(char):
    if char in "|:": return "barline"
    if char in "-_": return "dash"
    if char in "~": return "ornament"
    if char.isdigit(): return "note"
    if char.isalpha(): return "note"
    return "symbol"
