"""Log a `-Features timing` build's frame costs per tick from the C64U status block (layout::status), for
summaries and `examples/frame_census.rs`; or summarise a log per window of ticks.

    python tools/timing_log.py log SECONDS OUT.csv
    python tools/timing_log.py summary LOG.csv [WINDOW]
"""
import csv
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c64u  # noqa: E402

# name: status offset (u16)
FIELDS = dict(render=6, bg=10, audio=12, step=14, window=16, bands=18, sprites=20, wait=22, irq=24,
              ace=26, hud=29)


def log(seconds, out):
    d = c64u.Device(os.environ['C64U_HOST'])
    end = time.time() + seconds
    seen = set()
    with open(out, 'w') as f:
        f.write('t,' + ','.join(FIELDS) + ',wall\n')
        while time.time() < end:
            try:
                st = d.read(c64u.STATUS, 32)
            except Exception as e:  # a dropped request: try again
                print('read failed', e)
                continue
            t = st[4] | st[5] << 8
            if t in seen:
                continue
            seen.add(t)
            vals = [st[o] | st[o + 1] << 8 for o in FIELDS.values()]
            f.write(f'{t},' + ','.join(map(str, vals)) + f',{time.time():.3f}\n')
            f.flush()
    print(len(seen), 'samples')


def summary(path, window=300):
    rows = [{k: float(v) for k, v in r.items()} for r in csv.DictReader(open(path))]
    keys = [k for k in FIELDS if k in rows[0]]
    print('ticks         fps ' + ' '.join(f'{k:>6}' for k in keys) + '  (ms, means)')
    i = 0
    while i < len(rows) - 1:
        w = [r for r in rows[i:] if r['t'] < rows[i]['t'] + window]
        if len(w) < 2:
            break
        dt = w[-1]['wall'] - w[0]['wall']
        fps = (w[-1]['t'] - w[0]['t']) / dt if dt else 0
        means = [sum(r[k] for r in w) / len(w) / 1000 for k in keys]
        if 'irq' in keys:
            # A free-running u16: its microseconds per frame of wall time (60 Hz).
            spent = sum((b['irq'] - a['irq']) % 65536 for a, b in zip(w, w[1:]))
            means[keys.index('irq')] = spent / dt / 60 / 1000 if dt else 0
        print(f"{int(w[0]['t']):5d}-{int(w[-1]['t']):5d} {fps:5.1f} " + ' '.join(f'{m:6.2f}' for m in means))
        i += len(w)


if __name__ == '__main__':
    if sys.argv[1] == 'log':
        log(float(sys.argv[2]), sys.argv[3])
    else:
        summary(sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 300)
