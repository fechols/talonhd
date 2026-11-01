/* TALON HD C64 layout (from gclass a5775bf). Program, data and BSS stay in
 * $0801-$4BFF. VIC banks 1 and 2 hold the two frame buffers (screens $4C00 /
 * $8C00, bitmaps $6000 / $A000) and the HUD sprite blocks ($5000 / $8000);
 * $C000-$CBFF holds the renderer's DMA buffers and the HUD table, $E000 the
 * column table, $E500-$FEFF / $9000-$9FFF / $0200-$07FF relocated code (copied at boot), and the
 * soft stack grows down from $D000.
 * $01=$35 (RAM everywhere except I/O); no ROM routines are used. */
__basic_zp_start = 0x0002;
__basic_zp_end = 0x0090;

MEMORY {
    /* $0801-$4BFF: bank 1's $4000-$4BFF is free since the HUD blocks live at $5000. */
    ram (rw) : ORIGIN = 0x0801, LENGTH = 0x43FF
    /* More code where the VIC never looks, copied there at boot (talonhd_boot), each region
     * page-aligned and whole pages long:
     * - the RAM under the KERNAL, after the column table ($E000-$E40F), below the vectors;
     * - $9000-$9FFF, which VIC bank 2 sees as character ROM but the CPU sees as RAM;
     * - $0200-$07FF (no KERNAL, no text screen). */
    hiram (rwx) : ORIGIN = 0xE500, LENGTH = 0x1A00
    midram (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    lowram (rwx) : ORIGIN = 0x0200, LENGTH = 0x0600
    /* Their load images, back to back right after the program in the PRG, inside frame buffer 0's
     * bank ($4C00-$7BFF), which is free until the first frame. */
    hiload (r) : ORIGIN = 0x4C00, LENGTH = 0x3000
}

/* Sections are matched by name before commodore.ld's catch-all .text: the attributed .hitext
 * functions (HUD, boss), then whole sim modules by their mangled-name prefix. */
SECTIONS {
    .hitext : {
        *(.hitext .hitext.*)
        *(.text._ZN7talonhd3sim6actors*)
    } >hiram AT>hiload
    .midtext : {
        *(.text._ZN7talonhd3sim7collide*)
        *(.text._ZN7talonhd3sim6common*)
        *(.text._ZN7talonhd3sim5world*)
    } >midram AT>hiload
    .lowtext : {
        *(.text._ZN7talonhd3sim5gauge*)
        *(.text._ZN7talonhd6render*)
    } >lowram AT>hiload
}
__hitext_start = ADDR(.hitext);
__hitext_load_start = LOADADDR(.hitext);
__hitext_pages = (SIZEOF(.hitext) + 255) / 256;
__midtext_start = ADDR(.midtext);
__midtext_load_start = LOADADDR(.midtext);
__midtext_pages = (SIZEOF(.midtext) + 255) / 256;
__lowtext_start = ADDR(.lowtext);
__lowtext_load_start = LOADADDR(.lowtext);
__lowtext_pages = (SIZEOF(.lowtext) + 255) / 256;

/* The SDK's charset switch (.init.250) calls KERNAL CHROUT, which is
 * unmapped by the time it runs (and $FFD2 now holds relocated code). */
SECTIONS { /DISCARD/ : { *(.init.250) } }

INCLUDE commodore.ld

__stack = 0xD000;

OUTPUT_FORMAT {
    SHORT(ORIGIN(ram))
    FULL(ram)
    TRIM(hiload)
}
