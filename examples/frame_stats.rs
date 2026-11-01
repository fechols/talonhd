//! Per-frame work counts for the attract mode at tick `t`, from the host model.
//! `cargo run --release --example frame_stats -- dist/talonhd.reu [t]`
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
    in_column: bool,
}

impl Hw for Counting<'_> {
    fn peek(&mut self, a: u16) -> u8 { self.m.peek(a) }
    fn poke(&mut self, a: u16, v: u8) {
        if (0xD000..0xE000).contains(&a) { self.io_writes += 1; }
        if self.in_column && !(0xD000..0xE000).contains(&a) { self.blends += 1; }
        self.m.poke(a, v)
    }
    fn reu_exec(&mut self, c: u8) {
        self.io_writes += 1;
        let before = self.m.stats.bytes;
        self.m.reu_exec(c);
        self.dma_bytes += (self.m.stats.bytes - before) as u32;
        if self.in_column { self.links += 1 } else { self.other_dmas += 1 }
    }
    fn draw_column(&mut self) {
        self.in_column = true;
        render::draw_column(self);
        self.in_column = false;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let image = std::fs::read(&args[1]).unwrap();
    let t: u16 = args.get(2).map_or(25, |s| s.parse().unwrap());
    let mut m = Machine::new(&image);
    let img = render::read_image(&mut m).unwrap();
    let mut c = Counting { m: &mut m, links: 0, other_dmas: 0, dma_bytes: 0, blends: 0, io_writes: 0, in_column: false };
    let g = game::replay(t as u32);
    let mut sprites = [render::Sprite::default(); game::INSTANCES];
    let n = game::scene(&g, &mut sprites);
    let scroll = game::view(&g, &img);
    render::render_frame(&mut c, &img, BUFFERS[0], scroll, &mut sprites[..n]);
    println!("t={t}: column links {} | setup/bg DMAs {} | DMA bytes {} | blends {} | I/O writes {}",
        c.links, c.other_dmas, c.dma_bytes, c.blends, c.io_writes);
}
