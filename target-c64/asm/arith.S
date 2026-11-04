; Integer libcalls in place of libcrt's (967 bytes of main RAM; these are 248): the u8 and u16 divides
; and remainders and the u8/u16 multiplies. target-c64/build.rs links with -Wl,--wrap=__X for each, so
; every call LLVM-MOS lowers to __X (those generated during LTO too) reaches __wrap___X here. libcrt keeps all its
; divides in one archive member, so defining __X itself would be a duplicate symbol.
;
; LLVM-MOS C ABI: 8-bit arguments in A then X; 16-bit in A/X then __rc2/__rc3; results the same way.
; A, X, Y and __rc2-__rc19 are caller-saved; each routine writes only the registers listed for it.
; Rust never divides by zero (it checks first) and never calls a signed divide with MIN / -1 (the
; same check). tools/test_arith.py runs each one from the linked program against Python.

; ---------------------------------------------------------------------------------------------------
; u8: one 8-step restoring divide for __udivqi3, __umodqi3 and __udivmodqi4 (libcrt's: 93 + 64 + 125
; bytes). The remainder stays in A with its 9th bit in carry: R < d <= 255 before each step, so 2R + 1
; < 2d and one subtraction always brings it back under d.

; __umodqi3(n: u8, d: u8) -> u8, and the core: A = n % d, __rc4 = n / d. Writes __rc4 (n, then the
; quotient), __rc5 (d), X.
.section .text.__wrap___umodqi3,"ax",@progbits  ; the whole u8 family
.globl __wrap___umodqi3
.type __wrap___umodqi3,@function
__wrap___umodqi3:
    sta mos8(__rc4)
    stx mos8(__rc5)
    lda #0
    ldx #8
1:  asl mos8(__rc4)
    rol a
    bcs 2f
    cmp mos8(__rc5)
    bcc 3f
2:  sbc mos8(__rc5)
    inc mos8(__rc4)
3:  dex
    bne 1b
    rts
.size __wrap___umodqi3, .-__wrap___umodqi3

; __udivqi3(n: u8, d: u8) -> u8. Writes __rc4, __rc5, X.
.globl __wrap___udivqi3
.type __wrap___udivqi3,@function
__wrap___udivqi3:
    jsr __wrap___umodqi3
    lda mos8(__rc4)
    rts
.size __wrap___udivqi3, .-__wrap___udivqi3

; __udivmodqi4(n: u8, d: u8, rem: *mut u8) -> u8: the quotient, the remainder stored through the
; pointer in __rc2/__rc3 (libcrt's ABI). Writes __rc4, __rc5, X, Y.
.globl __wrap___udivmodqi4
.type __wrap___udivmodqi4,@function
__wrap___udivmodqi4:
    jsr __wrap___umodqi3
    ldy #0
    sta (__rc2),y
    lda mos8(__rc4)
    rts
.size __wrap___udivmodqi4, .-__wrap___udivmodqi4

; ---------------------------------------------------------------------------------------------------
; u16: one 16-step restoring divide for __udivhi3, __umodhi3, __divhi3 and __modhi3 (libcrt's: 196 +
; 174 + 125 + 111 bytes). The signed ones divide the magnitudes and fix the signs: the quotient
; truncates toward zero, the remainder takes the dividend's sign.

; __divhi3(a: i16, b: i16) -> i16. Writes __rc2-__rc9, X (and two return addresses on the stack).
.section .text.__wrap___divhi3,"ax",@progbits  ; the whole u16 family: branches stay inside one section
.globl __wrap___divhi3
.type __wrap___divhi3,@function
__wrap___divhi3:
    jsr sdivmod16
    lda mos8(__rc9)
    ldx #mos8(__rc4)
    ; Return the u16 at zero page X, negated when A's bit 7 is set.
sfinish:
    asl a
    bcc 1f
    jsr neg16
1:  lda 0,x
    pha
    lda 1,x
    tax
    pla
    rts
.size __wrap___divhi3, .-__wrap___divhi3

; __modhi3(a: i16, b: i16) -> i16. Writes __rc2-__rc9, X.
.globl __wrap___modhi3
.type __wrap___modhi3,@function
__wrap___modhi3:
    jsr sdivmod16
    lda mos8(__rc8)
    ldx #mos8(__rc6)
    bne sfinish             ; always (X != 0)
.size __wrap___modhi3, .-__wrap___modhi3

; |a| / |b| into the core: __rc8 bit 7 = a < 0 (the remainder's sign), __rc9 bit 7 = the quotient's
; sign, then udivmod16 on the magnitudes (|-32768| = 32768 as a u16).
.type sdivmod16,@function
sdivmod16:
    sta mos8(__rc4)
    stx mos8(__rc5)
    stx mos8(__rc8)
    txa
    eor mos8(__rc3)
    sta mos8(__rc9)
    ldx #mos8(__rc4)
    bit mos8(__rc8)
    bpl 1f
    jsr neg16
1:  ldx #mos8(__rc2)
    bit mos8(__rc3)
    bpl udivmod16
    jsr neg16
    ; falls through
; The core: __rc4/__rc5 = n / d, __rc6/__rc7 = n % d for n in __rc4/__rc5, d in __rc2/__rc3. The
; remainder keeps its 17th bit in carry (d may exceed 32767). When d < 256 and n's high byte is below it
; (a quotient under 256: most of the game's), the first 8 steps would only shift that byte into the
; remainder, so it starts there and 8 steps remain. Writes __rc4-__rc7, A, X.
udivmod16:
    lda #0
    sta mos8(__rc6)
    sta mos8(__rc7)
    ldx #16
    lda mos8(__rc3)
    bne 1f
    lda mos8(__rc5)
    cmp mos8(__rc2)
    bcs 1f
    sta mos8(__rc6)
    lda mos8(__rc4)
    sta mos8(__rc5)
    lda #0
    sta mos8(__rc4)
    ldx #8
1:  asl mos8(__rc4)
    rol mos8(__rc5)
    rol mos8(__rc6)
    rol mos8(__rc7)
    lda mos8(__rc6)
    bcs 2f                  ; R >= 65536 > d: subtract (carry set)
    cmp mos8(__rc2)
    lda mos8(__rc7)
    sbc mos8(__rc3)
    bcc 3f                  ; R < d
    lda mos8(__rc6)
2:  sbc mos8(__rc2)
    sta mos8(__rc6)
    lda mos8(__rc7)
    sbc mos8(__rc3)
    sta mos8(__rc7)
    inc mos8(__rc4)
3:  dex
    bne 1b
    rts
.size sdivmod16, .-sdivmod16

; Negate the u16 at zero page X, X + 1. Writes A.
.type neg16,@function
neg16:
    sec
    lda #0
    sbc 0,x
    sta 0,x
    lda #0
    sbc 1,x
    sta 1,x
    rts
.size neg16, .-neg16

; __umodhi3(n: u16, d: u16) -> u16. Writes __rc4-__rc7, X.
.globl __wrap___umodhi3
.type __wrap___umodhi3,@function
__wrap___umodhi3:
    jsr __wrap___udivhi3
    lda mos8(__rc6)
    ldx mos8(__rc7)
    rts
.size __wrap___umodhi3, .-__wrap___umodhi3

; __udivhi3(n: u16, d: u16) -> u16. Writes __rc4-__rc7, X.
.globl __wrap___udivhi3
.type __wrap___udivhi3,@function
__wrap___udivhi3:
    sta mos8(__rc4)
    stx mos8(__rc5)
    jsr udivmod16
    lda mos8(__rc4)
    ldx mos8(__rc5)
    rts
.size __wrap___udivhi3, .-__wrap___udivhi3

; ---------------------------------------------------------------------------------------------------
; Multiplies: shift and add, the low bits (the same signed or unsigned), until b runs out.

; __mulqi3(a: u8, b: u8) -> u8 (libcrt's: 27 bytes): the u16 multiply with both high bytes 0 (its
; low byte is in A). Writes __rc2-__rc6, X, Y.
.section .text.__wrap___mulhi3,"ax",@progbits
.globl __wrap___mulqi3
.type __wrap___mulqi3,@function
__wrap___mulqi3:
    stx mos8(__rc2)
    ldx #0
    stx mos8(__rc3)
    ; falls through, A = a, X = 0
.size __wrap___mulqi3, .-__wrap___mulqi3

; __mulhi3(a: u16, b: u16) -> u16 (libcrt's: 52 bytes). The product's low byte stays in A. Writes
; __rc2/__rc3 (b, shifted), __rc4/__rc5 (a, shifted), __rc6 (the product's high byte), X, Y.
.globl __wrap___mulhi3
.type __wrap___mulhi3,@function
__wrap___mulhi3:
    sta mos8(__rc4)
    stx mos8(__rc5)
    lda #0
    sta mos8(__rc6)
1:  lsr mos8(__rc3)
    ror mos8(__rc2)
    bcc 2f
    clc
    adc mos8(__rc4)
    tay
    lda mos8(__rc6)
    adc mos8(__rc5)
    sta mos8(__rc6)
    tya
2:  asl mos8(__rc4)
    rol mos8(__rc5)
    ldx mos8(__rc2)
    bne 1b
    ldx mos8(__rc3)
    bne 1b
    ldx mos8(__rc6)
    rts
.size __wrap___mulhi3, .-__wrap___mulhi3
