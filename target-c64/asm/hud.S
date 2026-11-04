; hud_irq — the HUD sprite multiplexer (raster IRQ). Same contract as
; talonhd::hud::Table at HUD_TABLE:
;   +0       ROW    next row to set up (owned by this handler)
;   +1..+8   LINES  raster line of the IRQ that sets up each row
;   +9..+16  YS     sprite Y of each row
;   +17..    PTRS   8 pointers per row
;
; Each run writes row ROW's Y into all 8 sprites and its 8 pointers into both
; buffers' pointer slots: the table value into bank 2's ($8FF8, blocks at
; $8000) and the value | $40 into bank 1's ($4FF8, blocks at $5000). Then it
; arms the IRQ for the next
; row (wrapping to row 0, which is set up in the top border).
;
; Touches only A, X, Y (saved), $01 (saved), VIC registers, the pointer slots
; and ROW: no zero page, no REU, no Rust state, so it may interrupt anything,
; including draw_column_asm (which owns $F0-$FF) and the game step, which runs
; with I/O banked out ($01 = $34, code in the RAM under $D000): the handler
; banks I/O in for its own writes and restores $01 on the way out.

HUD      = $CB00
H_ROW    = HUD + 0
H_LINES  = HUD + 1
H_YS     = HUD + 9
H_PTRS   = HUD + 17
ROWS     = 8

; The border painter's chain (asm/border.S; P_ON = 0: the plain HUD chain above). After row 0 (@20) the
; DMA gate closes (@27) and block T paints (@30, a line before its first painted line, 31); after row 7
; the gate closes (@243) and block B paints (@246; its first line is 247). A DMA started before a gate is done a
; line before its block; each block opens the gate when it is done. P_NEXT: 0 a HUD row, 1 the gate
; before T, 2 the gate before B, 3 block B, 4 block T.
PB       = $CAE0
P_ON     = PB + 0
P_GATE   = PB + 1
P_NEXT   = PB + 2
GATE_T   = 27
T_LINE   = 30
GATE_B   = 243
B_LINE   = 246

.section .text.hud_irq,"ax",@progbits
.globl hud_irq
.type hud_irq,@function
hud_irq:
    pha
    txa
    pha
    tya
    pha
    lda $01
    pha
    lda #$35
    sta $01             ; I/O in (the game step may have banked it out)
    jsr irq_clock_in
    lda #$01
    sta $d019           ; acknowledge the raster interrupt

    lda P_NEXT
    bne 90f
    jmp row
90: cmp #3
    bne 91f
    jmp blk_b
91: cmp #4
    bne 92f
    jmp blk_t
92: tax                 ; 1 or 2: no DMA may start before the block
    lda #$80
    sta P_GATE
    cpx #1
    bne 1f
    lda #4
    sta P_NEXT
    lda #T_LINE
    sta $d012
    jmp out
1:  lda #3
    sta P_NEXT
    lda #B_LINE
    sta $d012
    jmp out
blk_t:
    jsr border_t        ; block T, then row 1
    lda #0
    sta P_NEXT
    ldx H_ROW
    lda H_LINES,x
    sta $d012
    jmp out
blk_b:
    jsr border_b        ; block B, then row 0
    lda #0
    sta P_NEXT
    lda H_LINES
    sta $d012
    jmp out

row:
    ldx H_ROW
    lda H_YS,x
    sta $d001
    sta $d003
    sta $d005
    sta $d007
    sta $d009
    sta $d00b
    sta $d00d
    sta $d00f

    txa
    asl a
    asl a
    asl a
    tay                 ; Y = row * 8
    lda H_PTRS+0,y
    sta $8ff8
    ora #$40
    sta $4ff8
    lda H_PTRS+1,y
    sta $8ff9
    ora #$40
    sta $4ff9
    lda H_PTRS+2,y
    sta $8ffa
    ora #$40
    sta $4ffa
    lda H_PTRS+3,y
    sta $8ffb
    ora #$40
    sta $4ffb
    lda H_PTRS+4,y
    sta $8ffc
    ora #$40
    sta $4ffc
    lda H_PTRS+5,y
    sta $8ffd
    ora #$40
    sta $4ffd
    lda H_PTRS+6,y
    sta $8ffe
    ora #$40
    sta $4ffe
    lda H_PTRS+7,y
    sta $8fff
    ora #$40
    sta $4fff

    inx
    cpx #ROWS
    bne 1f
    ldx #0
1:
    stx H_ROW
    lda P_ON
    beq 3f
    cpx #1
    bne 4f
    lda #1              ; row 0 done: the gate, then block T
    sta P_NEXT
    lda #GATE_T
    sta $d012
    jmp out
4:  cpx #0
    bne 3f
    lda #2              ; row 7 done: the gate, then block B
    sta P_NEXT
    lda #GATE_B
    sta $d012
    jmp out
3:  lda H_LINES,x
    sta $d012

out:
    jsr irq_clock_out
    pla
    sta $01
    pla
    tay
    pla
    tax
    pla
    rti
.size hud_irq, .-hud_irq

; The IRQ clock (layout::status::IRQ_US): every raster IRQ's microseconds (CIA 2 timer A, free-running,
; counting down) added to a free-running u16, so the main loop's timings can leave the IRQs' time out.
IRQ_T0  = $CADA     ; (free bytes of the transpose job, layout::IRQ_T0)
IRQ_US  = $CA80 + 24
.section .ptext.irq_clock,"ax",@progbits
irq_clock_in:
    jsr irq_now
    sta 1f + 1          ; T0 into the operands below
    stx 2f + 1
    rts
irq_clock_out:
    jsr irq_now
    sta IRQ_T0 + 1
    stx IRQ_T0
2:  lda #0              ; IRQ_US += T0 - now
    sec
    sbc IRQ_T0
    tax
1:  lda #0
    sbc IRQ_T0 + 1
    tay
    txa
    clc
    adc IRQ_US
    sta IRQ_US
    tya
    adc IRQ_US + 1
    sta IRQ_US + 1
    rts
.section .uptext.irq_now,"ax",@progbits
irq_now:                ; CIA 2 timer A: A = high, X = low (read until stable)
    lda $dd05
    ldx $dd04
    cmp $dd05
    bne irq_now
    rts

; Enable interrupts (the raster IRQ chain is armed by the caller).
.section .text.hud_irq_enable,"ax",@progbits
.globl hud_irq_enable
.type hud_irq_enable,@function
hud_irq_enable:
    cli
    rts
.size hud_irq_enable, .-hud_irq_enable

; Gate both sampler voices on the same tick (audio.rs: the right voice plays the ring 10 ms behind the
; left). Called with I/O banked in.
.section .text.audio_gate,"ax",@progbits
.globl audio_gate
.type audio_gate,@function
audio_gate:
    sei
    lda #3
    sta $df20
    sta $df40
    cli
    rts
.size audio_gate, .-audio_gate
