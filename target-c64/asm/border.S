; The border painter (from ../typo 51e1b41, probe/tallprobe.S; THIRD-PARTY.md): the 20 lines above the
; 25-row window (block T, raster 31..50, canvas lines 0..19) and the 20 below (block B, raster 251..7,
; canvas lines 220..239) are painted through $D020, one store per multicolor pixel (2 dots, 15.75 cycles
; at 64 MHz), with the 160 pixels of the window's width. The window itself is the bitmap (25 rows,
; YSCROLL 3: no badline falls on a painted line).
;
; Each block is one raster IRQ (asm/hud.S's chain: HUD row 0 @20, T @29, HUD rows 1..7, gate @243,
; B @249). Per line: wait for CIA1 timer A's underflow (free-running at one raster line, 65 PHI2, phased
; once at boot by border_sync), spin to TA, re-align to PHI2 with a $D031 toggle, a 1-cycle trim sled
; (DF), a 5-cycle delay loop (DL), then the kernel: 40 bytes of the line, 4 stores each, exactly 63
; cycles a byte (stores at +0, 16, 32, 48: at most 0.375 cycle off the 15.75 grid), and the border colour
; for the right corner on the next grid slot.
;
; The lines come from STAGE ($5800-$5BFF): one block's 20 lines, line-major, 40 bytes a line, 6 lines a
; page (no line crosses a page: every load has a fixed cost). After a block the IRQ refills the staging
; with the next block's lines from the shown buffer's band set in the REU (render::stash_bands). The
; foreground loads and triggers every DMA with interrupts masked (asm/reu.S reu_transfer, blit.S reu_piece),
; so an IRQ never finds the REU half set up and saves nothing. Block B's IRQ also does a
; flip the main loop asked for (P_FLIP): bitmap bank, $D016, band set, together.
;
; The timing-critical part (border_lines .. fine's sled) is the one-page section .ptext ($8600): nothing in
; it can cross a page (a crossing costs a cycle and silently moves every store).

PB        = $CAE0     ; the painter's block (layout::painter)
P_ON      = PB + 0    ; 0: no painting (the HUD chain is the plain one)
P_GATE    = PB + 1    ; bit 7: no REU DMA may start (asm/reu.S, asm/blit.S)
P_NEXT    = PB + 2    ; the IRQ chain's next stage (asm/hud.S)
P_FLIP    = PB + 3    ; $80 | buffer: show that buffer at the end of block B; 0 once done
P_D016    = PB + 4
P_FRONT   = PB + 5    ; the shown buffer (its band set)
P_TA      = PB + 6
P_DL      = PB + 7
P_DF      = PB + 8
P_TURBO   = PB + 9
P_AOFS    = PB + 10
P_SYNC    = PB + 11
P_DELTA   = PB + 12
P_DLB     = PB + 13   ; the badline row's delay loop and trim (P_DL + 6, P_DF + 6: `ldx P_DL,y`)
P_DFB     = PB + 14
P_BADLINE = $CB5E     ; the line of this block on a badline (0..23; $FF none): set per block by setup
P_D011    = $CB5F     ; the $D011 block B's flip writes (bitmap, 24 rows, the frame's YSCROLL)
P_LATEBLK = PB + 15   ; u16: blocks entered too late to paint (skipped)
P_BORDER  = PB + 17   ; the border colour (the corners, the sides)
P_LI      = PB + 18
P_TMP     = PB + 19
P_FIRST   = PB + 20   ; the raster line of the block's first painted line
DJ        = PB + 21   ; 7: the foreground's DMA job (asm/reu.S reu_transfer; not used here)
P_COL     = PB + 28   ; 4: the colours of codes 00, 01, 10, 11 (border_tables)

STAGE     = $5800
DEC0      = $8700     ; dec0..3: the colour of pixel 0..3 of a bitmap byte (border_tables)
DEC1      = $8800
DEC2      = $8900
DEC3      = $8A00
LINES     = 24

; ---------------------------------------------------------------- timing-critical (one page)
.section .ptext,"ax",@progbits
; Paint one block's 20 lines from STAGE. P_FIRST: the raster line of line 0.
.globl border_lines
.type border_lines,@function
border_lines:
    lda $dc0d           ; drop a stale flag
1:  lda $dc0d
    lsr a
    bcc 1b
    lda $d012
    clc
    adc P_DELTA         ; the line this underflow's spin point falls in
    sta P_TMP
    lda P_FIRST
    sec
    sbc P_TMP
    beq 3f
    cmp #5
    bcc 1b              ; early (the IRQ comes 2 lines ahead): the next underflow
    inc P_LATEBLK       ; past it: too late to paint this block
    bne 2f
    inc P_LATEBLK+1
2:  rts
3:  ldx #0
line:
    stx P_LI
    lda line_lo,x
    sta k0+1
    sta k1+1
    sta k2a+1
    sta k2b+1
    lda line_hi,x
    sta k0+2
    sta k1+2
    sta k2a+2
    sta k2b+2
    ldy #0              ; Y: 0 a plain row, 6 the badline row (its delays: P_DLB, P_DFB)
    cpx P_BADLINE
    bne 9f
    ldy #6
9:  lda P_TA
spin:
    cmp $dc04
    bcc spin
    lda #$80
    ldx P_TURBO
    sta $d031
    stx $d031
    nop
    nop
    ldx P_DF,y
    jsr fine
    ldx P_DL,y
4:  dex
    bne 4b
    cpy #6
    beq kbad
    ldy #0
k0: ldx $ffff,y         ; byte 0
kernel:
    lda DEC0,x
    sta $d020           ; +0
    lda DEC1,x
    iny
    nop
    nop
    nop
    sta $d020           ; +16
    lda DEC2,x
    nop
    nop
    nop
    nop
    sta $d020           ; +32
    lda DEC3,x
k1: ldx $ffff,y         ; the next byte
    nop
    nop
    sta $d020           ; +48
    nop
    cpy #40
    bne kernel
    bit $02             ; (3 cycles: the corner lands on the next grid slot, +63)
kb: lda #0              ; the border colour (patched per block)
    sta $d020
.globl border_ltail
border_ltail:
    ldx P_LI
    inx
    cpx #LINES
    beq 6f
8:  lda $dc0d
    lsr a
    bcc 8b
    jmp line
6:  rts
kbad:
    jmp border_kbad
.size border_lines, . - border_lines

; ---------------------------------------------------------------- the trim (main RAM)
; No branches, so its cost does not depend on where it lives; finetab 16-byte aligned (no page crossing).
.section .text.border_fine,"ax",@progbits
; Delay X & 7 = 0..7 -> 6..13 cycles plus a fixed call cost (the 1-cycle trim).
.globl border_fine
fine:
border_fine:
    txa
    and #7
    asl a
    tax
    lda finetab,x
    sta 9f+1
    lda finetab+1,x
    sta 9f+2
9:  jmp $0000
f4: nop
    nop
    rts
f5: nop
    bit $ff
    rts
f6: nop
    nop
    nop
    rts
f7: nop
    nop
    bit $ff
    rts
f8: nop
    nop
    nop
    nop
    rts
f9: nop
    nop
    nop
    bit $ff
    rts
f10: nop
    nop
    nop
    nop
    nop
    rts
f11: nop
    nop
    nop
    nop
    bit $ff
    rts
.balign 16
finetab:
    .word f4, f5, f6, f7, f8, f9, f10, f11

; ---------------------------------------------------------------- the IRQ side ($8B00, .p2text)
.section .p2text,"ax",@progbits
; Block T (from hud_irq at raster 30, I/O in, registers saved): paint, refill the staging with the shown
; set's block B lines, open the DMA gate. (Refilling a line at a time in each painted line's idle tail made
; the lines with HUD sprites in the border late on the C64U: the sprite fetches eat that tail.)
.globl border_t
.type border_t,@function
border_t:
    lda $d011           ; the badline row: line 17 + YSCROLL (raster 48 + YSCROLL), none for 7
    and #7
    cmp #7
    beq 1f
    adc #17             ; C = 0
    .byte $2c           ; (bit abs: skips the lda)
1:  lda #$ff
    sta P_BADLINE
    lda #31
    jsr setup
    jsr border_lines
    lda P_FRONT
    ldx #1
    jsr refill
    lda #0
    sta P_GATE
    rts
.size border_t, . - border_t

; Block B (raster 246): paint, flip if asked, refill block T's lines from the (new) shown buffer's set.
.globl border_b
.type border_b,@function
border_b:
    lda $d011           ; the badline row: line 0 (raster 247, the 25th character row) for YSCROLL 7
    and #7
    cmp #7
    beq 1f
    lda #$ff
    .byte $2c           ; (bit abs: skips the lda)
1:  lda #0
    sta P_BADLINE
    lda #247
    jsr setup
    jsr border_lines
    lda P_FLIP
    bpl 1f
    and #1
    tax
    stx P_FRONT
    lda $dd00
    and #$fc
    ora dd00_bits,x
    sta $dd00
    lda P_D016
    sta $d016
    lda P_D011          ; bitmap, display on, 24 rows, this frame's YSCROLL
    sta $d011
    lda #0
    sta P_FLIP
1:  lda P_FRONT
    ldx #0
    jsr refill
    lda #0
    sta P_GATE
    rts
.size border_b, . - border_b

; A = the block's first raster line; the border colour patched in.
setup:
    sta P_FIRST
    lda P_BORDER
    sta kb+1
    sta kb2+1
    rts

; The badline row's kernel. On a badline the VIC's 40 character fetches each take about one 64 MHz cycle
; from the CPU, so the plain 63-cycle byte drifts a dot every ~8 bytes: here a byte takes 62 cycles, stores
; 16, 15, 16, 15 apart, the steals making up the rest. Entered after its own delay (P_DLB, P_DFB); the loop
; stays inside this page.
.globl border_kbad
border_kbad:
    ldy #0
k2a: ldx $ffff,y        ; byte 0
kbadl:
    lda DEC0,x
    sta $d020           ; +0
    lda DEC1,x
    iny
    nop
    nop
    nop
    sta $d020           ; +16
    lda DEC2,x
    nop
    nop
    bit $02
    sta $d020           ; +31
    lda DEC3,x
k2b: ldx $ffff,y        ; the next byte
    nop
    nop
    sta $d020           ; +47
    nop
    cpy #40
    bne kbadl
    bit $02
kb2: lda #0             ; the border colour (patched per block)
    sta $d020
    jmp border_ltail

; Refill the staging with block X's 24 lines (0 = T, 1 = B) of set A (layout::band_set: $FFF000 + $800 per
; buffer, block T's 960 bytes then block B's): four DMAs, one per page (6 lines = 240 bytes), the REU's
; address running on from one to the next.
refill:
    asl a
    asl a
    asl a
    ora #$f0
    clc
    adc block_mid,x
    sta $df05
    lda block_lo,x
    sta $df04
    lda #$ff
    sta $df06
    lda #0
    sta $df08
    ldx #0
2:  lda #0
    sta $df02
    txa
    clc
    adc #>STAGE
    sta $df03
    lda #240
    sta $df07
    lda #$91
    sta $df01
    inx
    cpx #4
    bne 2b
    stx $cade           ; layout::REU_DIRTY: the main loop's transfer reloads every register
    rts

; ---------------------------------------------------------------- boot and palette (main RAM)
.section .text.border,"ax",@progbits
; The decode tables from P_COL (call after every palette write): dec_i[b] = the colour of pair i of b.
.globl border_tables
.type border_tables,@function
border_tables:
    ldx #0
1:  txa
    lsr a
    lsr a
    lsr a
    lsr a
    lsr a
    lsr a
    tay
    lda P_COL,y
    sta DEC0,x
    txa
    lsr a
    lsr a
    lsr a
    lsr a
    and #3
    tay
    lda P_COL,y
    sta DEC1,x
    txa
    lsr a
    lsr a
    and #3
    tay
    lda P_COL,y
    sta DEC2,x
    txa
    and #3
    tay
    lda P_COL,y
    sta DEC3,x
    inx
    bne 1b
    rts
.size border_tables, . - border_tables

; The painter at boot (interrupts off): its parameters (defaults), state and diagnostics cleared, the
; staging zeroed (its first block, before any refill, paints void), CIA1 timer A phased (border_sync).
; It stays off (P_ON 0) until the main loop has shown a frame with its bands.
.globl border_boot
.type border_boot,@function
border_boot:
    lda #0
    ldx #31
1:  sta PB,x
    dex
    bpl 1b
    ldx #5
2:  lda defaults,x
    sta P_TA,x
    dex
    bpl 2b
    lda #71             ; the badline row's delays, calibrated on the C64U (2026-11-04)
    sta P_DLB
    lda #4
    sta P_DFB
    lda #$ff
    sta P_BADLINE
    lda #0
    tax
3:  sta STAGE,x
    sta STAGE+$100,x
    sta STAGE+$200,x
    sta STAGE+$300,x
    inx
    bne 3b
    jmp border_sync
.size border_boot, . - border_boot

; Phase CIA1 timer A to the raster (once at boot, interrupts off): one raster line a period (65 PHI2),
; started on a raster edge at line P_SYNC with the first period shortened by P_AOFS; then P_DELTA, the
; painted raster line minus the line the underflow flag is seen in, measured on the same instructions as
; a painted line. Waits for turbo first (the C64U engages it ~2 s after reset).
.globl border_sync
.type border_sync,@function
border_sync:
    php
    sei
1:  lda $d012
2:  cmp $d012
    beq 2b              ; a new raster line starts
    ldx #0
    lda $d012
3:  inx
    beq 4f              ; > 255 passes in one line: turbo
    cmp $d012
    beq 3b
    jmp 1b
4:  lda #0
    sta $dc0e
    sta $dc0f
    lda #64
    sec
    sbc P_AOFS
    sta $dc04
    lda #0
    sta $dc05           ; a high byte write while stopped loads the counter
    lda P_SYNC
5:  cmp $d012
    bne 5b
    lda $d012
6:  cmp $d012
    beq 6b
    lda #$80
    ldx P_TURBO
    sta $d031
    stx $d031
    lda #$01            ; start, continuous
    sta $dc0e
    lda #64             ; later periods: 65 (the latch only)
    sta $dc04
    lda #0
    sta $dc05
    lda #$7f            ; no CIA1 interrupts: the painter polls the flag
    sta $dc0d
    lda #120
7:  cmp $d012
    bne 7b
    lda $dc0d
8:  lda $dc0d
    lsr a
    bcc 8b
    lda $d012
    sta P_TMP
    lda P_TA
9:  cmp $dc04
    bcc 9b
    lda #$80
    ldx P_TURBO
    sta $d031
    stx $d031
    nop
    nop
    ldx P_DF
    jsr border_fine
    ldx P_DL
10: dex
    bne 10b
    lda $d012
    sec
    sbc P_TMP
    sta P_DELTA
    plp
    rts
.size border_sync, . - border_sync

.section .rodata.border,"a",@progbits
; STAGE line i: page i / 6, offset (i % 6) * 40.
line_lo:
.set i, 0
.rept LINES
    .byte (i % 6) * 40
.set i, i + 1
.endr
line_hi:
.set i, 0
.rept LINES
    .byte (STAGE >> 8) + i / 6
.set i, i + 1
.endr
; TA, DL, DF, TURBO, AOFS, SYNC (layout::painter::DEFAULTS): DL/DF calibrated on the C64U (2026-11-04:
; both bands on x 32..351 like the 40-column window, every pixel 2 dots, in phase with the window's).
defaults:
    .byte 37, 73, 2, $8f, 26, 100
; Block T's lines are the set's first 800 bytes, block B's the next: + $0320.
block_lo:
    .byte $00, $c0
block_mid:
    .byte $00, $03
; layout::BUFFERS' $DD00 bits.
dd00_bits:
    .byte 2, 1
