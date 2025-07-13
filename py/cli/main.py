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

    # Run LLM-powered notation prompt
    result = prompting.run_notation_prompt(content, prompt_name='structure')
    result['document']['src']= content
    # print(result)
    result_yaml = yaml.safe_dump(result, sort_keys=False)
    if output_file:
        output_file.write_text(result_yaml)
        print(f'Output written to {output_file}')
    else:
        print(result_yaml)


if __name__ == '__main__':
    typer.run(process)
