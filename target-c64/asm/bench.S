; Unit-cost microbenchmarks for the renderer (feature `bench`, main.rs `bench`): each routine repeats
; one primitive REPS times (16-bit at $F0/$F1), unrolled 8x where the primitive is a single instruction
; so the loop's own cost stays small; `bench_empty` measures that loop alone. Timing is the caller's
; (CIA 2 timer A, 1 us ticks).
;
; Params (zero page, the kernel's scratch, free during the benchmark):
;   $F0/$F1 reps   $F2/$F3 length (DMA)   $F4/$F5 pointer (stores, blends)
; The I/O-write and RAM-store tests write harmless locations: REU register $DF04 (re-set before every
; DMA anyway) and $C000+ (SEG_BUF).

REPS = $F0
LEN  = $F2
PTR  = $F4

.section .text.bench,"ax",@progbits

.macro countdown
    lda REPS
    bne 1f
    dec REPS+1
1:  dec REPS
    lda REPS
    ora REPS+1
.endm

; The loop alone.
.globl bench_empty
bench_empty:
0:  countdown
    bne 0b
    rts

; One whole REU fetch (REU $000000 -> $C000, LEN bytes): 8 register writes + the command, as
; hw::transfer issues it.
.globl bench_dma
bench_dma:
0:  lda #0
    sta $df04
    sta $df05
    sta $df06
    lda #$00
    sta $df02
    lda #$C0
    sta $df03
    lda LEN
    sta $df07
    lda LEN+1
    sta $df08
    lda #$91
    sta $df01
    countdown
    bne 0b
    rts

; One chained link as the column kernel issues it: length, C64 address, command (4 writes), 8 bytes.
.globl bench_link
bench_link:
    lda #0
    sta $df04
    sta $df05
    sta $df06
    sta $df08
0:  lda #8
    sta $df07
    lda #$00
    sta $df02
    lda #$C0
    sta $df03
    lda #$91
    sta $df01
    countdown
    bne 0b
    rts

; 8 I/O register writes.
.globl bench_io
bench_io:
    lda #0
0:  sta $df04
    sta $df04
    sta $df04
    sta $df04
    sta $df04
    sta $df04
    sta $df04
    sta $df04
    countdown
    bne 0b
    rts

; 8 RAM stores, absolute indexed.
.globl bench_sta
bench_sta:
    ldx #0
    lda #$55
0:  sta $C000,x
    sta $C001,x
    sta $C002,x
    sta $C003,x
    sta $C004,x
    sta $C005,x
    sta $C006,x
    sta $C007,x
    countdown
    bne 0b
    rts

; 8 byte copies, RAM to RAM (lda abs,x / sta abs,y).
.globl bench_copy
bench_copy:
    ldx #0
    ldy #0
0:  lda $C100,x
    sta $C000,y
    lda $C101,x
    sta $C001,y
    lda $C102,x
    sta $C002,y
    lda $C103,x
    sta $C003,y
    lda $C104,x
    sta $C004,y
    lda $C105,x
    sta $C005,y
    lda $C106,x
    sta $C006,y
    lda $C107,x
    sta $C007,y
    countdown
    bne 0b
    rts

; 8 mixed-byte blends through a pointer: dst = dst & keep | bits.
.globl bench_blend
bench_blend:
    lda #$00
    sta PTR
    lda #$C0
    sta PTR+1
0:  .rept 8
    ldy #0
    lda (PTR),y
    and #$0F
    ora #$50
    sta (PTR),y
    .endr
    countdown
    bne 0b
    rts

; 8 bitmap address lookups as the kernel does them (line tables), result discarded.
.globl bench_lookup
bench_lookup:
    ldx #13
0:  .rept 8
    clc
    lda #$00
    adc line_lo,x
    sta PTR
    lda #$60
    adc line_hi,x
    sta PTR+1
    .endr
    countdown
    bne 0b
    rts

; The whole run: every test, timed with CIA 2 timer A (counting down, 1 us), results as u16 us at
; RESULTS (+2 per test), $BE at RESULTS+30 when done. Table entries: routine, reps, DMA length.
RESULTS = $CA60
T0      = $F6
IDX     = $F8
T1      = $FA

.macro test routine, reps, len
    .word \routine - 1
    .word \reps
    .word \len
.endm

.globl bench_all
bench_all:
    lda #0
    sta RESULTS+30
    sta IDX
    sta RESPTR
0:  ldx IDX
    lda table+2,x
    sta REPS
    lda table+3,x
    sta REPS+1
    lda table+4,x
    sta LEN
    lda table+5,x
    sta LEN+1
    ; t0
1:  lda $dd05
    ldy $dd04
    cmp $dd05
    bne 1b
    sty T0
    sta T0+1
    ; call the routine (push its address - 1, rts into it, it returns here)
    lda #>(2f - 1)
    pha
    lda #<(2f - 1)
    pha
    ldx IDX
    lda table+1,x
    pha
    lda table,x
    pha
    rts
2:  ; dt = t0 - t1 (the timer counts down)
3:  lda $dd05
    ldy $dd04
    cmp $dd05
    bne 3b
    sta T1
    ldx RESPTR
    sec
    tya
    eor #$ff
    adc T0        ; t0 + ~t1 + 1 = t0 - t1 (low)
    sta RESULTS,x
    lda T1
    eor #$ff
    adc T0+1
    sta RESULTS+1,x
    inx
    inx
    stx RESPTR
    lda IDX
    clc
    adc #6
    sta IDX
    cmp #(table_end - table)
    bne 0b
    lda #$BE
    sta RESULTS+30
    lda #0
    sta RESPTR
    rts

RESPTR = $F9

.section .rodata.bench_table,"a",@progbits
table:
    test bench_empty, 2048, 0
    test bench_dma, 256, 1
    test bench_dma, 256, 2
    test bench_dma, 256, 4
    test bench_dma, 256, 8
    test bench_dma, 256, 16
    test bench_dma, 256, 64
    test bench_dma, 256, 256
    test bench_dma, 256, 512
    test bench_link, 2048, 0
    test bench_io, 2048, 0
    test bench_sta, 2048, 0
    test bench_copy, 2048, 0
    test bench_blend, 2048, 0
    test bench_lookup, 2048, 0
table_end:
