def add_annotations_to_notation_spans(document, src_text):
    """
    Attaches annotations (highlights) to notation spans based on proximity.
    This function modifies the input 'document' dictionary in place.
    """
    lines = document["document"]["lines"]
    
    # --- Data Collection ---
    # Create a lookup for lines by line number for easier access
    lines_by_number = {line['line-number']: line for line in lines}

    # Collect all notation and annotation spans, adding line number info
    notation_spans = []
    annotation_spans = []
    for line in lines:
        line_num = line['line-number']
        for span in line.get("spans", []):
            span['line_num'] = line_num # Keep track of the line number
            if span["tag"] == "notation":
                notation_spans.append(span)
            elif span["tag"] == "highlight":
                annotation_spans.append(span)

    # --- Attachment Logic ---
    
    # Set to track consumed annotations (using a tuple of line_num and start_col for a unique ID)
    consumed_annotations = set()

    # Helper to get a unique ID for an annotation
    def get_ann_id(ann):
        return (ann['line_num'], ann['start_col'])

    # Helper to determine the layer of an annotation relative to a notation span
    def get_layer(ann_line_num, not_line_num):
        if ann_line_num < not_line_num:
            return 'above'
        elif ann_line_num > not_line_num:
            return 'below'
        return 'same' # Should not happen with adjacent lines

    # --- Phase 1: Exact Column Match ---
    for ann in annotation_spans:
        ann_id = get_ann_id(ann)
        if ann_id in consumed_annotations:
            continue

        for not_span in notation_spans:
            # Check for adjacent lines and exact start_col match
            if abs(ann['line_num'] - not_span['line_num']) == 1 and ann['start_col'] == not_span['start_col']:
                layer = get_layer(ann['line_num'], not_span['line_num'])
                ann['layer'] = layer
                not_span.setdefault('annotations', []).append(ann)
                consumed_annotations.add(ann_id)
                ann['attached'] = True
                break # Annotation consumed, move to the next one
    
    # --- Phase 2: ±1 Column Tolerance ---
    for ann in annotation_spans:
        ann_id = get_ann_id(ann)
        if ann_id in consumed_annotations:
            continue

        for not_span in notation_spans:
            # Check for adjacent lines and ±1 start_col match
            if abs(ann['line_num'] - not_span['line_num']) == 1 and abs(ann['start_col'] - not_span['start_col']) == 1:
                layer = get_layer(ann['line_num'], not_span['line_num'])
                
                # Check if there's already an annotation on this layer
                has_annotation_from_layer = any(
                    a.get('layer') == layer for a in not_span.get('annotations', [])
                )

                if not has_annotation_from_layer:
                    ann['layer'] = layer
                    not_span.setdefault('annotations', []).append(ann)
                    consumed_annotations.add(ann_id)
                    ann['attached'] = True
                    break # Annotation consumed

    # --- Phase 3: Fuzzy Matching (Nearest on adjacent line) ---
    for ann in annotation_spans:
        ann_id = get_ann_id(ann)
        if ann_id in consumed_annotations:
            continue

        best_match = None
        min_dist = float('inf')

        for not_span in notation_spans:
            # Only consider adjacent lines
            if abs(ann['line_num'] - not_span['line_num']) != 1:
                continue

            layer = get_layer(ann['line_num'], not_span['line_num'])
            
            # Check if there's already an annotation on this layer
            has_annotation_from_layer = any(
                a.get('layer') == layer for a in not_span.get('annotations', [])
            )

            if not has_annotation_from_layer:
                dist = abs(ann['start_col'] - not_span['start_col'])
                if dist < min_dist:
                    min_dist = dist
                    best_match = not_span
        
        if best_match:
            layer = get_layer(ann['line_num'], best_match['line_num'])
            ann['layer'] = layer
            best_match.setdefault('annotations', []).append(ann)
            consumed_annotations.add(ann_id)
            ann['attached'] = True

    # --- Final Cleanup ---
    # The original `document` is modified in place.
    # We can remove the temporary 'line_num' key from spans if we want to keep the structure clean.
    for line in lines:
        for span in line.get("spans", []):
            if 'line_num' in span:
                del span['line_num']
            # The 'attached' flag is left on annotations to show which ones were used.

    # The problem description implies modifying the original document.
    # The old code returned a new structure. Let's stick to modifying the original.
    return document


def classify_char(char):
    if char in "|:": return "barline"
    if char in "-_": return "dash"
    if char in "~": return "ornament"
    if char.isdigit(): return "note"
    if char.isalpha(): return "note"
    return "symbol"