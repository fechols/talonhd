//! What killed the attract pilot: the world around the ship for the ticks before each death.
//!
//!     cargo run --release --example ace_death -- 16000
use talonhd::game::Game;
use talonhd::sim::world::{DYING, PLAY};

fn main() {
    let ticks: u32 = std::env::args().nth(1).map_or(16_000, |a| a.parse().unwrap());
    let mut g = Game::new();
    let mut mind = talonhd::ace::Mind::new();
    let mut log: std::collections::VecDeque<String> = Default::default();
    for t in 0..ticks {
        let was = g.world.state;
        let input = talonhd::ace::fly(&mut g, &mut mind);
        let w = &g.world;
        let (px, py) = (w.player.x >> 8, w.player.y >> 8);
        let mut near = vec![];
        for b in w.ebullets.iter().filter(|b| b.kind != 0) {
            let (bx, by) = ((b.x >> 6) as i32, (b.y >> 6) as i32);
            if (bx - px).abs() < 40 && (by - py).abs() < 40 {
                near.push(format!("bullet {} ({},{}) v ({},{})", b.kind, bx - px, by - py, b.vx, b.vy));
            }
        }
        for k in w.wasps.iter().filter(|k| k.alive) {
            let (kx, ky) = (k.x >> 8, k.y >> 8);
            if (kx - px).abs() < 48 && (ky - py).abs() < 48 {
                near.push(format!("wasp ({},{}) angle {}", kx - px, ky - py, k.angle));
            }
        }
        for k in w.gannets.iter().filter(|k| k.state != 0) {
            let (kx, ky) = (k.x >> 8, k.y >> 8);
            if (kx - px).abs() < 60 && (ky - py).abs() < 60 {
                near.push(format!("gannet state {} ({},{})", k.state, kx - px, ky - py));
            }
        }
        log.push_back(format!("t {t} ship ({px},{py}) input {input:06b} slice {} near: {}", g.ace.slice(), near.join("; ")));
        if log.len() > 14 {
            log.pop_front();
        }
        g.step(input);
        if was == PLAY && g.world.state == DYING {
            println!("death at {t}:");
            for l in &log {
                println!("  {l}");
            }
        }
    }
}
