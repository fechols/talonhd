; Start an REU command. A foreign call, so LLVM must assume any escaped RAM
; changed (a volatile MMIO store alone does not model the DMA's writes).
; Waits while the border painter's gate is closed (asm/border.S P_GATE, bit 7): a DMA stalls the CPU, so
; one started in the lines before a painted block would delay the block's IRQ past its first line.
P_GATE = $CAE1
.section .text.reu_execute,"ax",@progbits
.globl reu_execute
.type reu_execute,@function
reu_execute:
1:  bit P_GATE
    bmi 1b
    sta $df01
    rts
.size reu_execute, .-reu_execute

; The foreground's job block (layout::painter::DMA_JOB, $DF02..$DF08 order: C64 address, REU address,
; length), loaded and triggered with interrupts masked (reu_transfer): an interrupt (the border painter's
; refill) never finds the registers half set, so it needs to save nothing.
DJ = $CAF5


; reu_transfer: hw::transfer on the C64 (Hw::transfer): DJ's c64, REU address and length,
; A = the command, as 256-byte DMAs (MAX_DMA, so the HUD's raster IRQ always lands in its gap), split at
; 512 KiB banks. The first piece loads every REU register; the next ones only the length and the command:
; the REU's addresses run on from one DMA to the next. Unless REU_DIRTY: the border painter's refill set
; it (it loads the registers itself), or a piece ended on a bank boundary. 3 I/O writes a piece instead
; of 8 (each ~1.26 us on the C64U), and no 32-bit Rust per piece.
REU_DIRTY = $CADE       ; layout::REU_DIRTY (asm/border.S refill sets it)
REU_N     = $CADF       ; this piece's length, low byte (0 = 256)
.section .uptext.reu_transfer,"ax",@progbits
.globl reu_transfer
.type reu_transfer,@function
reu_transfer:
    sta 7f + 1          ; the command
    lda #1
    sta REU_DIRTY       ; the first piece loads everything
next:
    lda DJ + 6
    beq 1f
    lda #0              ; 256 bytes
    beq 2f
1:  lda DJ + 5
    bne 2f
    rts                 ; nothing left
2:  sta REU_N
    lda DJ + 3          ; a bank boundary inside this piece? (REU address $x7FFxx, xx > 0)
    cmp #$ff
    bne 3f
    lda DJ + 4
    and #7
    cmp #7
    bne 3f
    lda DJ + 2
    beq 3f
    eor #$ff
    clc
    adc #1              ; 256 - xx: the bytes to the boundary
    ldx REU_N
    beq 4f              ; 256: cut
    cmp REU_N
    bcs 3f              ; it fits
4:  sta REU_N
3:  bit P_GATE
    bmi 3b
    php
    sei
    lda REU_DIRTY
    beq 5f
    ldx #4
6:  lda DJ,x            ; C64 and REU addresses
    sta $df02,x
    dex
    bpl 6b
    lda #0
    sta REU_DIRTY
5:  ldx #0
    lda REU_N
    bne 8f
    inx                 ; 256: high byte 1
8:  sta $df07
    stx $df08
7:  lda #0              ; the command (patched)
    sta $df01
    plp
    clc                 ; the addresses on by n (X = its high byte)
    lda DJ
    adc REU_N
    sta DJ
    txa
    adc DJ + 1
    sta DJ + 1
    clc
    lda DJ + 2
    adc REU_N
    sta DJ + 2
    txa
    adc DJ + 3
    sta DJ + 3
    lda DJ + 4
    adc #0
    sta DJ + 4
    sec                 ; the length down by n
    lda DJ + 5
    sbc REU_N
    sta DJ + 5
    stx REU_N
    lda DJ + 6
    sbc REU_N
    sta DJ + 6
    lda DJ + 2          ; landed on a bank boundary: load everything next time
    ora DJ + 3
    bne 9f
    lda DJ + 4
    and #7
    bne 9f
    inc REU_DIRTY
9:  jmp next
.size reu_transfer, .-reu_transfer
