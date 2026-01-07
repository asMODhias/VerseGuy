#!/usr/bin/env python3
import socket, threading, sys, time

LISTEN_HOST = '127.0.0.1'
LISTEN_PORT = 4318
REMOTE_HOST = '172.18.0.3'
REMOTE_PORT = 4318

def handle(client_sock, client_addr):
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.connect((REMOTE_HOST, REMOTE_PORT))
    client_sock.settimeout(10)
    srv.settimeout(10)
    buf = bytearray()

    def forward(src, dst, capture_from_client=False):
        while True:
            try:
                data = src.recv(4096)
            except Exception:
                break
            if not data:
                break
            if capture_from_client:
                buf.extend(data)
            try:
                dst.sendall(data)
            except Exception:
                break
        try:
            dst.shutdown(socket.SHUT_WR)
        except Exception:
            pass

    t1 = threading.Thread(target=forward, args=(client_sock, srv, True))
    t2 = threading.Thread(target=forward, args=(srv, client_sock, False))
    t1.start(); t2.start()
    t1.join(); t2.join()
    # write captured client->server bytes
    ts = int(time.time())
    fname = f"target/otlp_forward_capture_{ts}.bin"
    with open(fname, 'wb') as f:
        f.write(buf)
    print('Wrote capture bytes to', fname)
    client_sock.close(); srv.close()

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
sock.bind((LISTEN_HOST, LISTEN_PORT))
sock.listen(5)
print(f"Proxy listening on {LISTEN_HOST}:{LISTEN_PORT}, forwarding to {REMOTE_HOST}:{REMOTE_PORT}")
try:
    while True:
        c,a = sock.accept()
        threading.Thread(target=handle, args=(c,a)).start()
except KeyboardInterrupt:
    sock.close()
    print('Exiting')
    sys.exit(0)
