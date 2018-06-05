import os
import http.server
from http.server import HTTPServer, BaseHTTPRequestHandler
import socketserver

PORT = 9000

Handler = http.server.SimpleHTTPRequestHandler

Handler.extensions_map={
  '.wasm': 'application/wasm',
  '.html': 'text/html',
  '': 'application/octet-stream'
}

os.chdir("public")

httpd = socketserver.TCPServer(("", PORT), Handler)

print("serving at port", PORT)
httpd.serve_forever()
