#!/usr/bin/env python3
import sys
import yaml

LABEL_COLORS = {
    "notation": "\033[32m",
    "highlight": "\033[34m",
}
RESET = "\033[0m"

# Load YAML from stdin
try:
    data = yaml.safe_load(sys.stdin)
except yaml.YAMLError as e:
    sys.exit(f"❌ Failed to parse YAML: {e}")

lines = {}

# Process YAML document structure
for line_entry in data.get("document", []):
    line_data = line_entry.get("line", {})
    line_num = line_data.get("line-number")
    spans = line_data.get("spans", [])
    if line_num is None:
        continue
    line_spans = []
    for span in spans:
        tag = span.get("tag")
        start_col = span.get("start_col", 1) - 1
        text = span.get("text", "")
        color = LABEL_COLORS.get(tag, "\033[37m")
        line_spans.append((start_col, text, color))
    lines[line_num] = sorted(line_spans)

# Build and print lines with colorized spans
for line_num in sorted(lines.keys()):
    line = ""
    cursor = 0
    for col, text, color in lines[line_num]:
        if col > cursor:
            line += " " * (col - cursor)
        line += f"{color}{text}{RESET}"
        cursor = col + len(text)
    print(line)

# Reconstitute and print the plain document below the colored version
print("\n\nReconstituted Document:\n" + "=" * 30)
all_line_nums = range(1, max(lines.keys()) + 1)
for line_num in all_line_nums:
    if line_num in lines:
        plain_line = ""
        cursor = 0
        for col, text, _color in lines[line_num]:
            if col > cursor:
                plain_line += " " * (col - cursor)
            plain_line += text
            cursor = col + len(text)
        print(plain_line)
    else:
        print("")

# Print the original YAML document
print("\n\nYAML Document:\n" + "=" * 30)
print(yaml.dump(data, sort_keys=False, allow_unicode=True))