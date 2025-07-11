import typer
import yaml
from pathlib import Path
from lib import prompting

app = typer.Typer()

@app.command()
def process(
    # stdin fallback added below

    # stdin fallback added below

    input_file: Path = typer.Argument(None, help="Path to notation file (or reads from stdin if omitted)")
        ...,
        exists=True,
        file_okay=True,
        dir_okay=False,
        readable=True,
        help="The path to the input notation file."
    ),
    output_file: Path = typer.Option(
        None,
        "--output",
        "-o",
        help="The path to write the final YAML output to. Prints to console by default."
    )
):
    """
    Processes a musical notation document through a two-step pipeline:
    1. Parse the document structure.
    2. Parse the musical runs from the structured output.
    """
    typer.echo(f"Starting processing for: {input_file}")
    
    initial_content = input_file.read_text()
    
    # --- Step 1: Structure Parsing ---
    typer.echo("Step 1: Parsing structure...")
    structure_data = prompting.run_notation_prompt(
        content=initial_content, 
        prompt_name="structure_parser"
    )
    typer.echo("Structure parsing complete.")
    
    # --- Step 2: Runs Parsing ---
    # Convert the structured data from Step 1 back into a YAML string for the next prompt
    structure_yaml = yaml.dump(structure_data)
    
    typer.echo("Step 2: Parsing runs...")
    runs_data = prompting.run_notation_prompt(
        content=structure_yaml, 
        prompt_name="runs_parser"
    )
    typer.echo("Runs parsing complete.")
    
    # --- Final Output ---
    final_yaml = yaml.dump(runs_data, default_flow_style=False, allow_unicode=True)
    
    if output_file:
        typer.echo(f"Writing final output to: {output_file}")
        output_file.write_text(final_yaml)
    else:
        typer.echo("\n--- Final Output ---")
        typer.echo(final_yaml)

if __name__ == "__main__":
    app()