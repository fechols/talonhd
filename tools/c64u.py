#!/usr/bin/env python3
"""Drive a C64 Ultimate for TALON HD: configure, upload the REU image, run the
PRG, and dump the displayed frame for `talonhd compare`.

    python tools/c64u.py config
    python tools/c64u.py upload-reu dist/talonhd.reu
    python tools/c64u.py run dist/talonhd.prg
    python tools/c64u.py dump dist/dump [--hold]
    python tools/c64u.py status

Host from --host or C64U_HOST; password (if any) from C64U_PASSWORD.
REST + DMA-socket code adapted from impact/tools/c64u_device.py (MIT).
"""
import argparse
import json
import os
from pathlib import Path
import socket
import struct
import time
import urllib.parse
import urllib.request

STATUS = 0xCA80
STATUS_LEN = 32
HOLD = STATUS + 8
BUFFERS = [dict(screen=0x4C00, bitmap=0x6000), dict(screen=0x8C00, bitmap=0xA000)]

CONFIG = {
    "U64 Specific Settings": {
        "Turbo Control": "C64U Turbo Registers",
        "CPU Speed": "64",
        "Badline Timing": "Disabled",
    },
    "C64 and Cartridge Settings": {
        "RAM Expansion Unit": "Enabled",
        "REU Size": "16 MB",
    },
}


class Device:
    def __init__(self, host, timeout=20):
        self.base = host.rstrip('/') if '://' in host else 'http://' + host
        self.host = urllib.parse.urlsplit(self.base).hostname
        self.timeout = timeout

    def request(self, method, path, body=None, binary=False, **query):
        url = self.base + path
        if query:
            url += '?' + urllib.parse.urlencode(query)
        headers = {}
        if os.environ.get('C64U_PASSWORD'):
            headers['X-Password'] = os.environ['C64U_PASSWORD']
        if isinstance(body, dict):
            body = json.dumps(body).encode()
            headers['Content-Type'] = 'application/json'
        elif body is not None:
            headers['Content-Type'] = 'application/octet-stream'
        req = urllib.request.Request(url, method=method, data=body, headers=headers)
        with urllib.request.urlopen(req, timeout=self.timeout) as response:
            payload = response.read()
        if binary:
            return payload
        result = json.loads(payload) if payload else {}
        if result.get('errors'):
            raise RuntimeError(result['errors'])
        return result

    def read(self, address, length):
        data = self.request('GET', '/v1/machine:readmem', binary=True,
                            address=f'{address:04x}', length=length)
        if len(data) != length:
            raise RuntimeError(f'short read at ${address:04x}: {len(data)} of {length}')
        return data

    def write(self, address, data):
        self.request('PUT', '/v1/machine:writemem', address=f'{address:04x}', data=data.hex())

    def pause(self):
        self.request('PUT', '/v1/machine:pause')

    def resume(self):
        self.request('PUT', '/v1/machine:resume')

    def run_prg(self, path):
        self.request('POST', '/v1/runners:run_prg', Path(path).read_bytes())

    def write_reu(self, data, address=0):
        """REUWRITE (FF07) over the DMA socket, then IDENTIFY as a barrier."""
        def receive(sock, size):
            out = bytearray()
            while len(out) < size:
                block = sock.recv(size - len(out))
                if not block:
                    raise RuntimeError('DMA socket closed early')
                out.extend(block)
            return bytes(out)
        with socket.create_connection((self.host, 64), timeout=self.timeout) as sock:
            password = os.environ.get('C64U_PASSWORD')
            if password:
                secret = password.encode()
                sock.sendall(struct.pack('<HH', 0xff1f, len(secret)) + secret)
                if receive(sock, 1) != b'\x01':
                    raise RuntimeError('DMA authentication failed')
            for offset in range(0, len(data), 32768):
                chunk = data[offset:offset + 32768]
                payload = (address + offset).to_bytes(3, 'little') + chunk
                sock.sendall(struct.pack('<HH', 0xff07, len(payload)) + payload)
            sock.sendall(struct.pack('<HH', 0xff0e, 0))
            receive(sock, receive(sock, 1)[0])


def status(dev):
    st = dev.read(STATUS, STATUS_LEN)
    return dict(magic=st[0], flipping=st[1], front=st[2],
                shown_t=st[4] | st[5] << 8, render_us=st[6] | st[7] << 8, hold=st[8],
                background_us=st[10] | st[11] << 8,
                profile_setup_columns_kernel=[st[i] | st[i + 1] << 8 for i in range(16, 22, 2)],
                human=st[28]), st


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--host', default=os.environ.get('C64U_HOST'))
    sub = ap.add_subparsers(dest='cmd', required=True)
    sub.add_parser('config')
    p = sub.add_parser('upload-reu'); p.add_argument('file')
    p = sub.add_parser('run'); p.add_argument('prg')
    p = sub.add_parser('dump'); p.add_argument('out'); p.add_argument('--hold', action='store_true')
    p = sub.add_parser('hold'); p.add_argument('value', type=int)
    sub.add_parser('status')
    args = ap.parse_args()
    if not args.host:
        raise SystemExit('set --host or C64U_HOST')
    dev = Device(args.host)

    if args.cmd == 'config':
        print(dev.request('POST', '/v1/configs', CONFIG))
    elif args.cmd == 'upload-reu':
        data = Path(args.file).read_bytes()
        t0 = time.time()
        dev.write_reu(data)
        print(f'uploaded {len(data)} bytes to REU 0 in {time.time() - t0:.1f}s')
    elif args.cmd == 'run':
        dev.run_prg(args.prg)
        print('running', args.prg)
    elif args.cmd == 'hold':
        dev.write(HOLD, bytes([args.value]))
    elif args.cmd == 'status':
        print(status(dev)[0])
    elif args.cmd == 'dump':
        out = Path(args.out)
        out.mkdir(parents=True, exist_ok=True)
        if args.hold:
            dev.write(HOLD, b'\x01')
            time.sleep(0.5)
        for attempt in range(10):
            dev.pause()
            try:
                info, raw = status(dev)
                if info['magic'] != 0xC1 or info['flipping']:
                    continue
                buf = BUFFERS[info['front'] & 1]
                (out / 'status.bin').write_bytes(raw)
                (out / 'bitmap.bin').write_bytes(dev.read(buf['bitmap'], 8000))
                (out / 'screen.bin').write_bytes(dev.read(buf['screen'], 1000))
                (out / 'color.bin').write_bytes(dev.read(0xD800, 1000))
                print(info)
                break
            finally:
                dev.resume()
        else:
            raise SystemExit('no consistent frame captured')


if __name__ == '__main__':
    main()
