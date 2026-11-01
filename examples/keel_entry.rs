//! Render the KEELBREAKER's entry (attract game started just before its line) through the byte-exact
//! model and check every frame against the reference compositor.
use talonhd::layout::BUFFERS;
use talonhd::model::{self, Machine};
use talonhd::{game, reference, render};

fn main() {
    let image = std::fs::read("dist/talonhd.reu").unwrap();
    let mut m = Machine::new(&image);
    let img = render::read_image(&mut m).unwrap();
    let pano = talonhd::s1::panorama();
    let frames = talonhd::models::demo_frames_with_assets();
    let mut g = game::Game::new();
    g.world.scroll = (talonhd::midboss::ARRIVE as u32 - 20) << 8;
    g.world.track_base = 1_000_000;
    for t in 0..400u32 {
        let input = talonhd::sim::replay::autopilot(&g.world);
        g.step(input);
        let buf = BUFFERS[(t & 1) as usize];
        let scroll = model::render_game(&mut m, &img, &g, buf);
        let mut sc = [render::Sprite::default(); game::INSTANCES];
        let n = game::scene(&g, &mut sc);
        let want = reference::compose(&pano, &frames, scroll, &sc[..n]);
        let codes = m.codes(buf);
        let bad = (0..codes.len()).filter(|&i| codes[i] != want.codes[i]).count();
        if bad != 0 || m.stats.stray_writes != 0 || m.stats.bank_wraps != 0 {
            println!("tick {t}: {bad} bad pixels, {:?}; sprites {:?}", m.stats, &sc[..n.min(6)]);
            return;
        }
    }
    println!("400 entry frames match; stats {:?}", m.stats);
}
