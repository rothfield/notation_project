require 'sinatra'
require 'json'
require 'httparty'

LLM_API_URL = "https://api.your-llm-provider.com/v1/generate"
LLM_API_KEY = ENV['LLM_API_KEY']

def load_prompt(filename)
  File.read(File.join('prompts', filename))
end

def call_llm_api(prompt)
  HTTParty.post(LLM_API_URL,
    headers: {
      "Content-Type" => "application/json",
      "Authorization" => "Bearer #{LLM_API_KEY}"
    },
    body: { prompt: prompt, max_tokens: 1000 }.to_json
  )
end

post '/generate' do
  content_type :json
  request_payload = JSON.parse(request.body.read)
  prompt_file = request_payload["prompt_file"]
  prompt = load_prompt(prompt_file)
  response = call_llm_api(prompt)
  { result: response.parsed_response }.to_json
end
