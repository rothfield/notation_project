#!/usr/bin/env python3
import json
import sys

LABEL_COLORS = {
    "note": "\033[32m",
    "barline": "\033[34m",
    "ornament": "\033[35m",
    "ornament_start": "\033[35m",
    "ornament_end": "\033[35m",
    "lyric": "\033[33m",
    "octave_marker": "\033[36m",
    "text": "\033[37m",
    "punctuation": "\033[90m",
}
RESET = "\033[0m"

# Load JSON from stdin
try:
    data = json.load(sys.stdin)
except json.JSONDecodeError as e:
    sys.exit(f"❌ Failed to parse JSON: {e}")

# Handle OpenAI-style output if needed
if "choices" in data and isinstance(data["choices"], list):
    content = data["choices"][0].get("message", {}).get("content", "")
    try:
        tokens = json.loads(content).get("tokens", [])
    except json.JSONDecodeError:
        sys.exit("❌ Failed to parse inner content as JSON.")
else:
    tokens = data.get("tokens", [])

# Group tokens by line number
lines = {}
for tok in tokens:
    line_num = tok["line"]
    col = tok["column"] - 1
    text = tok["text"]
    label = tok["label"]
    color = LABEL_COLORS.get(label, "\033[37m")
    if line_num not in lines:
        lines[line_num] = []
    lines[line_num].append((col, text, color))

# Build and print lines with colorized tokens
for line_num in sorted(lines.keys()):
    line = ""
    cursor = 0
    for col, text, color in sorted(lines[line_num]):
        if col > cursor:
            line += " " * (col - cursor)
        line += f"{color}{text}{RESET}"
        cursor = col + len(text)
    print(line)

