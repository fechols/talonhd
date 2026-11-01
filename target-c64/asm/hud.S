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
    lda #$01
    sta $d019           ; acknowledge the raster interrupt

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
    lda H_LINES,x
    sta $d012

    pla
    sta $01
    pla
    tay
    pla
    tax
    pla
    rti
.size hud_irq, .-hud_irq

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
