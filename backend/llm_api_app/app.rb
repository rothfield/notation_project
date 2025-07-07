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
post "/parse" do
  notation = params[:notation]
  response = call_llm_api(notation)
  llm_output = response.parsed_response

  slim :index, locals: { result: llm_output, notation: notation }
end
#end parse_route#
post "/parse" do
  notation = params[:notation]
  response = call_llm_api(notation)
  llm_output = response.parsed_response

  slim :index, locals: { result: llm_output, notation: notation }
end
#end parse_route#
post "/parse" do
  notation = params[:notation]
  response = call_llm_api(notation)
  llm_output = response.parsed_response

  slim :index, locals: { result: llm_output, notation: notation }
end
#end parse_route#
post "/parse" do
  notation = params[:notation]
  response = call_llm_api(notation)
  llm_output = response.parsed_response

  slim :index, locals: { result: llm_output, notation: notation }
end
#end parse_route#
post "/parse" do
  notation = params[:notation]
  response = call_llm_api(notation)
  llm_output = response.parsed_response

  slim :index, locals: { result: llm_output, notation: notation }
end
#end parse_route#
post '/parse' do
  notation = params[:notation]

  # Call the LLM API (stub or real)
  response = call_llm_api(notation)

  "Received notation:<pre>#{Rack::Utils.escape_html(notation)}</pre>\n\nLLM Response:\n<pre>#{response.body}</pre>"
end
# ####end parse_route####

