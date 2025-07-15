from pathlib import Path
import typer
import sys
import codecs
import yaml
from lib import prompting

app = typer.Typer(help="Notation CLI")

# Custom YAML representer for multiline strings
def str_presenter(dumper, data):
    if "\n" in data:
        return dumper.represent_scalar("tag:yaml.org,2002:str", data, style="|")
    return dumper.represent_scalar("tag:yaml.org,2002:str", data)

yaml.add_representer(str, str_presenter, Dumper=yaml.SafeDumper)

@app.command()
def run(
    prompt_name: str = typer.Argument(..., help="Prompt to run (e.g. structure, runs)"),
    input_file: Path = typer.Argument(None, help="Input file to read from (or stdin if omitted)"),
    output_file: Path = typer.Option(
        None,
        "--output", "-o",
        help="Optional output YAML file path (prints to stdout by default)"
    ),
):
    """
    Run a single prompt (e.g. structure, runs) with given input.
    """
    if input_file:
        content = input_file.read_text()
    else:
        content = codecs.decode(sys.stdin.read(), "unicode_escape").rstrip("\n")

    result = prompting.run_notation_prompt(content, prompt_name=prompt_name)
    yaml_output = yaml.safe_dump(result, sort_keys=False)

    if output_file:
        output_file.write_text(yaml_output, encoding="utf-8")
        print(f"✅ Output written to {output_file}")
    else:
        print(yaml_output)


if __name__ == "__main__":
    app()

