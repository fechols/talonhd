#!/usr/bin/env python3
"""Drive a C64 Ultimate for TALON HD: configure, upload the REU image, run the
PRG, and dump the displayed frame for `talonhd compare`.

    python tools/c64u.py config
    python tools/c64u.py upload-reu dist/talonhd.reu
    python tools/c64u.py upload-levels [--dir /Usb0/talonhd]   # the per-level load's image files (FTP)
    python tools/c64u.py run dist/talonhd.prg
    python tools/c64u.py dump dist/dump [--hold]
    python tools/c64u.py status

Host from --host or C64U_HOST; password (if any) from C64U_PASSWORD.
REST + DMA-socket code adapted from impact/tools/c64u_device.py (MIT).
"""
import argparse
import ftplib
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
        # The sampler the music plays on (src/audio.rs).
        "Map Ultimate Audio $DF20-DFFF": "Enabled",
        # The per-level load (src/uci.rs): the PRG reads the next level's image file into the REU.
        "Command Interface": "Enabled",
    },
    # As ../sueme64's validated mix: the sampler +6 dB, its voices panned apart.
    "Audio Mixer": {
        "Vol Sampler L": "+6 dB",
        "Vol Sampler R": "+6 dB",
        "Pan Sampler L": "Left 3",
        "Pan Sampler R": "Right 3",
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
            # The first chunk (the image header, its magic) goes last: a booting PRG must never see a
            # header before the rest has arrived (it would find the image incomplete and erase the magic).
            offsets = list(range(0, len(data), 32768))
            for offset in offsets[1:] + offsets[:1]:
                chunk = data[offset:offset + 32768]
                payload = (address + offset).to_bytes(3, 'little') + chunk
                sock.sendall(struct.pack('<HH', 0xff07, len(payload)) + payload)
            sock.sendall(struct.pack('<HH', 0xff0e, 0))
            receive(sock, receive(sock, 1)[0])


# The level images, in level order (`uci::LEVEL_FILES`), and the device directory the PRG loads them from
# (`uci::LEVEL_DIR`: `TALONHD_DIR` at build time).
LEVEL_FILES = ['talonhd.reu', 'talonhd_s2.reu']
LEVEL_DIR = os.environ.get('TALONHD_DIR', '/Usb0/talonhd')


def upload_levels(host, folder, files):
    """Put the level images in `folder` on the device over its FTP server, creating the folder."""
    with ftplib.FTP(host, timeout=60) as ftp:
        ftp.login('anonymous', os.environ.get('C64U_PASSWORD', ''))
        ftp.cwd('/')
        for part in [p for p in folder.split('/') if p]:
            try:
                ftp.cwd(part)
            except ftplib.error_perm:
                ftp.mkd(part)
                ftp.cwd(part)
        for f in files:
            t0 = time.time()
            with open(f, 'rb') as fh:
                ftp.storbinary(f'STOR {Path(f).name}', fh, blocksize=65536)
            size = ftp.size(Path(f).name)
            if size != Path(f).stat().st_size:
                raise RuntimeError(f'{f}: {size} bytes on the device, {Path(f).stat().st_size} here')
            print(f'{f} -> {folder}/{Path(f).name}: {size} bytes in {time.time() - t0:.1f}s')


def status(dev):
    st = dev.read(STATUS, STATUS_LEN)
    return dict(magic=st[0], flipping=st[1], front=st[2],
                shown_t=st[4] | st[5] << 8, render_us=st[6] | st[7] << 8, hold=st[8],
                background_us=st[10] | st[11] << 8,
                audio_us=st[12] | st[13] << 8, step_us=st[14] | st[15] << 8,
                profile_setup_columns_kernel=[st[i] | st[i + 1] << 8 for i in range(16, 22, 2)],
                human=st[28]), st


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--host', default=os.environ.get('C64U_HOST'))
    sub = ap.add_subparsers(dest='cmd', required=True)
    sub.add_parser('config')
    p = sub.add_parser('upload-reu'); p.add_argument('file')
    p = sub.add_parser('upload-levels'); p.add_argument('--dir', default=LEVEL_DIR)
    p.add_argument('--from', dest='src', default='dist')
    p = sub.add_parser('run'); p.add_argument('prg')
    p = sub.add_parser('dump'); p.add_argument('out'); p.add_argument('--hold', action='store_true')
    p = sub.add_parser('hold'); p.add_argument('value', type=int)
    sub.add_parser('status')
    sub.add_parser('bench')
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
    elif args.cmd == 'upload-levels':
        upload_levels(dev.host, args.dir, [str(Path(args.src) / f) for f in LEVEL_FILES])
    elif args.cmd == 'run':
        # Clear the previous image's magic: the PRG starts on an image already in the REU when it is
        # complete and stamped for it, and `upload-reu` follows `run` here.
        dev.write_reu(bytes(8))
        dev.run_prg(args.prg)
        print('running', args.prg)
    elif args.cmd == 'hold':
        dev.write(HOLD, bytes([args.value]))
    elif args.cmd == 'bench':
        # The `bench` build's results (target-c64 main.rs `bench`): u16 us per test at $CA60.
        raw = dev.read(0xCA60, 31)
        if raw[30] != 0xBE:
            raise SystemExit('bench not finished (run the bench build first)')
        us = [raw[2 * i] | raw[2 * i + 1] << 8 for i in range(15)]
        empty = us[0] / 2048
        print(f'loop overhead: {empty * 1000:.0f} ns per iteration')
        for k, n in enumerate([1, 2, 4, 8, 16, 64, 256, 512]):
            per = us[1 + k] / 256 - empty
            print(f'REU fetch {n:3} B (8 regs + command): {per:7.2f} us  ({per / n * 1000:6.0f} ns/B)')
        names = ['chained 8-B link (4 regs)', '8 I/O writes', '8 RAM stores', '8 byte copies', '8 blends', '8 line lookups']
        for k, nm in enumerate(names):
            per = us[9 + k] / 2048 - empty
            print(f'{nm:27}: {per:7.3f} us')
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
