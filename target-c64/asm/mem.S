; memcpy for the whole program: a page loop, then the tail (forward; Rust's memcpy never overlaps).
; Replaces compiler_builtins' generic one (weak, 536 bytes plus a wrapper) to free resident RAM.
; llvm-mos C calling convention: dest in __rc2/__rc3 (returned unchanged), src in __rc4/__rc5,
; n in A (low) and X (high). Clobbers __rc4-__rc8 and A/X/Y (caller-saved).
.section .text.memcpy,"ax",@progbits
.globl memcpy
.type memcpy,@function
memcpy:
    sta mos8(__rc8)
    lda mos8(__rc2)
    sta mos8(__rc6)
    lda mos8(__rc3)
    sta mos8(__rc7)
    ldy #0
    cpx #0
    beq 2f
1:
    lda (__rc4),y
    sta (__rc6),y
    iny
    bne 1b
    inc mos8(__rc5)
    inc mos8(__rc7)
    dex
    bne 1b
2:
    ldx mos8(__rc8)
    beq 4f
3:
    lda (__rc4),y
    sta (__rc6),y
    iny
    dex
    bne 3b
4:
    rts
.size memcpy, .-memcpy

; __memset (what memset and Rust's write_bytes call) for the whole program: a page loop, then the tail;
; ~9 cycles a byte against libc's ~25 (the frame's clears: sprite lists, HUD blocks, the pilot's tables).
; llvm-mos convention as libc's: value in A, dest in __rc2/__rc3, n low in X, high in __rc4. Clobbers
; __rc2-__rc4, X, Y.
.section .text.__memset,"ax",@progbits
.globl __memset
.type __memset,@function
__memset:
    ldy mos8(__rc4)
    beq 2f
    ldy #0
1:
    sta (__rc2),y
    iny
    bne 1b
    inc mos8(__rc3)
    dec mos8(__rc4)
    bne 1b
2:
    cpx #0
    beq 4f
3:
    sta (__rc2),y
    iny
    dex
    bne 3b
4:
    rts
.size __memset, .-__memset
