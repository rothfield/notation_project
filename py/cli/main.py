from pathlib import Path
import typer
from lib import prompting
import sys
import yaml
import codecs

# Custom representer for multi-line strings to use literal block style for readability.
def str_presenter(dumper, data):
    if '\n' in data:
        return dumper.represent_scalar('tag:yaml.org,2002:str', data, style='|')
    return dumper.represent_scalar('tag:yaml.org,2002:str', data)

yaml.add_representer(str, str_presenter, Dumper=yaml.SafeDumper)


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
        # Unescape backslash sequences like \n into actual newlines.
        # This allows users to pipe in strings with newlines from the command line,
        # e.g., `echo "line1\nline2"`.
        content = codecs.decode(content, 'unicode_escape')
        # remove trailing newline which might have been from the unescaped sequence
        # or from the shell command itself (e.g. echo)
        if content.endswith('\n'):
            content = content[:-1]

    # --- Step 1: Run the 'structure' prompt to label notation and highlights ---
    structure_result = prompting.run_notation_prompt(content, prompt_name='structure')
    structure_result['document']['src'] = content

    # --- Step 2: Pass the result to the 'runs' prompt for finer-grained parsing ---
    # Convert the structured data back to a YAML string to serve as input for the next prompt.
    runs_input_yaml = yaml.safe_dump(structure_result, sort_keys=False)
    runs_result = prompting.run_notation_prompt(runs_input_yaml, prompt_name='runs')

    # The 'runs' prompt is expected to return a final dictionary.
    # We'll dump this to YAML for the final output.
    final_yaml = yaml.safe_dump(runs_result, sort_keys=False)

    if output_file:
        output_file.write_text(final_yaml)
        print(f'Output written to {output_file}')
    else:
        print(final_yaml)


if __name__ == '__main__':
    typer.run(process)
