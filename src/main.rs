//! TALON HD host tool.
//!
//! ```text
//! talonhd bake    [--out DIR]              tech-scene level image + previews, self-checked
//! talonhd preview [--reu FILE] [--t N] [--out PNG]
//! talonhd compare [--reu FILE] --dump DIR [--t N]
//! talonhd bake-model <file.mesh> [--palette v,d,l,h] [--out PNG]   orientation sheet + light share
//! ```

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use talonhd::bake::{self, Panorama};
use talonhd::encode::SpriteImage;
use talonhd::layout::{self, BUFFERS, status};
use talonhd::model::{self, Machine};
use talonhd::render::{self, Scroll};
use talonhd::{demo, mesh, models, png, reference};

struct Args(Vec<String>);

impl Args {
    fn opt(&self, name: &str) -> Option<&str> {
        self.0.iter().position(|a| a == name).and_then(|i| self.0.get(i + 1)).map(|s| s.as_str())
    }
    fn path(&self, name: &str, default: &str) -> PathBuf {
        PathBuf::from(self.opt(name).unwrap_or(default))
    }
    fn num(&self, name: &str, default: u16) -> Result<u16, String> {
        self.opt(name).map_or(Ok(default), |v| v.parse().map_err(|_| format!("{name}: not a number: {v}")))
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = argv.first().cloned() else {
        eprintln!("usage: talonhd bake|preview|compare [options]");
        return ExitCode::FAILURE;
    };
    let args = Args(argv[1..].to_vec());
    let result = match cmd.as_str() {
        "bake" => cmd_bake(&args),
        "preview" => cmd_preview(&args),
        "compare" => cmd_compare(&args),
        "bake-model" => cmd_bake_model(&argv[1..]),
        other => Err(format!("unknown command {other}")),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn write(path: &Path, data: &[u8]) -> Result<(), String> {
    std::fs::write(path, data).map_err(|e| format!("{}: {e}", path.display()))
}

fn load_image(path: &Path) -> Result<(Machine, render::Image), String> {
    let data = read(path)?;
    let mut m = Machine::new(&data);
    let img = render::read_image(&mut m).ok_or("REU image has no TALONHD1 header")?;
    Ok((m, img))
}

/// A contact sheet of frames side by side (multicolor pixels).
fn sheet(frames: &[SpriteImage], pal: render::Palette) -> (usize, Vec<[u8; 3]>) {
    let (w, h) = (frames[0].w as usize, frames[0].h as usize);
    let n = frames.len();
    let mut out = vec![[0u8; 3]; w * n * h];
    for (k, f) in frames.iter().enumerate() {
        let p = bake::sprite_preview(f, pal);
        for y in 0..h {
            out[y * w * n + k * w..y * w * n + k * w + w].copy_from_slice(&p[y * w..y * w + w]);
        }
    }
    (w * n, out)
}

fn cmd_bake(args: &Args) -> Result<(), String> {
    let out = args.path("--out", "dist");
    let previews = PathBuf::from("output");
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;

    let pano: Panorama = models::demo_panorama();
    let frames = models::demo_frames_with_assets();
    let pal = pano.palette;
    let groups = [
        ("boss", demo::BOSS_FRAME, demo::BOSS_BANKS),
        ("drone", demo::DRONE_FRAME, demo::DRONE_YAWS),
        ("player", demo::PLAYER_FRAME, demo::PLAYER_BANKS),
    ];
    let mut total = pano.rows as usize * 320;
    println!("panorama: {} cell rows, {} bytes", pano.rows, total);
    for (name, first, count) in groups {
        let set = &frames[first as usize..(first + count) as usize];
        let bytes: usize = set.iter().map(bake::frame_bytes).sum();
        total += bytes;
        println!("{name:>7}: {count:3} frames {}x{} x{} phases: {bytes:>9} bytes", set[0].w, set[0].h, set[0].phases);
        let (w, px) = sheet(set, pal);
        png::save_wide(&previews.join(format!("sheets/{name}.png")), w, &px)?;
    }
    let image = bake::build_image(&pano, &frames, 0)?;
    println!("level image: {} bytes of 16 MiB ({:.1} %), art ~{total} bytes", image.len(), image.len() as f64 * 100.0 / 16777216.0);
    write(&out.join("talonhd.reu"), &image)?;
    write(&out.join("talonhd.cfg"), include_bytes!("../config/talonhd.cfg"))?;

    // Self-check: the DMA renderer must match the naive compositor.
    let mut m = Machine::new(&image);
    let img = render::read_image(&mut m).unwrap();
    for t in [0u16, 1, 2, 3, 77, 150, 299, 512, 777, 1001, 1500] {
        let buf = BUFFERS[(t & 1) as usize];
        let scroll = model::render_demo(&mut m, &img, t, buf);
        let mut sc = [render::Sprite::default(); demo::INSTANCES];
        let n = demo::scene(t, &mut sc);
        let want = reference::compose(&pano, &frames, scroll, &sc[..n]);
        check(&m, buf, &want).map_err(|e| format!("t={t}: {e}"))?;
        png::save_wide(&previews.join(format!("frames/t{t:04}.png")), 160, &m.visible(buf, img.palette.d021, scroll.fine))?;
    }
    println!("model matches reference compositor; stats {:?}", m.stats);
    println!("wrote {} and previews in {}", out.display(), previews.display());
    Ok(())
}

fn check(m: &Machine, buf: layout::Buffer, want: &reference::Expected) -> Result<(), String> {
    let codes = m.codes(buf);
    if let Some(i) = (0..codes.len()).find(|&i| codes[i] != want.codes[i]) {
        return Err(format!("pixel ({}, {}) code {} want {}", i % 160, i / 160, codes[i], want.codes[i]));
    }
    for i in 0..1000 {
        let s = m.ram[(buf.screen + i as u16) as usize];
        if s != want.screen[i] {
            return Err(format!("screen cell {i}: {s:02x} want {:02x}", want.screen[i]));
        }
        let c = m.color_ram[i] & 15;
        if c != want.color[i] {
            return Err(format!("color cell {i}: {c} want {}", want.color[i]));
        }
    }
    if m.stats.bank_wraps != 0 || m.stats.stray_writes != 0 {
        return Err(format!("bad DMA: {:?}", m.stats));
    }
    Ok(())
}

fn cmd_preview(args: &Args) -> Result<(), String> {
    let (mut m, img) = load_image(&args.path("--reu", "dist/talonhd.reu"))?;
    let t = args.num("--t", 0)?;
    let out = args.path("--out", "output/preview.png");
    let scroll = model::render_demo(&mut m, &img, t, BUFFERS[0]);
    png::save_wide(&out, 160, &m.visible(BUFFERS[0], img.palette.d021, scroll.fine))?;
    println!("t={t}: {:?}", m.stats);
    Ok(())
}

/// Compare a memory dump of a paused C64U (see `tools/c64u.py dump`) with the
/// model's rendering of the frame it reports as displayed.
fn cmd_compare(args: &Args) -> Result<(), String> {
    let (mut m, img) = load_image(&args.path("--reu", "dist/talonhd.reu"))?;
    let dump = PathBuf::from(args.opt("--dump").ok_or("--dump DIR required")?);
    let st = read(&dump.join("status.bin"))?;
    if st[status::MAGIC as usize] != layout::STATUS_MAGIC {
        return Err("status block magic missing: is the demo running?".into());
    }
    if st[status::FLIPPING as usize] != 0 {
        return Err("captured mid-flip; dump again".into());
    }
    let front = BUFFERS[st[status::FRONT as usize] as usize & 1];
    let shown = st[status::SHOWN_T as usize] as u16 | (st[status::SHOWN_T as usize + 1] as u16) << 8;
    let t = args.num("--t", shown)?;
    let scroll: Scroll = model::render_demo(&mut m, &img, t, front);

    let bitmap = read(&dump.join("bitmap.bin"))?;
    let screen = read(&dump.join("screen.bin"))?;
    let color = read(&dump.join("color.bin"))?;
    let mut bad = 0;
    let mut cells = [b'.'; 1000];
    for (i, &b) in bitmap.iter().enumerate() {
        let want = m.ram[front.bitmap as usize + i];
        if b != want {
            if bad < 10 {
                println!("bitmap +{i:04x}: {b:02x} want {want:02x}");
            }
            cells[i / 8] = b'#';
            bad += 1;
        }
    }
    for row in cells.chunks(40) {
        println!("{}", String::from_utf8_lossy(row));
    }
    let mut dev = Machine::new(&[]);
    dev.ram[front.bitmap as usize..front.bitmap as usize + 8000].copy_from_slice(&bitmap);
    dev.ram[front.screen as usize..front.screen as usize + 1000].copy_from_slice(&screen);
    for (i, &c) in color.iter().enumerate() {
        dev.color_ram[i] = c & 15;
    }
    png::save_wide(&dump.join("device.png"), 160, &dev.visible(front, img.palette.d021, scroll.fine))?;
    for (i, &b) in screen.iter().enumerate() {
        if b != m.ram[front.screen as usize + i] {
            bad += 1;
        }
    }
    for (i, &b) in color.iter().enumerate() {
        if b & 15 != m.color_ram[i] {
            bad += 1;
        }
    }
    let micros = st[status::RENDER_US as usize] as u16 | (st[status::RENDER_US as usize + 1] as u16) << 8;
    println!("t={t} front={} render={micros} us: {bad} mismatched bytes", st[status::FRONT as usize]);
    png::save_wide(&dump.join("model.png"), 160, &m.visible(front, img.palette.d021, scroll.fine))?;
    if bad == 0 { Ok(()) } else { Err(format!("{bad} bytes differ")) }
}

/// Render a prepared model at every declared orientation into a sheet, in a stage palette.
fn cmd_bake_model(argv: &[String]) -> Result<(), String> {
    let path = argv.first().filter(|a| !a.starts_with("--")).ok_or("usage: bake-model <file.mesh> [--palette v,d,l,h] [--out PNG]")?;
    let args = Args(argv[1..].to_vec());
    let pal: Vec<u8> = args
        .opt("--palette")
        .unwrap_or("6,12,1,3")
        .split(',')
        .map(|c| c.trim().parse::<u8>().map_err(|_| format!("--palette: bad color {c}")))
        .collect::<Result<_, _>>()?;
    if pal.len() != 4 {
        return Err("--palette needs four colors: void,dark,light,hot".into());
    }
    let palette = render::Palette::new(pal[0], pal[1], pal[2], pal[3]);
    let model = mesh::load(Path::new(path))?;
    let frames = model.render_all();
    let default_out = format!("output/models/{}.png", model.name);
    let out = PathBuf::from(args.opt("--out").unwrap_or(&default_out));
    let (w, px) = sheet(&frames, palette);
    png::save_wide(&out, w, &px)?;
    let bytes: usize = frames.iter().map(bake::frame_bytes).sum();
    let share = frames.iter().map(mesh::code_share).fold([0.0; 4], |a, s| [a[0] + s[0], a[1] + s[1], a[2] + s[2], a[3] + s[3]]);
    let n = frames.len() as f64;
    println!(
        "{}: {} tris, {} frames {}x{}, {} REU bytes; code share void {:.0}% dark {:.0}% light {:.0}% hot {:.0}%",
        model.name,
        model.mesh.tris.len(),
        frames.len(),
        model.canvas.0,
        model.canvas.1,
        bytes,
        share[0] / n * 100.0,
        share[1] / n * 100.0,
        share[2] / n * 100.0,
        share[3] / n * 100.0
    );
    println!("wrote {}", out.display());
    Ok(())
}
