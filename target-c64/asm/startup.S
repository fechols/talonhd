; Runs before the CRT: take the machine over for good. No ROM, no IRQs.
.section .init.000,"ax",@progbits
.globl talonhd_boot
talonhd_boot:
    sei
    cld
    lda #$2f
    sta $00
    lda #$7f
    sta $dc0d
    sta $dd0d
    lda $dc0d
    lda $dd0d
    lda #0
    sta $d01a
    sta $d015
    lda #$ff
    sta $d019
    ; ROM is still mapped; these stores land in the RAM underneath.
    lda #<talonhd_ignore_interrupt
    sta $fffa
    sta $fffe
    lda #>talonhd_ignore_interrupt
    sta $fffb
    sta $ffff
    lda #$35
    sta $01
    ; The border painter's block ($CAE0, layout::PAINTER) before any REU DMA: RAM keeps whatever it held
    ; across a reset, and a stale P_GATE with bit 7 set would hold every DMA (asm/reu.S) for good.
    lda #0
    ldx #31
talonhd_clear_painter:
    sta $cae0,x
    dex
    bpl talonhd_clear_painter
    ; Copy the relocated code (.hitext, .lowtext, .uptext, .iotext, .vtext, .ptext, .p2text) from its load image ($4C00..., inside
    ; frame buffer 0's bank) to where it runs. Whole pages: every destination region is page-aligned
    ; and a whole number of pages long, so the tail of a last page is harmless padding.
    lda #<__hitext_load_start
    sta talonhd_hitext_src+1
    lda #>__hitext_load_start
    sta talonhd_hitext_src+2
    ldx #<__hitext_pages
    beq 3f
    ldy #0
1:
talonhd_hitext_src:
    lda $ffff,y
talonhd_hitext_dst:
    sta __hitext_start,y
    iny
    bne 1b
    inc talonhd_hitext_src+2
    inc talonhd_hitext_dst+2
    dex
    bne 1b
3:
    lda #<__lowtext_load_start
    sta talonhd_lowtext_src+1
    lda #>__lowtext_load_start
    sta talonhd_lowtext_src+2
    ldx #<__lowtext_pages
    beq 3f
    ldy #0
1:
talonhd_lowtext_src:
    lda $ffff,y
talonhd_lowtext_dst:
    sta __lowtext_start,y
    iny
    bne 1b
    inc talonhd_lowtext_src+2
    inc talonhd_lowtext_dst+2
    dex
    bne 1b
3:
    lda #<__uptext_load_start
    sta talonhd_uptext_src+1
    lda #>__uptext_load_start
    sta talonhd_uptext_src+2
    ldx #<__uptext_pages
    beq 3f
    ldy #0
1:
talonhd_uptext_src:
    lda $ffff,y
talonhd_uptext_dst:
    sta __uptext_start,y
    iny
    bne 1b
    inc talonhd_uptext_src+2
    inc talonhd_uptext_dst+2
    dex
    bne 1b
3:
    lda #$34
    sta $01             ; I/O out: .iotext lives in the RAM under it
    lda #<__iotext_load_start
    sta talonhd_iotext_src+1
    lda #>__iotext_load_start
    sta talonhd_iotext_src+2
    ldx #<__iotext_pages
    beq 3f
    ldy #0
1:
talonhd_iotext_src:
    lda $ffff,y
talonhd_iotext_dst:
    sta __iotext_start,y
    iny
    bne 1b
    inc talonhd_iotext_src+2
    inc talonhd_iotext_dst+2
    dex
    bne 1b
3:
    lda #$35
    sta $01
    lda #<__vtext_load_start
    sta talonhd_vtext_src+1
    lda #>__vtext_load_start
    sta talonhd_vtext_src+2
    ldx #<__vtext_pages
    beq 3f
    ldy #0
1:
talonhd_vtext_src:
    lda $ffff,y
talonhd_vtext_dst:
    sta __vtext_start,y
    iny
    bne 1b
    inc talonhd_vtext_src+2
    inc talonhd_vtext_dst+2
    dex
    bne 1b
3:
    lda #<__ptext_load_start
    sta talonhd_ptext_src+1
    lda #>__ptext_load_start
    sta talonhd_ptext_src+2
    ldx #<__ptext_pages
    beq 3f
    ldy #0
1:
talonhd_ptext_src:
    lda $ffff,y
talonhd_ptext_dst:
    sta __ptext_start,y
    iny
    bne 1b
    inc talonhd_ptext_src+2
    inc talonhd_ptext_dst+2
    dex
    bne 1b
3:
    lda #<__p2text_load_start
    sta talonhd_p2text_src+1
    lda #>__p2text_load_start
    sta talonhd_p2text_src+2
    ldx #<__p2text_pages
    beq 3f
    ldy #0
1:
talonhd_p2text_src:
    lda $ffff,y
talonhd_p2text_dst:
    sta __p2text_start,y
    iny
    bne 1b
    inc talonhd_p2text_src+2
    inc talonhd_p2text_dst+2
    dex
    bne 1b
3:
    nop                 ; (the falls-through point of the copies; a label at a section's end confuses the branch audit)

.section .text.talonhd_ignore_interrupt,"ax",@progbits
talonhd_ignore_interrupt:
    rti
