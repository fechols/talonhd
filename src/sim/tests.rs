//! Simulation unit tests and the deterministic replay test.
use super::actors::{self, BURST_TICKS};
use super::player::*;
use super::track::*;
use super::world::*;
use super::*;

fn px(v: i32) -> i32 { v >> 8 }

fn quiet() -> World {
    // A world with the spawn track pushed far away, so nothing flies in unless a test places it.
    let mut w = World::new();
    w.track_base = 1_000_000;
    w
}

fn wasp_at(w: &mut World, k: usize, x: i32, y: i32, leader: bool) {
    // A wasp parked on a curve that has ended (straight up at heading 0) is moved back each tick by tests.
    w.wasps[k] = Wasp { alive: true, leader, curve: HOOK, seg: 8, seg_t: 0, angle: 0, x: x * PX, y: y * PX };
}

#[test]
fn speed_levels_and_diagonals() {
    for level in 0..6u8 {
        let mut w = quiet();
        w.gauge.speed = level;
        let x0 = w.player.x;
        w.step(RIGHT);
        assert_eq!(w.player.x - x0, SPEED_Q8[level as usize]);
        let (x0, y0) = (w.player.x, w.player.y);
        w.step(LEFT | UP);
        let d = SPEED_DIAGONAL_Q8[level as usize];
        assert_eq!((x0 - w.player.x, y0 - w.player.y), (d, d));
        assert_eq!(d, SPEED_Q8[level as usize] * DIAGONAL >> 8);
        assert!(d >= SPEED_MIN * DIAGONAL >> 8);
    }
}

#[test]
fn clamps_keep_the_talon_in_the_safe_area_and_between_the_hud_rows() {
    let mut w = quiet();
    for _ in 0..400 { w.step(LEFT | UP); }
    assert_eq!((px(w.player.x), px(w.player.y)), (LEFT_X, TOP));
    // The 48 x 42 grid's top-left is inside the playfield.
    assert!(px(w.player.x) - 24 >= 0 && px(w.player.y) - 21 >= 0);
    for _ in 0..400 { w.step(RIGHT | DOWN); }
    assert_eq!((px(w.player.x), px(w.player.y)), (RIGHT_X, BOTTOM));
    assert!(px(w.player.x) + 24 <= FIELD_W as i32 && px(w.player.y) + 21 <= FIELD_H as i32);
}

#[test]
fn boss_ceiling_eases_in_over_thirty_ticks() {
    let mut w = quiet();
    for _ in 0..200 { w.step(UP); }
    assert_eq!(px(w.player.y), TOP);
    w.boss_ceiling = true;
    let mut ys = Vec::new();
    for _ in 0..40 { w.step(UP); ys.push(w.player.y); }
    // Never snaps: each tick moves at most one easing step, and it settles at line 88 within 31 ticks.
    let mut previous = TOP * PX;
    for &y in &ys { assert!(y - previous <= CEILING_EASE + 1); previous = y; }
    assert_eq!(px(ys[31]), CEILING_TOP);
    assert_eq!(px(w.player.y) - 21, 88, "the top edge sits on playfield line 88");
    w.boss_ceiling = false;
    for _ in 0..80 { w.step(UP); }
    assert_eq!(px(w.player.y), TOP);
}

#[test]
fn twin_fires_a_pair_every_six_ticks_and_bolts_leave_the_top() {
    let mut w = quiet();
    let mut fired = 0;
    for t in 0..60 {
        let before = w.pairs.iter().filter(|p| p.alive != 0).count();
        w.step(FIRE);
        let after = w.pairs.iter().filter(|p| p.alive != 0).count();
        if after > before { fired += 1; assert_eq!(t % TWIN_INTERVAL as usize, 0, "tick {t}"); }
    }
    assert_eq!(fired, 10);
    let p = w.pairs.iter().find(|p| p.alive == 3).unwrap();
    assert_eq!(p.x, px(w.player.x) as i16);
    // Holding fire never exceeds the pool; releasing lets every pair leave.
    for _ in 0..40 { w.step(0); }
    assert!(w.pairs.iter().all(|p| p.alive == 0));
}

#[test]
fn shots_kill_wasps_score_and_the_leader_drops_a_spark() {
    let mut w = quiet();
    let x = px(w.player.x);
    wasp_at(&mut w, 0, x - 4, 40, false);
    wasp_at(&mut w, 1, x + 4, 20, true);
    let mut ticks = 0;
    while w.kills < 2 && ticks < 60 {
        // Hold the WASPs in place (they would fly up on heading 0).
        w.wasps[0].y = 40 * PX;
        w.wasps[1].y = 20 * PX;
        w.step(FIRE);
        ticks += 1;
    }
    assert_eq!(w.kills, 2);
    assert_eq!(w.score, 200);
    assert_eq!(w.bursts.iter().filter(|b| b.t > 0).count(), 2);
    let spark = w.sparks.iter().find(|s| s.alive).expect("the leader drops a SPARK");
    assert_eq!(spark.x0, (x + 4) as i16);
    for _ in 0..BURST_TICKS { w.step(0); }
    assert!(w.bursts.iter().all(|b| b.t == 0));
}

#[test]
fn a_bolt_that_misses_by_a_pixel_misses() {
    let mut w = quiet();
    let x = px(w.player.x);
    // Right bolt at x + 4 spans x + 2 .. x + 5; the WASP's hurtbox x + 14 .. x + 29 does not reach it.
    wasp_at(&mut w, 0, x + 4 + 2 + 8, 60, false);
    for _ in 0..30 { w.wasps[0].y = 60 * PX; w.step(FIRE); }
    assert_eq!(w.kills, 0);
    wasp_at(&mut w, 0, x + 4 + 2 + 7, 60, false);
    for _ in 0..30 { w.wasps[0].y = 60 * PX; w.step(FIRE); }
    assert_eq!(w.kills, 1);
}

#[test]
fn wasps_kill_the_talon_unless_it_is_invulnerable() {
    let mut w = quiet();
    let (x, y) = (px(w.player.x), px(w.player.y));
    w.player.invuln = 10;
    w.player.invuln_kind = INV_ARM;
    wasp_at(&mut w, 0, x, y, false);
    w.step(0);
    assert_eq!((w.state, w.deaths), (PLAY, 0));
    w.player.invuln = 0;
    wasp_at(&mut w, 0, x + 9, y + 2, false);
    w.step(0);
    assert_eq!((w.state, w.deaths, w.lives), (DYING, 1, START_LIVES - 1));
    assert_eq!(w.gauge.cursor, 1, "resupply: the first ARM after a death is SPEED");
    // Death -> launch -> play with the respawn blink.
    for _ in 0..DYING_TICKS { w.step(0); }
    assert_eq!(w.state, LAUNCH);
    for _ in 0..LAUNCH_TICKS { w.step(0); }
    assert_eq!((w.state, w.player.y, w.player.invuln_kind), (PLAY, START_Y, INV_RESPAWN));
    assert_eq!(w.player.invuln, RESPAWN_INVULN - 1, "the tick that hands back control counts");
}

#[test]
fn the_wasp_hitbox_edges() {
    // The 4 x 4 hitbox (2 px below the anchor) against the 16 x 14 hurtbox: overlap needs |dx| < 10, |dy| < 9.
    for (dx, dy, dies) in [(9, 2, true), (10, 2, false), (0, 2 + 8, true), (0, 2 + 9, false), (-9, -6, true), (0, -7, false)] {
        let mut w = quiet();
        let (x, y) = (px(w.player.x), px(w.player.y));
        wasp_at(&mut w, 0, x + dx, y + dy, false);
        w.wasps[0].y += WASP_VY[0] as i32 * -1; // undo this tick's flight so it is tested where placed
        w.step(0);
        assert_eq!(w.state == DYING, dies, "dx {dx} dy {dy}");
    }
}

#[test]
fn sparks_advance_the_cursor_and_arm_speed_and_twin() {
    let mut w = quiet();
    let (x, y) = (px(w.player.x), px(w.player.y));
    w.sparks[0] = Spark { alive: true, x0: x as i16 + 8, y: (y as i32 + 2) * PX, t: 16 };
    w.step(0);
    assert_eq!((w.gauge.cursor, w.pickups, w.score), (1, 1, 100));
    // ARM is edge-triggered: holding it arms once.
    for _ in 0..5 { w.step(ARM); }
    assert_eq!((w.gauge.speed, w.gauge.cursor, w.arms), (1, 0, 1));
    assert_eq!((w.player.invuln_kind, w.player.invuln), (INV_ARM, ARM_INVULN - 5));
    // TWIN while holding TWIN is a refit: 500 points.
    w.gauge.cursor = gauge::TWIN;
    w.step(0);
    w.step(ARM);
    assert_eq!((w.score, w.gauge.cursor), (600, 0));
    // Unimplemented slots keep the cursor.
    w.gauge.cursor = 5;
    w.step(0);
    w.step(ARM);
    assert_eq!(w.gauge.cursor, 5);
    // SPEED 5 wraps to 1.
    w.gauge.speed = 5;
    w.gauge.cursor = gauge::SPEED;
    w.step(0);
    w.step(ARM);
    assert_eq!(w.gauge.speed, 1);
    // Cursor 8 -> 1 pays the WRAP bonus.
    w.gauge.cursor = 8;
    let score = w.score;
    w.advance_cursor();
    assert_eq!((w.gauge.cursor, w.score - score), (1, gauge::WRAP_BONUS));
}

#[test]
fn extends_and_counterstop() {
    let mut w = quiet();
    w.add_score(49_990);
    assert_eq!(w.lives, START_LIVES);
    w.add_score(10);
    assert_eq!(w.lives, START_LIVES + 1);
    w.add_score(100_000);
    assert_eq!((w.lives, w.next_extend), (START_LIVES + 2, 350_000));
    w.add_score(20_000_000);
    assert_eq!(w.score, gauge::COUNTERSTOP);
}

#[test]
fn game_over_then_a_new_game_keeps_the_high_score() {
    let mut w = quiet();
    w.lives = 1;
    w.score = 123_450;
    w.hi = 123_450;
    let (x, y) = (px(w.player.x), px(w.player.y));
    wasp_at(&mut w, 0, x, y, false);
    w.step(0);
    for _ in 0..DYING_TICKS { w.step(0); }
    assert_eq!(w.state, GAME_OVER);
    for _ in 0..GAME_OVER_TICKS { w.step(0); }
    assert_eq!((w.state, w.score, w.hi, w.lives, w.tick), (PLAY, 0, 123_450, START_LIVES, 0));
}

#[test]
fn reset_equals_new() {
    let mut w = World::new();
    w.hi = DEFAULT_HI;
    for _ in 0..500 { w.step(FIRE | RIGHT); }
    w.reset();
    assert_eq!(w, World::new());
}

#[test]
fn curves_keep_horizontal_legs_above_line_120_and_leave_the_screen() {
    for &curve in &[S_CURVE, S_CURVE | MIRROR, HOOK, HOOK | MIRROR, DROP, DROP | MIRROR] {
        let record = TRACK.iter().find(|r| r.curve == curve).unwrap();
        let mut w = quiet();
        w.wasps[0] = Wasp { alive: true, leader: false, curve, seg: 0, seg_t: 0, angle: CURVES[(curve & !MIRROR) as usize].heading,
            x: record.x2 as i32 * 2 * PX, y: record.y as i32 * PX };
        if curve & MIRROR != 0 { w.wasps[0].angle = w.wasps[0].angle.wrapping_neg(); }
        let mut path = Vec::new();
        for _ in 0..400 {
            let before = w.wasps[0];
            w.step(0);
            if !w.wasps[0].alive { break; }
            path.push((before, w.wasps[0]));
        }
        assert!(!w.wasps[0].alive, "curve {curve:#x} leaves the screen");
        for (a, b) in &path {
            let (dx, dy) = ((b.x - a.x).abs(), (b.y - a.y).abs());
            if dx > dy && px(b.y) >= 120 && px(b.y) < FIELD_H as i32 && (0..FIELD_W as i32).contains(&px(b.x)) {
                panic!("curve {curve:#x}: horizontal movement at y {}", px(b.y));
            }
        }
    }
}

#[test]
fn the_track_spawns_strings_of_six_with_a_leader() {
    // S1's first wave: 6 WASPs at scroll line 200 (tick 400 at 0.5 px/tick), released 10 ticks apart.
    let mut w = World::new();
    let first = TRACK[0];
    assert_eq!((first.at, first.count, first.kind), (200, 6, WASP | LEADER));
    let mut seen = 0;
    for _ in 0..(first.at as u32 * 2 + 60) {
        w.step(0);
        seen = seen.max(w.wasps.iter().filter(|x| x.alive).count());
    }
    assert_eq!(seen, 6);
    assert_eq!(w.wasps.iter().filter(|x| x.alive && x.leader).count(), 1);
    // Strings are released 10 ticks apart along one path.
    let mut alive: Vec<_> = w.wasps.iter().filter(|x| x.alive).collect();
    alive.sort_by_key(|x| x.leader);
    assert!(alive[5].leader);
}

#[test]
fn no_new_waves_while_dying() {
    let mut w = World::new();
    w.state = DYING;
    w.timer = 1000;
    for _ in 0..300 { w.step(0); }
    assert!(w.waves.iter().all(|x| x.left == 0) && w.track_next == 0);
}

#[test]
fn step_is_the_world_half_then_the_sim_half() {
    let (inputs, _) = replay::record(600);
    let mut a = World::new();
    let mut b = World::new();
    for &i in &inputs {
        a.step(i);
        b.step_world();
        b.step_sim(i);
    }
    assert_eq!(a, b);
}

fn gannet_over(w: &mut World, x: i32, y: i32) {
    w.gannets[0] = Gannet { state: actors::G_DIVE, hp: actors::GANNET_HP, t: 0, x: x * PX, y: y * PX, vy: actors::GANNET_DIVE_Q8 };
}

#[test]
fn a_gannet_dives_releases_two_bombs_and_climbs_out() {
    let mut w = quiet();
    w.player.x = 200 * PX; // away from the dive column: the bombs are not point-blank
    gannet_over(&mut w, 100, -24);
    let mut released = None;
    for t in 0..400 {
        w.step(0);
        let live: Vec<_> = w.ebullets.iter().filter(|b| b.kind == actors::B_BOMB).collect();
        if released.is_none() && !live.is_empty() {
            released = Some(t);
            assert_eq!(live.len(), 2);
            // Released just below the release line, 10 degrees either side of straight down, at the racks.
            assert!(w.gannets[0].y >= actors::GANNET_RELEASE_Y * PX);
            assert_eq!((live[0].vx, live[1].vx), (-actors::BOMB_VX, actors::BOMB_VX));
            // The bombs fly their first step in the tick they are released.
            assert_eq!(live[1].x - live[0].x, 2 * actors::BOMB_DX * actors::Q6 + 2 * actors::BOMB_VX as i16);
        }
        if w.gannets[0].state == 0 { break; }
    }
    // 112 lines at 2 px/tick, then the 16-tick telegraph.
    assert_eq!(released, Some(56 + 16));
    assert_eq!(w.gannets[0].state, 0, "it climbed out of the top");
    assert_eq!(w.state, PLAY);
}

#[test]
fn no_point_blank_bullets_and_none_right_after_a_relaunch() {
    let mut w = quiet();
    w.spawn_ebullet(actors::B_ROUND, px(w.player.x) as i16 + 20, px(w.player.y) as i16 - 31, 0, 96);
    assert!(w.ebullets.iter().all(|b| b.kind == 0), "within 32 px");
    w.spawn_ebullet(actors::B_ROUND, px(w.player.x) as i16 + 20, px(w.player.y) as i16 - 33, 0, 96);
    assert_eq!(w.ebullets.iter().filter(|b| b.kind != 0).count(), 1);
    let mut w = quiet();
    w.player.invuln_kind = INV_RESPAWN;
    w.player.invuln = RESPAWN_INVULN - 29;
    w.spawn_ebullet(actors::B_ROUND, 10, 10, 0, 96);
    assert!(w.ebullets.iter().all(|b| b.kind == 0), "in the launch grace");
    w.player.invuln = RESPAWN_INVULN - 30;
    w.spawn_ebullet(actors::B_ROUND, 10, 10, 0, 96);
    assert_eq!(w.ebullets.iter().filter(|b| b.kind != 0).count(), 1);
}

#[test]
fn bullets_kill_the_talon_and_the_pool_drops_spawns_beyond_its_cap() {
    let mut w = quiet();
    let (x, y) = (px(w.player.x) as i16, px(w.player.y) as i16);
    w.spawn_ebullet(actors::B_ROUND, x, y - 40, 0, 96);
    for _ in 0..40 { w.step(0); if w.state != PLAY { break; } }
    assert_eq!((w.state, w.deaths), (DYING, 1));
    assert!(w.ebullets.iter().all(|b| b.kind == 0), "the bullet that hit is gone");
    let mut w = quiet();
    for i in 0..EBULLETS + 5 { w.spawn_ebullet(actors::B_ROUND, (i * 8) as i16, 10, 0, 0); }
    assert_eq!(w.ebullets.iter().filter(|b| b.kind != 0).count(), EBULLETS);
    assert_eq!(w.ebullets[0].x, 0, "the oldest bullet stays");
}

#[test]
fn twin_shoots_down_bombs_and_gannets() {
    let mut w = quiet();
    let x = px(w.player.x) as i16;
    // A bomb straight above the left bolt, falling slowly: one bolt pops it for 50.
    w.spawn_ebullet(actors::B_BOMB, x - BOLT_DX, 40, 0, 16);
    for _ in 0..20 { w.step(FIRE); }
    assert!(w.ebullets.iter().all(|b| b.kind == 0));
    assert_eq!(w.score, 50);
    // A GANNET parked over the TALON takes six bolts.
    let mut w = quiet();
    let x = px(w.player.x);
    gannet_over(&mut w, x, 40);
    w.gannets[0].vy = 0;
    w.gannets[0].state = actors::G_CLIMB;
    let mut ticks = 0;
    while w.gannets[0].state != 0 && ticks < 200 { w.step(FIRE); w.gannets[0].vy = 0; ticks += 1; }
    assert_eq!(w.gannets[0].state, 0);
    assert_eq!((w.kills, w.score), (1, 500));
}

/// The replay: 3,900 ticks (65 s) of the autopilot from a new game on S1's authored track. The digest is pinned: any change to
/// the simulation's behaviour changes it, and `tools/test_native_sim.py` requires the C64 to match.
pub const REPLAY_TICKS: usize = 3900;
pub const REPLAY_DIGEST: u32 = 0x780f_e9b1;

#[test]
fn replay_is_deterministic_and_plays_the_game() {
    let (inputs, digests) = replay::record(REPLAY_TICKS);
    let (again, digests2) = replay::record(REPLAY_TICKS);
    assert_eq!((inputs.clone(), digests.clone()), (again, digests2));
    let mut w = World::new();
    for &i in &inputs { w.step(i); }
    assert_eq!(w.digest(), digests[REPLAY_TICKS - 1]);
    println!("replay: kills {} pickups {} arms {} deaths {} score {} digest {:08x}", w.kills, w.pickups, w.arms, w.deaths, w.score, w.digest());
    // 65 s is S1 lines 0-1,950: the WASP waves at 200, 400, 840 and 1200, then the GANNETs of line 1600:
    // the autopilot stands beside a diving GANNET, so it releases its bombs (and the TALON is lost once).
    assert!(w.kills >= 20 && w.pickups >= 2 && w.arms >= 1, "the replay exercises kills, pickups and arming");
    let mut bombs = 0;
    let mut v = World::new();
    for &i in &inputs {
        let before = v.ebullets.iter().filter(|b| b.kind != 0).count();
        v.step(i);
        bombs += v.ebullets.iter().filter(|b| b.kind != 0).count().saturating_sub(before);
    }
    assert!(bombs >= 2, "GANNETs released bombs in the replay ({bombs})");
    assert_eq!(w.digest(), REPLAY_DIGEST, "replay digest changed: {:08x}", w.digest());
}

/// Exports the replay for the native harness: `cargo test --lib write_sim_replay -- --ignored`.
#[test]
#[ignore]
fn write_sim_replay() {
    let (inputs, digests) = replay::record(REPLAY_TICKS);
    let mut world = World::new();
    let scores: Vec<u32> = inputs.iter().map(|&i| { world.step(i); world.score }).collect();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/target/sim_replay.json");
    let text = format!("{{\"inputs\":{:?},\"digests\":{:?},\"scores\":{:?}}}", inputs, digests, scores);
    std::fs::write(path, text).unwrap();
}

