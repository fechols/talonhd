//! The attract mode's stage clear: the tick each phase of the sequence starts (for `preview --t`).
//!
//!     cargo run --release --example clear_trace [ticks]
use talonhd::game::Game;

fn main() {
    let ticks: u32 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(20_000);
    let names = ["OFF", "DARK", "FLYOUT", "CARD", "WALK", "STAGE"];
    let mut g = Game::new();
    let mut mind = talonhd::ace::Mind::new();
    let mut last = 0;
    for t in 1..=ticks {
        let input = talonhd::ace::fly(&mut g, &mut mind);
        g.step(input);
        if g.clear.phase != last {
            last = g.clear.phase;
            println!("tick {t:6}: {:6} stage {} score {} pal {:?}", names[last as usize], g.clear.stage + 1, g.world.score, g.clear.pal);
        }
    }
}
