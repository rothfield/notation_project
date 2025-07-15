#!/usr/bin/env fish
# PATCH_MSG: feat: add add_annotations_to_notation_spans() with CLI and attached:true tracking

set PATCH_FILE lib/annotate.py
set CLI_FILE cli/add_annotations.py

for FILE in $PATCH_FILE $CLI_FILE
    if test -e $FILE
        read --prompt-str "⚠️ Overwrite $FILE? [y/N] " -l confirm
        if test "$confirm" != "y"
            echo "⏭️ Skipping write of $FILE"
            exit 1
        end
    end
end

mkdir -p lib cli

# Write annotate.py
begin
    echo 'def add_annotations_to_notation_spans(document, src_text):'
    echo '    lines = document["lines"]'
    echo '    notation_lines = []'
    echo '    for i, line in enumerate(lines):'
    echo '        for span in line.get("spans", []):'
    echo '            if span["tag"] != "notation": continue'
    echo '            tokens = []'
    echo '            text = span["text"]'
    echo '            start_col = span["start_col"]'
    echo '            for offset, char in enumerate(text):'
    echo '                if char == " ": continue'
    echo '                kind = classify_char(char)'
    echo '                tokens.append({'
    echo '                    "kind": kind,'
    echo '                    "text": char,'
    echo '                    "column": start_col + offset'
    echo '                })'
    echo '            for neighbor_offset in [-1, 1]:'
    echo '                j = i + neighbor_offset'
    echo '                if j < 0 or j >= len(lines): continue'
    echo '                for ann in lines[j].get("spans", []):'
    echo '                    if ann.get("tag") != "highlight": continue'
    echo '                    for t in tokens:'
    echo '                        if abs(t["column"] - ann["start_col"]) <= 1:'
    echo '                            t.setdefault("annotation", []).append(ann["text"])'
    echo '                            ann["attached"] = True'
    echo '            notation_lines.append({'
    echo '                "line_number": line["line-number"],'
    echo '                "tokens": tokens'
    echo '            })'
    echo '    return {'
    echo '        "notation_document": {'
    echo '            "src": src_text,'
    echo '            "lines": notation_lines'
    echo '        }'
    echo '    }'
    echo
    echo 'def classify_char(char):'
    echo '    if char in "|:": return "barline"'
    echo '    if char in "-_": return "dash"'
    echo '    if char in "~": return "ornament"'
    echo '    if char.isdigit(): return "note"'
    echo '    if char.isalpha(): return "note"'
    echo '    return "symbol"'
end > $PATCH_FILE

# Write CLI wrapper
begin
    echo 'import sys'
    echo 'import yaml'
    echo 'from pathlib import Path'
    echo 'from lib.annotate import add_annotations_to_notation_spans'
    echo
    echo 'def main():'
    echo '    if len(sys.argv) != 3:'
    echo '        print("Usage: python cli/add_annotations.py source.txt source.yml")'
    echo '        sys.exit(1)'
    echo '    src = Path(sys.argv[1]).read_text()'
    echo '    doc = yaml.safe_load(Path(sys.argv[2]).read_text())'
    echo '    result = add_annotations_to_notation_spans(doc, src)'
    echo '    yaml.dump(result, sys.stdout, sort_keys=False, allow_unicode=True)'
    echo
    echo 'if __name__ == "__main__":'
    echo '    main()'
end > $CLI_FILE

# Commit
git add $PATCH_FILE $CLI_FILE
git commit -m "feat: add add_annotations_to_notation_spans() with CLI and attached:true tracking"

