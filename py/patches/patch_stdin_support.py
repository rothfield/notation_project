import sys
from pathlib import Path

def main():
    target = Path("cli/main.py")
    text = target.read_text()

    # Replace argument definition
    text = text.replace(
        "input_file: Path = typer.Argument(..., help=\"Path to notation file (required)\")",
        "input_file: Path = typer.Argument(None, help=\"Path to notation file (or reads from stdin if omitted)\")"
    )

    # Insert stdin fallback logic (only if not already present)
    if "if input_file:" not in text:
        text = text.replace(
            "def process(",
            "def process(\n    # stdin fallback added below\n"
        ).replace(
            "    structure_data = prompting.run_notation_prompt(content, \"structure_parser\")",
            "    if input_file:\n        content = input_file.read_text()\n    else:\n        content = sys.stdin.read()\n\n    structure_data = prompting.run_notation_prompt(content, \"structure_parser\")"
        )

    target.write_text(text)
    print("✅ stdin support patch applied successfully.")

if __name__ == "__main__":
    main()

