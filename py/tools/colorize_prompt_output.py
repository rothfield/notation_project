#!/usr/bin/env python3
import sys
import yaml
import re

RESET = "\033[0m"
COLOR = {
    "note": "\033[32m",
    "dash": "\033[90m",
    "barline": "\033[34m",
    "lyric": "\033[33m",
    "octave_marker": "\033[36m",
    "text": "\033[37m",
}

def get_display_width(text):
    return len(re.sub(r'\033\[[0-9;]*m', '', text))

def classify(token):
    # Match keys regardless of capitalization
    token_keys = list(token.keys())
    upper_keys = [k.upper() for k in token_keys]
    if "NOTE" in upper_keys:
        return "note"
    elif "DASH" in upper_keys:
        return "dash"
    elif any(k in upper_keys for k in ["BARLINE", "LEFTREPEATBARLINE", "RIGHTREPEATBARLINE"]):
        return "barline"
    return "text"

    if "Note" in token:
        return "note"
    elif "Dash" in token:
        return "dash"
    elif "Barline" in token or "LeftRepeatBarline" in token or "RightRepeatBarline" in token:
        return "barline"
    return "text"

def render_tokens(line_text, tokens):
    display = [" " for _ in line_text]
    applied = [False] * len(line_text)

    cursor = 0
    for token in tokens:
        kind = list(token.keys())[0]
        info = token[kind]
        text = info.get("Text", info.get("text", "|"))
        label = classify(token)
        syllable = info.get("Syllable")
        octave = info.get("Octave")

        # Find matching position for text
        try:
            idx = line_text.index(text, cursor)
        except ValueError:
            continue

        # Apply color
        color = COLOR.get(label, COLOR["text"])
        for i, ch in enumerate(text):
            if idx + i < len(display):
                display[idx + i] = f"{color}{ch}{RESET}"
                applied[idx + i] = True
        cursor = idx + len(text)

        # Add octave dot above or below
        if isinstance(octave, int):
            dot = "•"
            if octave == 1 and idx < len(display):
                display[idx] = f"{COLOR['octave_marker']}{dot}{RESET}\n" + display[idx]
            elif octave == -1 and idx < len(display):
                display[idx] = display[idx] + f"\n{COLOR['octave_marker']}{dot}{RESET}"

        # Add syllable below
        if syllable and idx < len(display):
            display[idx] = display[idx] + f"\n{COLOR['lyric']}{syllable}{RESET}"

    return "".join(display)

def main():
    data = yaml.safe_load(sys.stdin)
    doc = data.get("Document", {})
    raw_lines = doc.get("Title", "").splitlines() + [""] * 5
    lines = doc.get("Lines", [])

    input_lines = []
    color_lines = []

    for line_obj in lines:
        line_data = line_obj.get("Line", {})
        label = line_data.get("Label", "")
        text_line = f"{label} "
        for token in line_data.get("Tokens", []):
            for kind in token:
                if kind in ["Note", "Dash", "Barline", "LeftRepeatBarline", "RightRepeatBarline"]:
                    t = token[kind]
                    text_line += t.get("Text", "|")
                else:
                    text_line += " "
            text_line += " "
        input_lines.append(text_line.strip())
        color_lines.append(render_tokens(text_line.strip(), line_data.get("Tokens", [])))

    # Header
    max_orig = max(len(s) for s in input_lines) if input_lines else 0
    max_colored = max(get_display_width(s) for s in color_lines) if color_lines else 0
    print(f"{'Original'.ljust(max_orig)} | {'Colorized'}")
    print(f"{'-' * max_orig}-+-{'-' * max_colored}")

    for o, c in zip(input_lines, color_lines):
        print(f"{o.ljust(max_orig)} | {c}")

    print("\n--- Legend ---")
    for tag in ["note", "dash", "barline", "lyric", "octave_marker"]:
        print(f"{COLOR[tag]}███{RESET} : {tag}")

if __name__ == "__main__":
    main()

