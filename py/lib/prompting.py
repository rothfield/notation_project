import os
import json
import yaml
import re
import requests
from pathlib import Path

# This is the key change: Find the project root relative to this file.
# This makes the prompt loading robust.
PROMPTS_DIR = Path(__file__).parent.parent.parent / "prompts"

def run_notation_prompt(content: str, prompt_name: str) -> dict:
    """
    Takes notation content, runs it through a specific prompt, 
    and returns the parsed YAML as a Python dictionary.
    """
    # 1. Process the raw input to handle blank lines
    lines = content.splitlines()
    processed_lines = [line if line.strip() else "BLANKLINE" for line in lines]
    input_processed = "\n".join(processed_lines)

    # 2. Find and load the specified prompt file
    prompt_file = PROMPTS_DIR / f"{prompt_name}.md"
    if not prompt_file.is_file():
        raise FileNotFoundError(f"Prompt file not found: {prompt_file}")

    with open(prompt_file, 'r', encoding='utf-8') as f:
        prompt_template = f.read()

    # 3. Construct the prompt and payload
    # The placeholder is now [[INPUT]] to match the actual prompts
    full_prompt = prompt_template.replace("[[INPUT]]", input_processed)
    payload = {
        "model": "gpt-4-turbo",
        "messages": [
            {"role": "system", "content": "You are a world-class music notation parser."},
            {"role": "user", "content": full_prompt}
        ],
        "temperature": 0
    }

    # 4. Send to API using 'requests' library
    api_key = os.environ.get("LLM_API_KEY")
    if not api_key:
        raise ValueError("LLM_API_KEY environment variable not set.")

    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    }
    
    try:
        response = requests.post(
            "https://api.openai.com/v1/chat/completions",
            headers=headers,
            json=payload,
            timeout=60
        )
        response.raise_for_status()
        response_data = response.json()

    except requests.exceptions.RequestException as e:
        raise ConnectionError(f"API request failed: {e}") from e

    # 5. Parse response and return a Python object
    try:
        ai_content_raw = response_data['choices'][0]['message']['content']
        match = re.fullmatch(r'^\s*```(?:yaml)?\s*\n(.*)\n\s*```\s*$', ai_content_raw, re.DOTALL | re.IGNORECASE)
        extracted_yaml_str = match.group(1).strip() if match else ai_content_raw.strip()
        
        parsed_yaml = yaml.safe_load(extracted_yaml_str)
        if not isinstance(parsed_yaml, dict):
            raise TypeError("LLM output was valid YAML, but not a dictionary as expected.")
            
        return parsed_yaml

    except (KeyError, IndexError) as e:
        raise ValueError(f"Invalid API response structure: {e}") from e
    except yaml.YAMLError as e:
        raise ValueError(f"Failed to parse LLM response as YAML: {e}") from e
