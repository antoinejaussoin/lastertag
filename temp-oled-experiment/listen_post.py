#!/usr/bin/env python3
"""Print HTTP POSTs from the Pico 2 W on this computer's LAN address."""

from __future__ import annotations

import socket
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


class Handler(BaseHTTPRequestHandler):
    def do_POST(self) -> None:
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        print(f"{self.command} {self.path} from {self.client_address[0]}")
        print(body.decode("utf-8", errors="replace"))
        print("---")
        self.send_response(204)
        self.end_headers()

    def log_message(self, format: str, *args: object) -> None:
        return


def lan_ip() -> str:
    probe = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        probe.connect(("1.1.1.1", 80))
        return probe.getsockname()[0]
    except OSError:
        return "127.0.0.1"
    finally:
        probe.close()


if __name__ == "__main__":
    host = lan_ip()
    port = 8090
    print(f"Listening on 0.0.0.0:{port}")
    print(f"On the Pico USB console:")
    print(f"  server {host}:{port}/temp")
    print(f"  save")
    ThreadingHTTPServer(("0.0.0.0", port), Handler).serve_forever()
