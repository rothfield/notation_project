# ####begin require_section
require 'sinatra'
require 'json'
require 'httparty'
# ####end require_section

# ####begin serve_home
# Serve static files from ./public
set :public_folder, 'public'

# Redirect root to index.html
get '/' do
  redirect '/index.html'
end
#end /#


#begin call_llm_api#
def call_llm_api(prompt)
  HTTParty.post("https://api.openai.com/v1/chat/completions",
    headers: {
      "Content-Type" => "application/json",
      "Authorization" => "Bearer #{ENV['LLM_API_KEY']}"
    },
    body: {
      model: "gpt-3.5-turbo",
      messages: [
        { role: "system", content: "You are a helpful assistant." },
        { role: "user", content: prompt }
      ]
    }.to_json
  )
end
#end call_llm_api#

#begin parse_route#
post '/parse' do
  user_input_notation = params[:notation]
  puts user_input_notation
  # Define the path to your document_parser.md prompt
  document_parser_prompt_path = File.join(File.dirname(__FILE__), 'prompts', 'parse_document.md')

  # Read the content of the prompt file
  begin
    prompt_template_content = File.read(document_parser_prompt_path)
  rescue Errno::ENOENT
    status 500
    return "Error: document_parser.md not found at #{document_parser_prompt_path}"
  end

  # Combine the prompt template with the user's notation
  # The prompt template includes a placeholder like "[Insert Raw Document Content Here]"
  # We replace this placeholder with the actual user_input_notation.
  full_llm_prompt = prompt_template_content.gsub("[Insert Raw Document Content Here]", user_input_notation)
  puts full_llm_prompt
  # Call the LLM API with the combined prompt
  response = call_llm_api(full_llm_prompt)
  llm_output = response.parsed_response

  # Render the slim template with the LLM's response
  slim :index, locals: { result: llm_output, notation: user_input_notation }
end
# ####end parse_route####
