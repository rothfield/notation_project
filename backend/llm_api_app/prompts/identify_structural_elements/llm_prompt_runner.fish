#!/usr/bin/env fish

set input_file "input.txt"
sed 's/^$/BLANKLINE/' <"$input_file" >"$input_file.processed"

set prompt_file "identify_structural_elements.md"
set output_file "output.yaml"
set payload_file "payload.yaml"

# Check for required files
for file in $input_file.processed $prompt_file
    if not test -f $file
        echo "❌ Missing required file: $file" >&2
        exit 1
    end
end

# Read files
set input_text (cat $input_file.processed)
set prompt_template (cat $prompt_file)

# Combine: Replace placeholder
set full_prompt (string replace "[[DOCUMENT]]" "$input_text" -- $prompt_template)

# Escape prompt safely into JSON string
set prompt_json (echo "$full_prompt" | jq -Rsa .)

# Write JSON payload to file (for debugging and sending)
begin
    echo '{
  "model": "gpt-4-turbo",
  "messages": [
    {"role": "system", "content": "You are a careful document labeler."},
    {"role": "user", "content": ' $prompt_json '}
  ],
  "temperature": 0
}'
end >$payload_file

# Debug: Show payload (optional)
# cat $payload_file | jq .

# Send to API (safe with file payload)
curl https://api.openai.com/v1/chat/completions \
    -H "Authorization: Bearer $LLM_API_KEY" \
    -H "Content-Type: application/json" \
    -d @$payload_file | tee $output_file
