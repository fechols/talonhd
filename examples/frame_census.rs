//! Join hardware frame timings with the host model's work for the same frames: replays the attract run
//! (ACE) to every logged tick and renders it through a counting `Hw`, so the hardware's microseconds can
//! be fitted against DMA bytes, column links, I/O writes and blends.
//!
//!     cargo run --release --example frame_census -- dist/talonhd.reu timing.csv > census.csv
//!
//! `timing.csv`: `t,render,bg,audio,step` per line (header first), from the `timing` C64 build's status.
use talonhd::hw::Hw;
use talonhd::layout::BUFFERS;
use talonhd::model::Machine;
use talonhd::{game, render};

struct Counting<'a> {
    m: &'a mut Machine,
    links: u32,
    other_dmas: u32,
    dma_bytes: u32,
    blends: u32,
    io_writes: u32,
    peeks: u32,
    in_column: bool,
}

impl Hw for Counting<'_> {
    fn peek(&mut self, a: u16) -> u8 {
        self.peeks += 1;
        self.m.peek(a)
    }
    fn poke(&mut self, a: u16, v: u8) {
        if (0xD000..0xE000).contains(&a) {
            self.io_writes += 1;
        } else if self.in_column {
            self.blends += 1;
        }
        self.m.poke(a, v)
    }
    fn reu_exec(&mut self, c: u8) {
        self.io_writes += 1;
        let before = self.m.stats.bytes;
        self.m.reu_exec(c);
        self.dma_bytes += (self.m.stats.bytes - before) as u32;
        if self.in_column { self.links += 1 } else { self.other_dmas += 1 }
    }
    fn draw_cols(&mut self) {
        self.in_column = true;
        render::draw_cols(self);
        self.in_column = false;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let image = std::fs::read(&args[1]).unwrap();
    let log = std::fs::read_to_string(&args[2]).unwrap();
    let mut rows: Vec<Vec<u32>> = log.lines().skip(1).filter_map(|l| l.split(',').map(|v| v.trim().parse().ok()).collect()).collect();
    rows.sort();
    rows.dedup_by_key(|r| r[0]);
    let mut m = Machine::new(&image);
    let img = render::read_image(&mut m).unwrap();
    let mut g = game::Game::new();
    let mut mind = talonhd::ace::Mind::new();
    println!("t,render,bg,audio,step,sprites,bullets,shots,links,dmas,dma_bytes,blends,io_writes,peeks,boss,keel,slice,threats");
    for r in rows {
        let t = r[0];
        // The C64's ticks wrap at 16 bits in the status block; the log starts from a fresh boot.
        while g.ticks < t {
            let input = talonhd::ace::fly(&mut g, &mut mind);
            g.step(input);
        }
        let mut c = Counting { m: &mut m, links: 0, other_dmas: 0, dma_bytes: 0, blends: 0, io_writes: 0, peeks: 0, in_column: false };
        let mut sprites = [render::Sprite::default(); game::INSTANCES];
        let n = game::scene(&g, &mut sprites);
        let scroll = game::view(&g, &img);
        let mut im = img;
        im.background = render::background_at(&mut *c.m, &img, g.ticks as u8);
        render::render_frame(&mut c, &im, BUFFERS[0], scroll, &mut sprites[..n]);
        let bullets = g.world.ebullets.iter().filter(|b| b.kind != 0).count();
        let shots = g.world.pairs.iter().filter(|p| p.alive != 0).count();
        // The step logged with this frame makes tick t + 1: the pilot's slice due now, and how many
        // threats `see` would gather (bullets, WASPs, GANNETs near the ship, at most 16).
        let slice = g.ace.slice();
        let (px, py) = ((g.world.player.x >> 8) as i32, (g.world.player.y >> 8) as i32);
        let near = |x: i32, y: i32| (x - px).abs() < 112 && y > py - 160 && y < py + 48;
        let threats = (g.world.ebullets.iter().filter(|b| b.kind != 0 && near((b.x >> 6) as i32, (b.y >> 6) as i32)).count()
            + g.world.wasps.iter().filter(|w| w.alive && near(w.x >> 8, w.y >> 8)).count()
            + g.world.gannets.iter().filter(|k| k.state != 0 && near(k.x >> 8, k.y >> 8)).count())
            .min(16);
        println!(
            "{},{},{},{},{},{n},{bullets},{shots},{},{},{},{},{},{},{},{},{slice},{threats}",
            t, r[1], r[2], r[3], r[4], c.links, c.other_dmas, c.dma_bytes, c.blends, c.io_writes, c.peeks, g.boss.state, g.midboss.state
        );
    }
}
