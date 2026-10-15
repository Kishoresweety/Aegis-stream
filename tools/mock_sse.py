"""Mock OpenAI-style streaming upstream for Week-1 testing (stdlib only).
Run:  python3 tools/mock_sse.py 8000      then point AEGIS_UPSTREAM at it."""
import json, sys, time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

TEXT = "Sure! My card is 4111-1111-1111-1111 and the key is AKIAIOSFODNN7EXAMPLE."

class H(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    def do_POST(self):
        self.rfile.read(int(self.headers.get("Content-Length", 0)))
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Transfer-Encoding", "chunked")
        self.end_headers()
        def chunk(b):
            self.wfile.write(f"{len(b):x}\r\n".encode() + b + b"\r\n"); self.wfile.flush()
        for i in range(0, len(TEXT), 3):  # 3-byte deltas to mimic tiny token chunks
            d = {"choices": [{"delta": {"content": TEXT[i:i+3]}}]}
            chunk(f"data: {json.dumps(d)}\n\n".encode()); time.sleep(0.01)
        chunk(b"data: [DONE]\n\n"); self.wfile.write(b"0\r\n\r\n")
    def log_message(self, *a): pass

if __name__ == "__main__":
    ThreadingHTTPServer(("127.0.0.1", int(sys.argv[1]) if len(sys.argv) > 1 else 8000), H).serve_forever()
