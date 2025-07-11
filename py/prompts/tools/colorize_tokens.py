#!/usr/bin/env python3
import sys
import yaml

# ANSI color codes for highlighting different labels
LABEL_COLORS = {
    "highlight": "\033[33m",  # Yellow for highlighted text (lyrics, titles)
    "notation": "\033[37m",   # White for music notation
    "note": "\033[32m",       # Green
    "barline": "\033[34m",    # Blue
    "ornament": "\033[35m",   # Magenta
    "lyric": "\033[33m",      # Yellow
    "octave_marker": "\033[36m", # Cyan
    "text": "\033[37m",       # White
    "punctuation": "\033[90m",# Grey
}
RESET = "\033[0m"

def main():
    # Load YAML from stdin
    try:
        data = yaml.safe_load(sys.stdin)
        doc = data.get("document", {})
        if not doc or "src" not in doc or "lines" not in doc:
            sys.exit("❌ YAML input is missing 'document.src' or 'document.lines'.")
    except yaml.YAMLError as e:
        sys.exit(f"❌ Failed to parse YAML: {e}")

    # --- 1. Prepare original and colorized lines ---
    original_lines = doc.get("src", "").splitlines()
    lines_data = doc.get("lines", [])
    
    colorized_lines_map = {}
    used_labels = set()

    for line_obj in lines_data:
        line_num = line_obj.get("line-number")
        if line_num is None:
            continue

        spans = sorted(line_obj.get("spans", []), key=lambda s: s.get("start_col", 1))
        line_str = ""
        cursor = 0
        for span in spans:
            col = span.get("start_col", 1) - 1
            text = span.get("text", "")
            label = span.get("tag", "text")
            
            used_labels.add(label)
            color = LABEL_COLORS.get(label, LABEL_COLORS["text"])
            
            if col > cursor:
                line_str += " " * (col - cursor)
            line_str += f"{color}{text}{RESET}"
            cursor = col + len(text)
        colorized_lines_map[line_num] = line_str

    # --- 2. Display side-by-side ---
    max_original_width = max(len(line) for line in original_lines) if original_lines else 0
    num_lines = max(len(original_lines), max(colorized_lines_map.keys()) if colorized_lines_map else 0)

    print(f"{'Original'.ljust(max_original_width)} | Colorized")
    print(f"{'-' * max_original_width}-+--{'-' * max_original_width}")

    for i in range(num_lines):
        line_num = i + 1
        original = original_lines[i] if i < len(original_lines) else ""
        colorized = colorized_lines_map.get(line_num, "")
        print(f"{original.ljust(max_original_width)} | {colorized}")

    # --- 3. Display Legend ---
    if used_labels:
        print("\n--- Legend ---")
        # Sort for consistent output
        for label in sorted(list(used_labels)):
            color = LABEL_COLORS.get(label, LABEL_COLORS["text"])
            print(f"{color}███{RESET} : {label}")

if __name__ == "__main__":
    main()

