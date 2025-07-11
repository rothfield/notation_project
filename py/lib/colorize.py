
#!/usr/bin/env python3
import yaml
import sys

# Load YAML from stdin or file argument
if len(sys.argv) > 1:
    with open(sys.argv[1], "r") as f:
        data = yaml.safe_load(f)
else:
    data = yaml.safe_load(sys.stdin)

# Flat colorization (stub - add real coloring logic if needed)
def flat_colorize(data):
    for line in data.get('lines', []):
        text = line.get('text', '')
        print(text)
    print("\n(Flat colorization mode complete)")

# Nesting-aware visualization with ASCII boxes
def visualize_with_boxes(text, annotations, depth=0):
    result = ''
    pad = '  ' * depth
    for ann in annotations:
        tag = ann['tag']
        start = ann['start']
        end = ann['end']
        result += f"{pad}[{tag.upper()} START]\n"
        segment = text[start:end]
        result += f"{pad}  {segment}\n"
        if ann.get('children'):
            result += visualize_with_boxes(text, ann['children'], depth + 1)
        result += f"{pad}[{tag.upper()} END]\n"
    return result

# Check for nesting (use 'text' and 'annotations') or flat token lines ('lines')
if 'annotations' in data and 'text' in data:
    visualization = visualize_with_boxes(data['text'], data['annotations'])
    legend = '''
Legend:
[SECTION START] / [SECTION END]   -> Outer section block
[PHRASE START] / [PHRASE END]     -> Nested phrase inside section
[HIGHLIGHT START] / [HIGHLIGHT END] -> Highlighted sub-segment within phrase
'''
    print(visualization)
    print(legend)
elif 'lines' in data:
    flat_colorize(data)
else:
    print("Unsupported data format. Please provide valid YAML with 'annotations' and 'text', or 'lines'.")
