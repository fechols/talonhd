; draw_cols_asm — the sprite kernel. Same contract as the Rust reference `talonhd::render::draw_cols`:
; draw NCOLS consecutive column records from RAM at SRC into the bitmap, the first at DST (its top
; cell), each next one 8 bytes right; sprite line l lands on screen line Y + l, lines clipped to
; [TOP, BOT).
;
; Inputs (JOB block, little-endian):
;   +0  DST    bitmap address of the first column's top cell
;   +2  Y      screen y of sprite line 0 (i16)
;   +4  TOP    first visible sprite line
;   +6  BOT    end of visible sprite lines
;   +8  SRC    RAM address of the first column record
;   +10 NCOLS  records to draw (u8)
; Record (talonhd::format): item count, then items in line order:
;   Run  line lo, line hi (bit 0), length (1..255), then the bytes (fully opaque)
;   Mix  line lo, line hi | $80, bits, keep (dst = dst & keep | bits)
;
; No REU access: the caller fetched the records (one DMA per band chunk). On the C64U an I/O write costs
; 16 RAM stores, so the bytes are scattered by the CPU: a run is copied a cell row (<= 8 bytes, adjacent
; in the bitmap) at a time.
;
; Clobbers A, X, Y and zero page $F0-$FF (outside the compiler's ZP range).
;
; Beware: the assembler does not reject out-of-range branches; it emits a wrong target. tools/test_blit.py
; runs this kernel against the reference.

JOB      = $CA00
J_DST    = JOB + 0
J_Y      = JOB + 2
J_TOP    = JOB + 4
J_BOT    = JOB + 6
J_SRC    = JOB + 8
J_NCOLS  = JOB + 10

zp_src   = $F0     ; 2: record read pointer
zp_dst   = $F2     ; 2: this column's top cell
zp_cols  = $F4     ; 1: columns left
zp_items = $F5     ; 1: items left in this column
zp_line  = $F6     ; 2: item line
zp_ptr   = $F8     ; 2: bitmap write pointer
zp_from  = $FA     ; 2: run byte read pointer
zp_n     = $FC     ; 1: bytes in this cell row
zp_rem   = $FD     ; 1: bytes left in the run
zp_sl    = $FE     ; 1: screen line
zp_len   = $FF     ; 1: run length

.section .text.draw_cols_asm,"ax",@progbits
.globl draw_cols_asm
.type draw_cols_asm,@function
draw_cols_asm:
    lda J_SRC
    sta zp_src
    lda J_SRC+1
    sta zp_src+1
    lda J_DST
    sta zp_dst
    lda J_DST+1
    sta zp_dst+1
    lda J_NCOLS
    bne column
    rts

column:
    sta zp_cols
    ldy #0
    lda (zp_src),y
    sta zp_items
    inc zp_src
    bne item
    inc zp_src+1

item:
    lda zp_items
    bne 1f
    jmp next_column     ; out of branch range
1:  dec zp_items
    ldy #0
    lda (zp_src),y
    sta zp_line
    iny
    lda (zp_src),y
    bpl run
    jmp mix             ; out of branch range

    ; --- run: copy bytes [max(line, TOP), min(line + len, BOT)) ---
run:
    and #1
    sta zp_line+1
    iny
    lda (zp_src),y
    sta zp_len
    ; e = line + len, clamped to BOT (16-bit), kept in zp_ptr for now
    clc
    lda zp_line
    adc zp_len
    sta zp_ptr
    lda zp_line+1
    adc #0
    sta zp_ptr+1
    lda zp_ptr
    cmp J_BOT
    lda zp_ptr+1
    sbc J_BOT+1
    bcc 2f
    lda J_BOT
    sta zp_ptr
    lda J_BOT+1
    sta zp_ptr+1
2:  ; s = max(line, TOP), kept in zp_from for now
    lda zp_line
    cmp J_TOP
    lda zp_line+1
    sbc J_TOP+1
    bcs 3f
    lda J_TOP
    sta zp_from
    lda J_TOP+1
    sta zp_from+1
    jmp 4f
3:  lda zp_line
    sta zp_from
    lda zp_line+1
    sta zp_from+1
4:  ; nothing visible when s >= e
    lda zp_from
    cmp zp_ptr
    lda zp_from+1
    sbc zp_ptr+1
    bcc 90f
    jmp run_done
90:
    sec
    lda zp_ptr
    sbc zp_from
    sta zp_rem
    ; sl = Y + s
    clc
    lda J_Y
    adc zp_from
    sta zp_sl
    ; from = src + (s - line) + 3   (s - line fits a byte)
    sec
    lda zp_from
    sbc zp_line
    clc
    adc zp_src
    sta zp_from
    lda zp_src+1
    adc #0
    sta zp_from+1
    clc
    lda zp_from
    adc #3
    sta zp_from
    bcc 1f
    inc zp_from+1
1:

    ; y indexes the run's bytes for the whole run; each cell row's bitmap pointer is biased by -y so
    ; (zp_ptr),y lands on it while y keeps counting (no source pointer update per row).
    ldy #0
cell:
    ; n = min(8 - (sl & 7), rem)
    lda zp_sl
    and #7
    eor #$ff
    sec
    adc #8
    cmp zp_rem
    bcc 5f
    lda zp_rem
5:  sta zp_n
    ; ptr = dst + line offset(sl) - y   (zp_line is free now: a scratch word)
    sty zp_line
    ldx zp_sl
    clc
    lda zp_dst
    adc line_lo,x
    sta zp_ptr
    lda zp_dst+1
    adc line_hi,x
    sta zp_ptr+1
    sec
    lda zp_ptr
    sbc zp_line
    sta zp_ptr
    bcs 6f
    dec zp_ptr+1
6:  ; copy n bytes: enter the unrolled 8 at 8 - n (Duff's device)
    lda zp_n
    asl a
    tax
    lda copy_entry,x
    sta zp_line
    lda copy_entry+1,x
    sta zp_line+1
    jmp (zp_line)
copy8:
    .rept 8
    lda (zp_from),y
    sta (zp_ptr),y
    iny
    .endr
copied:
    ; sl += n, rem -= n
    clc
    lda zp_sl
    adc zp_n
    sta zp_sl
    sec
    lda zp_rem
    sbc zp_n
    sta zp_rem
    bne cell
run_done:
    ; src += 3 + len
    clc
    lda zp_src
    adc #3
    bcc 8f
    inc zp_src+1
    clc
8:  adc zp_len
    sta zp_src
    bcc 9f
    inc zp_src+1
9:  jmp item

    ; --- mix: blend one byte if its line is visible ---
mix:
    and #1
    sta zp_line+1
    lda zp_line
    cmp J_TOP
    lda zp_line+1
    sbc J_TOP+1
    bcc mix_done
    lda zp_line
    cmp J_BOT
    lda zp_line+1
    sbc J_BOT+1
    bcs mix_done
    clc
    lda J_Y
    adc zp_line
    tax
    clc
    lda zp_dst
    adc line_lo,x
    sta zp_ptr
    lda zp_dst+1
    adc line_hi,x
    sta zp_ptr+1
    ldy #3
    lda (zp_src),y
    ldy #0
    and (zp_ptr),y
    ldy #2
    ora (zp_src),y
    ldy #0
    sta (zp_ptr),y
mix_done:
    clc
    lda zp_src
    adc #4
    sta zp_src
    bcc 1f
    inc zp_src+1
1:  jmp item

next_column:
    clc
    lda zp_dst
    adc #8
    sta zp_dst
    bcc 1f
    inc zp_dst+1
1:  dec zp_cols
    beq 2f
    lda zp_cols
    jmp column
2:  rts
.size draw_cols_asm, .-draw_cols_asm

; draw_cols_asm's unrolled copy: the entry for n bytes, n = 0..8 (each step is 5 bytes: lda (zp),y / sta (zp),y / iny).
.section .rodata.copy_entry,"a",@progbits
copy_entry:
    .word copied, copy8 + 35, copy8 + 30, copy8 + 25, copy8 + 20, copy8 + 15, copy8 + 10, copy8 + 5, copy8
; draw_sprite_asm — one whole sprite. Same contract as the Rust reference `talonhd::render::draw_sprite`
; (the host model runs that; tools/test_blit.py runs both on the baked image).
;
; Inputs (SP block, little-endian): frame (u16, < image frames), x, y (i16, a line of the target), the
; target's address (the back buffer's bitmap, or a band's rows: cell rows 320 bytes apart), the frame
; directory's REU address (u24), and (SP_HEIGHT) the target's lines (200, or a band's 24). Leaves the
; frame's height in SP_H (the caller decides from it which other targets the sprite reaches).
;
; Small one-band blocks come from the RAM cache (layout::SPRITE_CACHE: 8 slots of $82 bytes, picked by
; (frame * 4 + x & 3) & 7; tag frame + 1, x phase, byte columns, height, then the block). Otherwise: the
; frame record, then per visible band its on-screen column offsets and the records, a CHUNK at a time,
; each drawn by draw_cols_asm. REU fetches are chained 256-byte DMAs (layout::MAX_DMA: the HUD raster IRQ
; must not wait longer).

SP_FRAME  = $CAA0
SP_X      = $CAA2
SP_Y      = $CAA4
SP_BITMAP = $CAA6
SP_DIR    = $CAA8
SP_XP     = $CAAB
SP_SLOT   = $CAAC
SP_WB     = $CAAE
SP_H      = $CAAF
SP_BLOCK  = $CAB1
SP_SHIFT  = $CAB4
SP_BASE   = $CAB5
SP_J0     = $CAB7
SP_NCOLS  = $CAB8
SP_DST    = $CAB9
SP_NB     = $CABB
SP_B      = $CABC
SP_BAND   = $CABD
SP_J      = $CAC0
SP_K      = $CAC1
SP_START  = $CAC2
SP_LEN    = $CAC4
SP_CACHED = $CAC6
SP_TAG    = $CAC7
SP_TOP    = $CAC9
SP_BOT    = $CACB
SP_T      = $CACD
RF_REU    = $CAD0
RF_C64    = $CAD3
RF_LEN    = $CAD5
P_GATE    = $CAE1
REU_DIRTY = $CADE     ; layout::REU_DIRTY (asm/reu.S)
SP_HEIGHT = $CAD7

RECORD    = $CA20
CHUNK     = $C000
CHUNK_LEN = $0700
OFFS      = $C700
SLOT_META = 8
SLOT_MAX  = $82 - SLOT_META

zp_p      = $F0     ; 2: a pointer (free outside draw_cols_asm)

.section .text.draw_sprite_asm,"ax",@progbits
.globl draw_sprite_asm
.type draw_sprite_asm,@function
draw_sprite_asm:
    ; --- the cache slot and tag ---
    lda SP_X
    and #3
    sta SP_XP
    lda SP_FRAME
    and #1
    asl a
    asl a
    ora SP_XP
    asl a
    tax
    lda slot_addr,x
    sta SP_SLOT
    sta zp_p
    lda slot_addr+1,x
    sta SP_SLOT+1
    sta zp_p+1
    clc
    lda SP_FRAME
    adc #1
    sta SP_TAG
    lda SP_FRAME+1
    adc #0
    sta SP_TAG+1
    ldy #0
    lda (zp_p),y
    cmp SP_TAG
    beq 90f
    jmp miss
90:
    iny
    lda (zp_p),y
    cmp SP_TAG+1
    beq 90f
    jmp miss
90:
    iny
    lda (zp_p),y
    cmp SP_XP
    beq 90f
    jmp miss
90:
    iny
    lda (zp_p),y
    sta SP_WB
    iny
    lda (zp_p),y
    sta SP_H
    iny
    lda (zp_p),y
    sta SP_H+1
    lda #1
    sta SP_CACHED
    jmp clip

miss:
    ; --- the frame record: REU dir + frame * 32 -> RECORD ---
    lda SP_FRAME
    sta SP_T
    lda SP_FRAME+1
    sta SP_T+1
    lda #0
    sta SP_T+2
    ldx #5
1:  asl SP_T
    rol SP_T+1
    rol SP_T+2
    dex
    bne 1b
    clc
    lda SP_DIR
    adc SP_T
    sta RF_REU
    lda SP_DIR+1
    adc SP_T+1
    sta RF_REU+1
    lda SP_DIR+2
    adc SP_T+2
    sta RF_REU+2
    lda #<RECORD
    sta RF_C64
    lda #>RECORD
    sta RF_C64+1
    lda #32
    sta RF_LEN
    lda #0
    sta RF_LEN+1
    jsr reu_fetch
    ; phase record: 8 + 6 p (p = 0 for a 1-phase frame)
    ldx #8
    lda RECORD+4
    cmp #1
    beq 2f
    ldy SP_XP
    ldx phase_off,y
2:  lda RECORD,x
    sta SP_WB
    lda RECORD+1,x
    sta SP_BLOCK
    lda RECORD+2,x
    sta SP_BLOCK+1
    lda RECORD+3,x
    sta SP_BLOCK+2
    lda RECORD+4,x
    sta SP_LEN
    lda RECORD+5,x
    sta SP_LEN+1
    lda RECORD+2
    sta SP_H
    lda RECORD+3
    sta SP_H+1
    lda RECORD+5
    sta SP_SHIFT
    lda #0
    sta SP_CACHED
    ; cacheable: one band, the block fits a slot
    lda RECORD+6
    cmp #1
    beq 90f
    jmp clip
90:
    lda SP_LEN+1
    beq 90f
    jmp clip
90:
    lda SP_LEN
    cmp #SLOT_MAX + 1
    bcc 90f
    jmp clip
90:
    lda SP_BLOCK
    sta RF_REU
    lda SP_BLOCK+1
    sta RF_REU+1
    lda SP_BLOCK+2
    sta RF_REU+2
    clc
    lda SP_SLOT
    adc #SLOT_META
    sta RF_C64
    lda SP_SLOT+1
    adc #0
    sta RF_C64+1
    lda SP_LEN
    sta RF_LEN
    lda #0
    sta RF_LEN+1
    jsr reu_fetch
    lda SP_SLOT
    sta zp_p
    lda SP_SLOT+1
    sta zp_p+1
    ldy #0
    lda SP_TAG
    sta (zp_p),y
    iny
    lda SP_TAG+1
    sta (zp_p),y
    iny
    lda SP_XP
    sta (zp_p),y
    iny
    lda SP_WB
    sta (zp_p),y
    iny
    lda SP_H
    sta (zp_p),y
    iny
    lda SP_H+1
    sta (zp_p),y
    lda #1
    sta SP_CACHED

clip:
    ; base = x >> 2 (arithmetic)
    lda SP_X
    sta SP_BASE
    lda SP_X+1
    sta SP_BASE+1
    ldx #2
3:  lda SP_BASE+1
    cmp #$80
    ror SP_BASE+1
    ror SP_BASE
    dex
    bne 3b
    ; j0 = base < 0 ? -base : 0; nothing when j0 >= wb
    lda #0
    sta SP_J0
    lda SP_BASE+1
    bpl 4f
    sec
    lda #0
    sbc SP_BASE
    sta SP_J0
    lda #0
    sbc SP_BASE+1
    beq 90f
    jmp done
90:
    lda SP_J0
    cmp SP_WB
    bcc 90f
    jmp done
90:
4:  ; t = 40 - base; nothing when t <= 0; j1 = min(wb, t); ncols = j1 - j0
    sec
    lda #40
    sbc SP_BASE
    sta SP_T
    lda #0
    sbc SP_BASE+1
    bpl 90f
    jmp done
90:
    bne 5f              ; t > 255: j1 = wb
    lda SP_T
    bne 90f
    jmp done
90:
    cmp SP_WB
    bcc 6f
5:  lda SP_WB
6:  sec
    sbc SP_J0
    bne 90f
    jmp done
90:
    bcs 90f
    jmp done
90:
    sta SP_NCOLS
    ; top = y < 0 ? -y : 0
    lda #0
    sta SP_TOP
    sta SP_TOP+1
    lda SP_Y+1
    bpl 7f
    sec
    lda #0
    sbc SP_Y
    sta SP_TOP
    lda #0
    sbc SP_Y+1
    sta SP_TOP+1
7:  ; bot = min(h, height - y) (signed)
    sec
    lda SP_HEIGHT
    sbc SP_Y
    sta SP_BOT
    lda #0
    sbc SP_Y+1
    sta SP_BOT+1
    bpl 90f
    jmp done
90:
    lda SP_BOT
    cmp SP_H
    lda SP_BOT+1
    sbc SP_H+1
    bcc 8f
    lda SP_H
    sta SP_BOT
    lda SP_H+1
    sta SP_BOT+1
8:  ; nothing when top >= bot
    lda SP_TOP
    cmp SP_BOT
    lda SP_TOP+1
    sbc SP_BOT+1
    bcc 9f
done:
    rts
9:  ; the kernel's job: Y, TOP, BOT; dst = bitmap + (base + j0) * 8
    lda SP_Y
    sta $CA02
    lda SP_Y+1
    sta $CA03
    lda SP_TOP
    sta $CA04
    lda SP_TOP+1
    sta $CA05
    lda SP_BOT
    sta $CA06
    lda SP_BOT+1
    sta $CA07
    clc
    lda SP_BASE
    adc SP_J0
    sta SP_DST
    lda #0
    asl SP_DST
    rol a
    asl SP_DST
    rol a
    asl SP_DST
    rol a
    sta SP_DST+1
    clc
    lda SP_DST
    adc SP_BITMAP
    sta SP_DST
    lda SP_DST+1
    adc SP_BITMAP+1
    sta SP_DST+1
    lda SP_CACHED
    bne 90f
    jmp big
90:
    clc
    lda SP_SLOT
    adc #SLOT_META
    sta zp_p
    lda SP_SLOT+1
    adc #0
    sta zp_p+1
    ldy #0
    clc
    lda (zp_p),y
    adc zp_p
    tax
    iny
    lda (zp_p),y
    adc zp_p+1
    stx zp_p
    sta zp_p+1
    lda SP_J0
    asl a
    tay
    clc
    lda (zp_p),y
    adc zp_p
    sta $CA08
    iny
    lda (zp_p),y
    adc zp_p+1
    sta $CA09
    lda SP_DST
    sta $CA00
    lda SP_DST+1
    sta $CA01
    lda SP_NCOLS
    sta $CA0A
    jmp draw_cols_asm

big:
    ; bands b0 = top >> shift .. b1 = (bot - 1) >> shift
    lda SP_TOP
    sta SP_T
    lda SP_TOP+1
    sta SP_T+1
    sec
    lda SP_BOT
    sbc #1
    sta SP_T+2
    lda SP_BOT+1
    sbc #0
    sta SP_T+3
    ldx SP_SHIFT
10: lsr SP_T+1
    ror SP_T
    lsr SP_T+3
    ror SP_T+2
    dex
    bne 10b
    ; nb = b1 - b0 + 1; band starts of b0.. -> RECORD (3 nb bytes)
    sec
    lda SP_T+2
    sbc SP_T
    clc
    adc #1
    sta SP_NB
    lda SP_T
    asl a
    adc SP_T            ; 3 b0 (b0 < 32)
    clc
    adc SP_BLOCK
    sta RF_REU
    lda SP_BLOCK+1
    adc #0
    sta RF_REU+1
    lda SP_BLOCK+2
    adc #0
    sta RF_REU+2
    lda #<RECORD
    sta RF_C64
    lda #>RECORD
    sta RF_C64+1
    lda SP_NB
    asl a
    adc SP_NB
    sta RF_LEN
    lda #0
    sta RF_LEN+1
    jsr reu_fetch
    lda #0
    sta SP_B
band:
    ; band = block + start[b]
    lda SP_B
    asl a
    adc SP_B
    tax
    clc
    lda SP_BLOCK
    adc RECORD,x
    sta SP_BAND
    lda SP_BLOCK+1
    adc RECORD+1,x
    sta SP_BAND+1
    lda SP_BLOCK+2
    adc RECORD+2,x
    sta SP_BAND+2
    ; its offsets j0 .. j0 + ncols -> OFFS
    lda SP_J0
    asl a
    clc
    adc SP_BAND
    sta RF_REU
    lda SP_BAND+1
    adc #0
    sta RF_REU+1
    lda SP_BAND+2
    adc #0
    sta RF_REU+2
    lda #<OFFS
    sta RF_C64
    lda #>OFFS
    sta RF_C64+1
    ldx SP_NCOLS
    inx
    txa
    asl a
    sta RF_LEN
    lda #0
    rol a
    sta RF_LEN+1
    jsr reu_fetch
    lda #0
    sta SP_J
chunk:
    lda SP_J
    cmp SP_NCOLS
    bcc 11f
    ; next band
    inc SP_B
    lda SP_B
    cmp SP_NB
    bcs 90f
    jmp band
90:
    rts
11: ; start = offs[j]; k = j + 1; grow while k < ncols and offs[k + 1] - start <= CHUNK_LEN
    asl a
    tax
    lda OFFS,x
    sta SP_START
    lda OFFS+1,x
    sta SP_START+1
    ; Fast path: the rest of the band fits the chunk (the usual case: 32 lines of 40 columns).
    lda SP_NCOLS
    asl a
    tax
    sec
    lda OFFS,x
    sbc SP_START
    sta SP_T
    lda OFFS+1,x
    sbc SP_START+1
    sta SP_T+1
    lda #<CHUNK_LEN
    cmp SP_T
    lda #>CHUNK_LEN
    sbc SP_T+1
    bcc 14f
    lda SP_NCOLS
    sta SP_K
    jmp 13f
14: ldx SP_J
    inx
    stx SP_K
12: lda SP_K
    cmp SP_NCOLS
    bcs 13f
    asl a
    tax
    sec
    lda OFFS+2,x
    sbc SP_START
    sta SP_T
    lda OFFS+3,x
    sbc SP_START+1
    sta SP_T+1
    lda #<CHUNK_LEN
    cmp SP_T
    lda #>CHUNK_LEN
    sbc SP_T+1
    bcc 13f             ; CHUNK_LEN < offs[k+1] - start
    inc SP_K
    jmp 12b
13: ; fetch band + start .. band + offs[k] -> CHUNK
    lda SP_K
    asl a
    tax
    sec
    lda OFFS,x
    sbc SP_START
    sta RF_LEN
    lda OFFS+1,x
    sbc SP_START+1
    sta RF_LEN+1
    clc
    lda SP_BAND
    adc SP_START
    sta RF_REU
    lda SP_BAND+1
    adc SP_START+1
    sta RF_REU+1
    lda SP_BAND+2
    adc #0
    sta RF_REU+2
    lda #<CHUNK
    sta RF_C64
    lda #>CHUNK
    sta RF_C64+1
    jsr reu_fetch
    ; draw columns j .. k: DST = dst + 8 j
    lda SP_J
    sta $CA00
    lda #0
    asl $CA00
    rol a
    asl $CA00
    rol a
    asl $CA00
    rol a
    sta $CA01
    clc
    lda $CA00
    adc SP_DST
    sta $CA00
    lda $CA01
    adc SP_DST+1
    sta $CA01
    lda #<CHUNK
    sta $CA08
    lda #>CHUNK
    sta $CA09
    sec
    lda SP_K
    sbc SP_J
    sta $CA0A
    jsr draw_cols_asm
    lda SP_K
    sta SP_J
    jmp chunk
.size draw_sprite_asm, .-draw_sprite_asm

; REU -> C64: RF_LEN bytes from RF_REU to RF_C64 (u24/u16/u16), 256-byte DMAs. The first piece loads every
; REU register, the next ones only the length and the command (the REU's addresses run on), unless
; REU_DIRTY: the border painter's refill loaded them between pieces (asm/reu.S reu_transfer), or the REU
; address crossed 64 KiB.
.section .text.reu_fetch_chain,"ax",@progbits
reu_fetch:
    lda #1
    sta REU_DIRTY
1:  lda RF_LEN+1
    beq 2f
    lda #0
    ldx #1
    jsr reu_piece
    inc RF_C64+1        ; + 256
    inc RF_REU+1
    bne 4f
    inc RF_REU+2
    inc REU_DIRTY
4:  dec RF_LEN+1
    jmp 1b
2:  lda RF_LEN
    beq 3f
    ldx #0
    jmp reu_piece
3:  rts

; One fetch of X:A bytes from RF_REU to RF_C64, after the painter's gate, with interrupts masked.
reu_piece:
97: bit P_GATE          ; the border painter's DMA gate (asm/reu.S)
    bmi 97b
    php
    sei
    sta $df07
    stx $df08
    lda REU_DIRTY
    beq 98f
    lda RF_C64
    sta $df02
    lda RF_C64+1
    sta $df03
    lda RF_REU
    sta $df04
    lda RF_REU+1
    sta $df05
    lda RF_REU+2
    sta $df06
    lda #0
    sta REU_DIRTY
98: lda #$91
    sta $df01
    plp
    rts

.section .rodata.sprite_tables,"a",@progbits
slot_addr:
    .word $E000, $E082, $E104, $E186, $E208, $E28A, $E30C, $E38E
phase_off:
    .byte 8, 14, 20, 26

; Byte offset of screen line y within a bitmap column: (y/8)*320 + (y&7).
.section .rodata.line_tables,"a",@progbits
.globl line_lo
.globl line_hi
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
