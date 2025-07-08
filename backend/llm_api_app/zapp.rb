require 'sinatra'
require 'slim'
require 'sinatra/reloader' if development?

#begin root_index_slim#
get '/' do
  notation = ""
  result = nil
  slim :index, locals: { notation: notation, result: result }
end
#end root_index_slim#

#begin get_index_html#
get '/index.html' do
  notation = ""
  result = nil
  slim :index, locals: { notation: notation, result: result }
end
#end get_index_html#

#begin parse_route#
post '/parse' do
  notation = params[:notation] || ""
  # Replace the following line with your LLM/parsing logic
  result = "Result goes here"
  slim :index, locals: { notation: notation, result: result }
end
#end parse_route#

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

#begin static_file_fallback#
# Serve static files from /public (Sinatra does this by default)
set :public_folder, File.dirname(__FILE__) + '/public'
#end static_file_fallback#

