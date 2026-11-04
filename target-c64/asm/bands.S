; The border bands laid out line-major (render::transpose_row): the painter's lines come from 4 cell rows
; per block (band rows and the bitmap's hidden lines), transposed here and stashed from line `fine`.

SH_A   = $CAD8
SH_DST = $CADC

; The border bands laid out line-major for the painter (render::transpose_row, its reference): one
; 320-byte cell row at SH_A to SH_DST, line l's 40 bytes (one per cell) at SH_DST + 40 l. Same shape as
; shift_row_asm: 8 load/store pairs patched per line, cells 0..31 then (sources a page up) 32..39; X is
; the cell. About 10 cycles a byte. BANDS phase.
.section .text.transpose_row_asm,"ax",@progbits
.globl transpose_row_asm
.type transpose_row_asm,@function
transpose_row_asm:
    ldx #7
1:  ldy tr_off,x
    txa
    clc
    adc SH_A            ; A + l
    sta tr_copy+1,y
    lda SH_A+1
    adc #0
    sta tr_copy+2,y
    lda tr_mul40,x
    clc
    adc SH_DST          ; DST + 40 l
    sta tr_copy+4,y
    lda SH_DST+1
    adc tr_mul40h,x
    sta tr_copy+5,y
    dex
    bpl 1b
    lda #0              ; cells 0..31: until Y wraps
    sta tr_end+1
    tax
    tay
    jsr tr_copy
    ldy #7              ; the sources a page up (the destinations go on with X)
4:  ldx tr_off,y
    inc tr_copy+2,x
    dey
    bpl 4b
    lda #64             ; cells 32..39
    sta tr_end+1
    ldx #32
    ldy #0
    jmp tr_copy

tr_copy:
.rept 8
    lda $ffff,y
    sta $ffff,x
.endr
    inx
    tya
    clc
    adc #8
    tay
tr_end:
    cpy #0
    bne tr_copy
    rts

tr_off:
    .byte 0, 6, 12, 18, 24, 30, 36, 42
tr_mul40:
    .byte 0, 40, 80, 120, 160, 200, 240, 280 & 255
tr_mul40h:
    .byte 0, 0, 0, 0, 0, 0, 0, 280 >> 8
.size transpose_row_asm, . - transpose_row_asm
