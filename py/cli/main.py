from pathlib import Path
import typer
from lib import prompting
import sys
import yaml

def process(
    input_file: Path = typer.Argument(None, help='Path to notation file (or reads from stdin if omitted)'),
    output_file: Path = typer.Option(
        None,
        '--output',
        '-o',
        help='The path to write the final YAML output to. Prints to console by default.'
    ),
):
    # Read from file or stdin
    if input_file:
        content = input_file.read_text()
    else:
        content = sys.stdin.read()

    # Run LLM-powered notation prompt
    result = prompting.run_notation_prompt(content, prompt_name='structure')

    # Output YAML
    result_yaml = yaml.safe_dump(result, sort_keys=False)
    if output_file:
        output_file.write_text(result_yaml)
        print(f'Output written to {output_file}')
    else:
        print(result_yaml)


if __name__ == '__main__':
    typer.run(process)
