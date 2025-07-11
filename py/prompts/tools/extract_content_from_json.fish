#!/usr/bin/env fish
# Extracts content from OpenAI JSON response safely.
# Usage: cat step1_output.json | ./extract_content_from_json.fish

set tmpfile (mktemp)

# Save stdin to temp file
cat > "$tmpfile"

# Check if it's valid JSON
if not jq empty "$tmpfile" > /dev/null 2>&1
    echo "❌ Error: Input is not valid JSON." >&2
    rm "$tmpfile"
    exit 1
end

# Extract and clean
jq -r '.choices[0].message.content' "$tmpfile" | \
    sed 's/^```json$//; s/^```$//; s/^```json//; s/```$//' | \
    jq . || begin
    echo "❌ Error: Content field missing or malformed." >&2
    rm "$tmpfile"
    exit 1
end

rm "$tmpfile"

