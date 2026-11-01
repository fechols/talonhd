/* TALON HD C64 layout (from gclass a5775bf). Program, data and BSS stay in
 * $0801-$4BFF. VIC banks 1 and 2 hold the two frame buffers (screens $4C00 /
 * $8C00, bitmaps $6000 / $A000) and the HUD sprite blocks ($5000 / $9000);
 * $C000-$CBFF holds the renderer's DMA buffers and the HUD table, $E000 the
 * column table, and the soft stack grows down from $D000.
 * $01=$35 (RAM everywhere except I/O); no ROM routines are used. */
__basic_zp_start = 0x0002;
__basic_zp_end = 0x0090;

MEMORY {
    /* $0801-$4BFF: bank 1's $4000-$4BFF is free since the HUD blocks live at $5000. */
    ram (rw) : ORIGIN = 0x0801, LENGTH = 0x43FF
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
