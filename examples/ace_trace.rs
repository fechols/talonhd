//! The attract pilot's flight log: score, lives, kills, deaths and the fights, every N ticks.
//!
//!     cargo run --release --example ace_trace -- 16000 1000
use talonhd::game::Game;

fn main() {
    let args: Vec<u32> = std::env::args().skip(1).map(|a| a.parse().unwrap()).collect();
    let (ticks, every) = (args.first().copied().unwrap_or(16_000), args.get(1).copied().unwrap_or(1_000));
    let mut g = Game::new();
    let mut mind = talonhd::ace::Mind::new();
    for t in 1..=ticks {
        let input = talonhd::ace::fly(&mut g, &mut mind);
        g.step(input);
        if t % every == 0 {
            let w = &g.world;
            println!(
                "t {t:6}  score {:7}  lives {}  kills {:4}  deaths {}  speed {}  keel {}  queen {}",
                w.score, w.lives, w.kills, g.pilot.deaths, w.gauge.speed, g.midboss.state, g.boss.state
            );
        }
    }
}
