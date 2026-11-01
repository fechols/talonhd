//! A state digest for replay tests: the host and the C64 must produce the same value after the same
//! input log. Fields are fed one by one (little-endian), so struct layout and padding never matter.
//! The mixing is cheap on the 6502 (a rotate, an exclusive or and an add per byte): it detects
//! divergence, it is not a cryptographic hash.
use super::world::*;

pub struct Digest {
    a: u16,
    b: u16,
}

impl Digest {
    pub const fn new() -> Digest { Digest { a: 0x5441, b: 0x4c4f } }

    #[inline(never)]
    pub fn byte(&mut self, v: u8) {
        self.a = self.a.rotate_left(1) ^ v as u16;
        self.b = self.b.wrapping_add(self.a);
    }

    #[inline(never)]
    pub fn bytes(&mut self, values: &[u8]) {
        for &v in values { self.byte(v); }
    }

    #[inline(never)]
    pub fn u16(&mut self, v: u16) { self.bytes(&v.to_le_bytes()); }
    #[inline(never)]
    pub fn i16(&mut self, v: i16) { self.bytes(&v.to_le_bytes()); }
    #[inline(never)]
    pub fn u32(&mut self, v: u32) { self.bytes(&v.to_le_bytes()); }
    #[inline(never)]
    pub fn i32(&mut self, v: i32) { self.bytes(&v.to_le_bytes()); }

    pub fn value(&self) -> u32 { (self.b as u32) << 16 | self.a as u32 }
}

impl World {
    /// The digest of every gameplay field.
    #[inline(never)]
    pub fn digest(&self) -> u32 {
        let mut d = Digest::new();
        d.u32(self.tick);
        d.u32(self.scroll);
        d.byte(self.state);
        d.u16(self.timer);
        let p = &self.player;
        d.i32(p.x);
        d.i32(p.y);
        d.bytes(&[p.fire_timer, p.invuln, p.invuln_kind, p.arm_held as u8]);
        for pair in &self.pairs {
            d.byte(pair.alive);
            d.i16(pair.x);
            d.i16(pair.y);
        }
        for w in &self.wasps {
            d.bytes(&[w.alive as u8, w.leader as u8, w.curve, w.seg, w.seg_t, w.angle]);
            d.i32(w.x);
            d.i32(w.y);
        }
        for b in &self.bursts {
            d.byte(b.t);
            d.i16(b.x);
            d.i16(b.y);
        }
        for s in &self.sparks {
            d.byte(s.alive as u8);
            d.i16(s.x0);
            d.i32(s.y);
            d.byte(s.t);
        }
        for g in &self.gannets {
            d.bytes(&[g.state, g.hp, g.t]);
            d.i32(g.x);
            d.i32(g.y);
            d.i16(g.vy);
        }
        for b in &self.ebullets {
            d.byte(b.kind);
            d.i16(b.x);
            d.i16(b.y);
            d.bytes(&[b.vx as u8, b.vy as u8]);
        }
        for w in &self.waves { d.bytes(&[w.left, w.timer, w.record]); }
        d.byte(self.track_next);
        d.u32(self.track_base);
        d.bytes(&[self.gauge.cursor, self.gauge.speed, self.gauge.main]);
        d.u32(self.score);
        d.u32(self.hi);
        d.byte(self.lives);
        d.u32(self.next_extend);
        d.byte(self.boss_ceiling as u8);
        d.i32(self.ceiling);
        d.u16(self.kills);
        d.u16(self.pickups);
        d.u16(self.deaths);
        d.u16(self.arms);
        d.value()
    }
}
