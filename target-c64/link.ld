/* TALON HD C64 layout (from gclass a5775bf). Program, data and BSS stay below $4000: VIC banks 1
 * and 2 ($4000-$BFFF) hold the two frame buffers, $C000-$CBFF holds the
 * renderer's DMA buffers, and the soft stack grows down from $D000.
 * $01=$35 (RAM everywhere except I/O); no ROM routines are used. */
__basic_zp_start = 0x0002;
__basic_zp_end = 0x0090;

MEMORY {
    ram (rw) : ORIGIN = 0x0801, LENGTH = 0x37FF
}

/* The SDK's charset switch (.init.250) calls KERNAL CHROUT, which is
 * unmapped by the time it runs. */
SECTIONS { /DISCARD/ : { *(.init.250) } }

INCLUDE commodore.ld

__stack = 0xD000;

OUTPUT_FORMAT {
    SHORT(ORIGIN(ram))
    TRIM(ram)
}
