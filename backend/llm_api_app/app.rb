# ####begin require_section
require 'sinatra'
require 'json'
require 'httparty'
# ####end require_section

# ####begin serve_home
# Serve static files from ./public
set :public_folder, 'public'

#begin helpers#
helpers do
  def call_LLM_API(prompt)
    # call the api with the prompt; return result
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
end
#end helpers#

#begin parse_route#
post '/zzzparse' do
  require 'yaml'

  notation = params[:notation] || ""

  # Stage 1: Parse full document
  document = call_LLM_parse_document(notation) # returns YAML as string

  doc_hash = YAML.safe_load(document)
  blocks = doc_hash['blocks'] || []

  # Stage 2: Tokenize each block
  parsed_blocks = blocks.map do |block|
    call_LLM_tokenize_block(block)
  end
  puts "******"
  puts parsed_blocks
  puts parsed_blocks
  puts "******"
  # Pass all pieces to Slim template
  slim :index, locals: {
    notation: notation,
    document: document,
    parsed_blocks: parsed_blocks
  }
end
#end parse_route#

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
  full_LLM_prompt = prompt_template_content.gsub("[Insert Raw Document Content Here]", user_input_notation)
  puts full_LLM_prompt if false
  puts "Calling LLM "
  response = call_LLM_API(full_LLM_prompt)
  parse_results = response.parsed_response
   puts "****LLM_output***"
   puts parse_results
   # Render the slim template with the LLM's response
    result = parse_results["choices"][0]["message"]["content"]  # get YAML strin
    puts
    puts "****yaml of parsed document"
    puts result
    puts
   slim :index, locals: { result: result, notation: user_input_notation }
end
# ####end parse_route####
#

#begin root_index_slim#
get '/' do
  notation = ""
  result = nil
  slim :index, locals: { notation: notation, result: result }
end
#end root_index_slim#

#begin get_parse_document_prompt#
get '/parse_document.md' do
  send_file File.join(settings.root, 'prompts', 'parse_document.md')
end
#end get_parse_document_prompt#

#begin get_reduce_block_prompt#
get '/reduce_block_prompt.md' do
  send_file File.join(settings.root, 'prompts', 'reduce_block_prompt.md')
end
#end get_reduce_block_prompt#

#begin get_tokenize_block_prompt#
get '/tokenize_block.md' do
  send_file File.join(settings.root, 'prompts', 'tokenize_block.md')
end
#end get_tokenize_block_prompt#

