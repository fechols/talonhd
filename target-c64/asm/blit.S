; draw_column_asm — the hot column kernel. Same contract as the Rust
; reference `talonhd::render::draw_column` (vendored from gclass `a5775bf`):
;
; Inputs (JOB block, little-endian):
;   +0  DST    back-buffer address of this column's top cell
;   +2  Y      screen y of sprite line 0 (i16)
;   +4  TOP    first visible sprite line
;   +6  BOT    end of visible sprite lines
;   +8  DATA   REU address of the column's pixel bytes for line 0 (u24)
;   +11 COUNT  segments in SEG_BUF
; SEG_BUF: 4-byte segments sorted by line (see talonhd::format).
;
; Opaque runs are clamped to [TOP, BOT) and copied as a chain of DMAs split
; at cell-row boundaries; the REU source address auto-increments between
; links, so each link only rewrites the C64 address and length. Mixed bytes
; are blended: dst = dst & keep | bits.
;
; Clobbers A, X, Y and zero page $F0-$FF (outside the compiler's ZP range).
;
; Beware: the assembler does not reject out-of-range branches; it emits a
; wrong target. tools/test_blit.py runs this kernel against the reference.

JOB      = $CA00
J_DST    = JOB + 0
J_Y      = JOB + 2
J_TOP    = JOB + 4
J_BOT    = JOB + 6
J_DATA   = JOB + 8
J_COUNT  = JOB + 11
SEG_BUF  = $C000

zp_segp  = $F0     ; 2: current segment
zp_cnt   = $F2     ; 2: segments left
zp_line  = $F4     ; 2: segment line
zp_kind  = $F6     ; 1: segment byte 1 (bit 7 = mix)
zp_sl    = $F7     ; 1: screen line
zp_s     = $F8     ; 2: first line to copy
zp_e     = $FA     ; 2: end line
zp_rem   = $FC     ; 1: lines left in run (≤ 200)
zp_n     = $FD     ; 1: lines in this link
zp_ptr   = $FE     ; 2: blend destination

.section .text.draw_column_asm,"ax",@progbits
.globl draw_column_asm
.type draw_column_asm,@function
draw_column_asm:
    lda J_COUNT
    sta zp_cnt
    lda J_COUNT+1
    sta zp_cnt+1
    lda #<SEG_BUF
    sta zp_segp
    lda #>SEG_BUF
    sta zp_segp+1

next_segment:
    lda zp_cnt
    ora zp_cnt+1
    bne 1f
    rts
1:  lda zp_cnt
    bne 2f
    dec zp_cnt+1
2:  dec zp_cnt

    ldy #0
    lda (zp_segp),y
    sta zp_line
    iny
    lda (zp_segp),y
    sta zp_kind
    and #1
    sta zp_line+1
    ; Segments are sorted: stop at the first one at or below BOT.
    lda zp_line
    cmp J_BOT
    lda zp_line+1
    sbc J_BOT+1
    bcc 3f
    rts
3:  bit zp_kind
    bpl run_segment
    jmp mix         ; out of branch range
run_segment:

    ; --- opaque run: e = min(line + len, BOT), s = max(line, TOP) ---
    iny
    lda (zp_segp),y
    clc
    adc zp_line
    sta zp_e
    iny
    lda (zp_segp),y
    adc zp_line+1
    sta zp_e+1
    lda zp_e
    cmp J_BOT
    lda zp_e+1
    sbc J_BOT+1
    bcc 4f
    lda J_BOT
    sta zp_e
    lda J_BOT+1
    sta zp_e+1
4:  lda zp_line
    cmp J_TOP
    lda zp_line+1
    sbc J_TOP+1
    bcs 5f
    lda J_TOP
    sta zp_s
    lda J_TOP+1
    sta zp_s+1
    jmp 6f
5:  lda zp_line
    sta zp_s
    lda zp_line+1
    sta zp_s+1
6:  lda zp_s
    cmp zp_e
    lda zp_s+1
    sbc zp_e+1
    bcs advance
    sec
    lda zp_e
    sbc zp_s
    sta zp_rem
    ; REU source = DATA + s; it then auto-increments along the chain.
    clc
    lda J_DATA
    adc zp_s
    sta $df04
    lda J_DATA+1
    adc zp_s+1
    sta $df05
    lda J_DATA+2
    adc #0
    sta $df06
    lda #0
    sta $df08
    clc
    lda J_Y
    adc zp_s
    sta zp_sl

link:
    ; n = min(8 - (sl & 7), rem)
    lda zp_sl
    and #7
    eor #$ff
    sec
    adc #8          ; 8 - (sl & 7)
    cmp zp_rem
    bcc 7f
    lda zp_rem
7:  sta zp_n
    sta $df07
    ldx zp_sl
    clc
    lda J_DST
    adc line_lo,x
    sta $df02
    lda J_DST+1
    adc line_hi,x
    sta $df03
    lda #$91
    sta $df01
    clc
    lda zp_sl
    adc zp_n
    sta zp_sl
    sec
    lda zp_rem
    sbc zp_n
    sta zp_rem
    bne link

advance:
    clc
    lda zp_segp
    adc #4
    sta zp_segp
    bcc 8f
    inc zp_segp+1
8:  jmp next_segment    ; out of branch range

    ; --- mixed byte: blend if the line is visible ---
mix:
    lda zp_line
    cmp J_TOP
    lda zp_line+1
    sbc J_TOP+1
    bcc advance
    clc
    lda J_Y
    adc zp_line
    tax
    clc
    lda J_DST
    adc line_lo,x
    sta zp_ptr
    lda J_DST+1
    adc line_hi,x
    sta zp_ptr+1
    ldy #3
    lda (zp_segp),y
    ldy #0
    and (zp_ptr),y
    ldy #2
    ora (zp_segp),y
    ldy #0
    sta (zp_ptr),y
    jmp advance
.size draw_column_asm, .-draw_column_asm

; Byte offset of screen line y within a bitmap column: (y/8)*320 + (y&7).
.section .rodata.line_tables,"a",@progbits
line_lo:
.set yy, 0
.rept 200
    .byte ((((yy >> 3) * 320) + (yy & 7)) & 0xff)
.set yy, yy + 1
.endr
line_hi:
.set yy, 0
.rept 200
    .byte ((((yy >> 3) * 320) + (yy & 7)) >> 8)
.set yy, yy + 1
.endr
