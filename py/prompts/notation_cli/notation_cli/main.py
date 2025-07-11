import typer
from pathlib import Path
import yaml
from rich.console import Console
from rich.table import Table
from rich.text import Text

app = typer.Typer()
console = Console()

LABEL_COLORS = {
    "highlight": "yellow",
    "notation": "white",
    "note": "green",
    "barline": "blue",
    "ornament": "magenta",
    "lyric": "yellow",
    "octave_marker": "cyan",
    "text": "white",
    "punctuation": "grey50",
}

@app.command()
def view(
    file_path: Path = typer.Argument(..., exists=True, file_okay=True, dir_okay=False, readable=True, help="Path to the labeled YAML file.")
):
    """
    Displays a side-by-side view of the original source and the colorized, labeled output.
    """
    try:
        with open(file_path, 'r') as f:
            data = yaml.safe_load(f)
    except yaml.YAMLError as e:
        console.print(f"[bold red]Error parsing YAML file:[/] {e}")
        raise typer.Exit(1)

    doc = data.get("document", {})
    if not doc or "src" not in doc or "lines" not in doc:
        console.print("[bold red]Error:[/] YAML file must contain 'document.src' and 'document.lines'.")
        raise typer.Exit(1)

    original_lines = doc.get("src", "").splitlines()
    lines_data = doc.get("lines", [])
    
    colorized_lines_map = {}
    used_labels = set()

    for line_obj in lines_data:
        line_num = line_obj.get("line-number")
        if line_num is None:
            continue

        spans = sorted(line_obj.get("spans", []), key=lambda s: s.get("start_col", 1))
        rich_text = Text()
        cursor = 0
        for span in spans:
            col = span.get("start_col", 1) - 1
            text = span.get("text", "")
            label = span.get("tag", "text")
            
            used_labels.add(label)
            color = LABEL_COLORS.get(label, "white")
            
            if col > cursor:
                rich_text.append(" " * (col - cursor))
            rich_text.append(text, style=color)
            cursor = col + len(text)
        colorized_lines_map[line_num] = rich_text

    table = Table(title="Notation Analysis", show_header=True, header_style="bold magenta")
    table.add_column("Original", justify="left", no_wrap=True)
    table.add_column("Colorized", justify="left")

    num_lines = max(len(original_lines), max(colorized_lines_map.keys()) if colorized_lines_map else 0)
    for i in range(num_lines):
        line_num = i + 1
        original = original_lines[i] if i < len(original_lines) else ""
        colorized = colorized_lines_map.get(line_num, Text(""))
        table.add_row(original, colorized)

    console.print(table)

    if used_labels:
        legend_table = Table(title="Legend", show_header=False, box=None, padding=(0, 1))
        legend_table.add_column()
        legend_table.add_column()
        for label in sorted(list(used_labels)):
            color = LABEL_COLORS.get(label, "white")
            legend_table.add_row(Text("███", style=color), label)
        console.print(legend_table)


if __name__ == "__main__":
    app()
