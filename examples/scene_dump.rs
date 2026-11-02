//! The attract mode's sprite lists, in draw order, as `tools/profile_frame.py` replays them: one line per
//! sprite, `tick frame x y` (y already moved down by the fine scroll, as the C64's main loop draws it).
//!
//!     cargo run --release --example scene_dump -- 10300 10765 > scenes.txt
use talonhd::{game, render};

fn main() {
    let mut ticks: Vec<u32> = std::env::args().skip(1).map(|a| a.parse().unwrap()).collect();
    ticks.sort();
    let mut g = game::Game::new();
    let mut mind = talonhd::ace::Mind::new();
    let image = std::fs::read("dist/talonhd.reu").expect("bake first");
    let mut m = talonhd::model::Machine::new(&image);
    let img = render::read_image(&mut m).unwrap();
    for t in ticks {
        while g.ticks < t {
            let input = talonhd::ace::fly(&mut g, &mut mind);
            g.step(input);
        }
        let mut sprites = [render::Sprite::default(); game::INSTANCES];
        let n = game::scene(&g, &mut sprites);
        let scroll = game::view(&g, &img);
        render::sort_by_z(&mut sprites[..n]);
        for s in &sprites[..n] {
            println!("{t} {} {} {}", s.frame, s.x, s.y + scroll.fine as i16);
        }
    }
}
