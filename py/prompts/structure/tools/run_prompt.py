#!/usr/bin/env python3
import os
import sys
import json
import subprocess
import yaml
import re

def main():
    # 1. Read stdin into a variable
    input_raw = sys.stdin.read()

    # 2. Process the raw input to replace blank lines
    lines = input_raw.splitlines()
    processed_lines = []
    for line in lines:
        if not line.strip():
            processed_lines.append("BLANKLINE")
        else:
            processed_lines.append(line)
    input_processed = "\n".join(processed_lines)

    # 3. Determine prompt file to use
    current_dir = os.getcwd()
    parent_dir_name = os.path.basename(current_dir)
    specific_prompt_file = f"{parent_dir_name}_prompt.md"
    generic_prompt_file = "prompt.md"

    prompt_file_to_use = None

    if os.path.exists(specific_prompt_file):
        prompt_file_to_use = specific_prompt_file
        print(f"Found specific prompt file: {prompt_file_to_use}", file=sys.stderr)
    elif os.path.exists(generic_prompt_file):
        prompt_file_to_use = generic_prompt_file
        print(f"Specific prompt file not found. Using generic prompt file: {prompt_file_to_use}", file=sys.stderr)
    else:
        print(f"❌ Error: Neither '{specific_prompt_file}' nor '{generic_prompt_file}' found in the current directory.", file=sys.stderr)
        sys.exit(1)

    # 4. Read prompt template
    try:
        with open(prompt_file_to_use, 'r', encoding='utf-8') as f:
            prompt_template = f.read()
    except IOError as e:
        print(f"❌ Error reading prompt file '{prompt_file_to_use}': {e}", file=sys.stderr)
        sys.exit(1)

    # 5. Combine: Replace placeholder
    full_prompt = prompt_template.replace("[[DOCUMENT]]", input_processed)

    # 6. Construct the JSON payload
    payload = {
        "model": "gpt-4-turbo",
        "messages": [
            {"role": "system", "content": "You are a careful document labeler."},
            {"role": "user", "content": full_prompt}
        ],
        "temperature": 0
    }

    payload_json = json.dumps(payload)

    # 7. Send to API and capture output
    api_key = os.environ.get("LLM_API_KEY")
    if not api_key:
        print("❌ Error: LLM_API_KEY environment variable not set.", file=sys.stderr)
        sys.exit(1)

    curl_command = [
        "curl",
        "-v", # Verbose output to stderr
        "https://api.openai.com/v1/chat/completions",
        "-H", f"Authorization: Bearer {api_key}",
        "-H", "Content-Type: application/json",
        "--data-binary", "@-"
    ]

    try:
        process = subprocess.run(
            curl_command,
            input=payload_json,
            text=True,
            stdout=subprocess.PIPE,  # <--- IMPORTANT: Capture stdout for JSON parsing
            stderr=sys.stderr,      # <--- IMPORTANT: Direct stderr (verbose progress) to console
            check=True
        )
        api_response_json = process.stdout # This now correctly gets only stdout (the JSON)

        # No need to print process.stderr here, as it's already being streamed live by stderr=sys.stderr

    except subprocess.CalledProcessError as e:
        print(f"❌ Error calling OpenAI API: {e}", file=sys.stderr)
        # e.stderr will still contain stderr if check=True raised it, but usually it's already printed.
        if e.stderr: # In case the error output wasn't fully streamed before crash
            print(f"Stderr from curl during error:\n{e.stderr}", file=sys.stderr)
        sys.exit(1)
    except FileNotFoundError:
        print("❌ Error: 'curl' command not found. Please ensure curl is installed and in your PATH.", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"❌ An unexpected error occurred during curl call: {e}", file=sys.stderr)
        sys.exit(1)

    # 8. Parse JSON response, extract content, strip fences, and convert to YAML
    try:
        response_data = json.loads(api_response_json)

        ai_content_raw = response_data['choices'][0]['message']['content']

        # Strip Markdown code fences from the extracted content if present
        # This regex matches the opening fence (``` optionally with lang) at the start
        # and the closing fence (```) at the end, considering potential whitespace.
        match = re.fullmatch(r'^\s*```(?:yaml)?\s*\n(.*)\n\s*```\s*$', ai_content_raw, re.DOTALL | re.IGNORECASE)
        if match:
            # If fences are found, extract the content within them
            extracted_yaml_str = match.group(1).strip()
        else:
            # If no fences are found, assume the whole content is the YAML string
            extracted_yaml_str = ai_content_raw.strip()

        # Parse this extracted string as YAML
        parsed_yaml_data = yaml.safe_load(extracted_yaml_str)

        # Convert the parsed YAML data back to a clean YAML string for output
        yaml_output_of_content = yaml.dump(parsed_yaml_data, default_flow_style=False, allow_unicode=True)

        # 9. Print YAML content (without fences)
        print(yaml_output_of_content)

    except json.JSONDecodeError as e:
        print(f"❌ Error decoding JSON response from API. Is it valid JSON?: {e}", file=sys.stderr)
        print(f"Raw API response (non-JSON/malformed):\n{api_response_json}", file=sys.stderr)
        sys.exit(1)
    except KeyError as e:
        print(f"❌ Error: Could not find expected key in API response structure: {e}", file=sys.stderr)
        print(f"Full API response received:\n{json.dumps(response_data, indent=2)}", file=sys.stderr)
        sys.exit(1)
    except yaml.YAMLError as e:
        print(f"❌ Error parsing extracted content as YAML: {e}", file=sys.stderr)
        print(f"Content that failed YAML parsing:\n{extracted_yaml_str}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"❌ An unexpected error occurred during YAML conversion or content extraction: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
