#!/usr/bin/env fish
# Read stdin into a variable
set input_raw (cat)
# echo $input_raw
#echo "1"
# Process the raw input to replace blank lines using 'sd'
# This is a hack because LLMs sometimes have a problem with empty lines
set input_processed (sd -p '^$' 'BLANKLINE' "$input_raw")
exit
#echo "2"
# Get parent directory name to form the specific prompt file name
set parent_dir (basename (pwd))
echo "parent_dir $parent_dir"
set specific_prompt_file "$parent_dir"_prompt.md
set generic_prompt_file "prompt.md"
#echo "3"
set prompt_file_to_use "" # Variable to store the chosen prompt file path

if test -f "$specific_prompt_file"
    set prompt_file_to_use "$specific_prompt_file"
    echo "Found specific prompt file: $prompt_file_to_use"
else if test -f "$generic_prompt_file"
    set prompt_file_to_use "$generic_prompt_file"
    echo "Specific prompt file not found. Using generic prompt file: $prompt_file_to_use"
else
    echo "❌ Error: Neither '$specific_prompt_file' nor '$generic_prompt_file' found in the current directory." >&2
    exit 1
end

# Read prompt template
set prompt_template (cat "$prompt_file_to_use")

# Combine: Replace placeholder
set full_prompt (string replace "[[DOCUMENT]]" "$input_processed" -- "$prompt_template")

# Escape prompt safely into JSON string
set prompt_json (echo "$full_prompt" | jq -Rsa .)

# Construct the JSON payload directly as a string, ensuring correct JSON escaping for content
# Note: Internal double quotes for JSON keys/values must be escaped with a backslash (\")
# The $prompt_json variable, being already a JSON string, is embedded directly without extra quotes.
set payload_content "{
  \"model\": \"gpt-4-turbo\",
  \"messages\": [
    {\"role\": \"system\", \"content\": \"You are a careful document labeler.\"},
    {\"role\": \"user\", \"content\": $prompt_json}
  ],
  \"temperature\": 0
}"

# Send to API by piping the payload directly to curl's stdin using printf
# This is the most robust method for older and newer fish versions, and other shells.
# The `printf %s` ensures the string is printed literally without extra newlines.
printf "%s" "$payload_content" | curl https://api.openai.com/v1/chat/completions \
    -H "Authorization: Bearer $LLM_API_KEY" \
    -H "Content-Type: application/json" \
    --data-binary @-
