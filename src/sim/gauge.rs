//! The arming gauge (`docs/design.md` 6) and the score (10). M2 arms SPEED and TWIN only; ARM on any other
//! slot does nothing yet (the cursor stays), so the gauge row still teaches the cursor.
use super::world::*;
use super::ARM;

/// Gauge slots (1-based, `docs/design.md` 6.2) and the weapon ids of `Gauge::main`.
pub const SPEED: u8 = 1;
pub const TWIN: u8 = 2;
pub const SLOTS: u8 = 8;
pub const SPEED_MAX: u8 = 5;
/// Bonuses: passing every slot (cursor 8 -> 1) and arming something already held.
pub const WRAP_BONUS: u32 = 2_000;
pub const REFIT_BONUS: u32 = 500;
/// Extends at 50,000 and 150,000, then every 200,000; 20,000 instead when the stock is full.
pub const FIRST_EXTEND: u32 = 50_000;
pub const SECOND_EXTEND: u32 = 150_000;
pub const EXTEND_STEP: u32 = 200_000;
pub const FULL_STOCK_BONUS: u32 = 20_000;
/// Counterstop: scores count in tens up to 9,999,990.
pub const COUNTERSTOP: u32 = 9_999_990;

impl World {
    /// A SPARK was collected: the cursor moves one slot, wrapping 8 -> 1 with the WRAP bonus.
    #[inline(never)]
    pub(crate) fn advance_cursor(&mut self) {
        if self.gauge.cursor == SLOTS {
            self.gauge.cursor = 1;
            self.add_score(WRAP_BONUS);
        } else {
            self.gauge.cursor += 1;
        }
    }

    /// ARM (edge-triggered) commits the highlighted slot; every ARM grants the arming invulnerability.
    #[inline(never)]
    pub(crate) fn arm(&mut self, input: u8) {
        if input & ARM == 0 || self.player.arm_held || self.gauge.cursor == 0 { return; }
        match self.gauge.cursor {
            SPEED => {
                self.gauge.speed = if self.gauge.speed == SPEED_MAX { 1 } else { self.gauge.speed + 1 };
            }
            TWIN => {
                if self.gauge.main == TWIN { self.add_score(REFIT_BONUS); }
                self.gauge.main = TWIN;
            }
            _ => return,
        }
        self.gauge.cursor = 0;
        self.arms = self.arms.wrapping_add(1);
        if self.player.invuln < super::player::ARM_INVULN {
            self.player.invuln = super::player::ARM_INVULN;
            self.player.invuln_kind = INV_ARM;
        }
    }

    /// Add points with the counterstop and the extends.
    #[inline(never)]
    pub(crate) fn add_score(&mut self, points: u32) {
        self.score = (self.score + points).min(COUNTERSTOP);
        if self.score > self.hi { self.hi = self.score; }
        while self.score >= self.next_extend {
            if self.lives < MAX_LIVES { self.lives += 1; } else { self.score = (self.score + FULL_STOCK_BONUS).min(COUNTERSTOP); }
            self.next_extend = if self.next_extend == FIRST_EXTEND { SECOND_EXTEND } else { self.next_extend + EXTEND_STEP };
        }
    }
}
