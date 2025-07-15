import sys
import yaml
from pathlib import Path
from lib.annotate import add_annotations_to_notation_spans

def main():
    if len(sys.argv) != 3:
        print("Usage: python cli/add_annotations.py source.txt source.yml")
        sys.exit(1)
    src = Path(sys.argv[1]).read_text()
    doc = yaml.safe_load(Path(sys.argv[2]).read_text())
    result = add_annotations_to_notation_spans(doc, src)
    yaml.dump(result, sys.stdout, sort_keys=False, allow_unicode=True)

if __name__ == "__main__":
    main()
