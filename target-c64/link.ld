/* TALON HD C64 layout (from gclass a5775bf). Program, data and BSS stay in
 * $0801-$4BFF. VIC banks 1 and 2 hold the two frame buffers (screens $4C00 /
 * $8C00, bitmaps $6000 / $A000) and the HUD sprite blocks ($5000 / $8000);
 * $C000-$CBFF holds the renderer's DMA buffers and the HUD table, $E000 the
 * column table, $E410-$FEFF / $0200-$07FF / $5C00-$5FFF / $D000-$DFFF relocated code, $9000-$9FFF the paging window (copied at boot), and the
 * soft stack grows down from $D000.
 * $01=$35 (RAM everywhere except I/O); no ROM routines are used. */
__basic_zp_start = 0x0002;
__basic_zp_end = 0x0090;

MEMORY {
    /* $0801-$4BFF: bank 1's $4000-$4BFF is free since the HUD blocks live at $5000. */
    ram (rw) : ORIGIN = 0x0801, LENGTH = 0x43FF
    /* More code where the VIC never looks, copied there at boot (talonhd_boot), each region
     * page-aligned and whole pages long:
     * - the RAM under the KERNAL, after the column table ($E000-$E40F), below the vectors (the last
     *   page copy may run up to $FF0F, still clear of them);
     * - $0200-$07FF (no KERNAL, no text screen). */
    hiram (rwx) : ORIGIN = 0xE410, LENGTH = 0x1AF0
    lowram (rwx) : ORIGIN = 0x0200, LENGTH = 0x0600
    /* - $5C00-$5FFF, bank 1 after the HUD blocks;
     * - the RAM under the I/O chips: only for code that never touches I/O and runs with $01 = $34
     *   (the game step; the HUD IRQ banks I/O in for itself). */
    upram (rwx) : ORIGIN = 0x5C00, LENGTH = 0x0400
    ioram (rwx) : ORIGIN = 0xD000, LENGTH = 0x1000
    /* llvm-mos's static stack (`.noinit`: the fixed frames of non-recursive functions), moved out of
     * main RAM: after the HUD table ($CB00-$CB50), below the soft stack, which grows down from $D000 and
     * measured 325 bytes deep (tools/emu64.py reports its high-water mark). Never loaded: the PRG's load
     * images end far below. */
    frames (rw) : ORIGIN = 0xCB60, LENGTH = 0x02D0
    /* Their load images, back to back right after the program in the PRG, from $4C00 up to at most $CFFF
     * (frame-buffer, HUD and window space, free until the first frame), followed by the BOOT phase's image, which main stashes
     * in the REU before anything overwrites them. The copies run in this order, and upram's destination
     * only overlaps .hitext's image, which is copied first. */
    hiload (r) : ORIGIN = 0x4C00, LENGTH = 0x8400
    /* REU code paging (src/paging.rs): every phase is linked at the window, $9000-$9FFF (VIC bank 2 sees
     * character ROM there, the CPU RAM), and paged in from the REU when it is needed. */
    ph_boss (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_attract (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_midboss (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_boot (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_ground (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_audio (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_tune (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_ace (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_motor (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_stage (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_s2 (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_s2gnd (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_s2air (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    ph_astro (rwx) : ORIGIN = 0x9000, LENGTH = 0x1000
    /* Load addresses of every phase but BOOT: outside the PRG (OUTPUT_FORMAT writes ram and hiload
     * only). scripts/build-target.sh extracts them to dist/phases/<name>.bin; bake puts them in the
     * level REU image, and boot copies them to the phase bank (main.rs install_phases). */
    phimg (r) : ORIGIN = 0x10000, LENGTH = 0x10000
}

/* Sections are matched by name before commodore.ld's catch-all .text, first match wins: the attributed
 * .hitext functions (HUD, boss flight), whole sim modules by their mangled-name prefix, the boss's
 * render-time helpers (resident), then the paged phases. */
SECTIONS {
    .hitext : {
        *(.hitext .hitext.*)
        *(.text._ZN7talonhd3sim6actors*)
        *(.text._ZN7talonhd3sim6player*4fire*)
        *(.text._ZN7talonhd4game9hud_state* .text._ZN7talonhd4game4view* .text._ZN7talonhd4game5shake* .text._ZN7talonhd4game7palette*)
        *(.text._ZN7talonhd4boss4Boss7sprites* .text._ZN7talonhd4boss4Boss7palette* .text._ZN7talonhd4boss9placement*)
    } >hiram AT>hiload
    .phase.boot : {
        *(.text._ZN11talonhd_c6414install_phases*)
        *(.text._ZN11talonhd_c6410boot_image*)
        *(.text._ZN7talonhd4game4Game5reset* .text._ZN7talonhd4game4Game8reset_at*)
        *(.text._ZN7talonhd6render8hud_init* .text._ZN7talonhd6render10read_image* .text._ZN7talonhd6render12verify_image* .text._ZN7talonhd6render16invalidate_image*)
    } >ph_boot AT>hiload
    .lowtext : {
        *(.text._ZN7talonhd6render*)
        *(.text._ZN11talonhd_c6410read_input*)
        *(.text._ZN7talonhd7midboss7Midboss7sprites* .text._ZN7talonhd7midboss7Midboss6active*)
        *(.text._ZN7talonhd7midboss4roll* .text._ZN7talonhd7midboss4deck* .text._ZN7talonhd7midboss10turret_yaw*)
    } >lowram AT>hiload
    /* The sprite list (main.rs SPRITES): RAM after .lowtext, never loaded or cleared. */
    .lowbss (NOLOAD) : {
        *(.lowbss .lowbss.*)
    } >lowram
    .uptext : {
        *(.uptext .uptext.*)
        *(.text._ZN7talonhd3sim6common*)
        *(.text._ZN7talonhd3sim5world*)
        *(.text._ZN7talonhd2hw*)
    } >upram AT>hiload
    .noinit (NOLOAD) : {
        *(.noinit .noinit.* NULL INIT ZPSAVE)
    } >frames
    .iotext : {
        *(.iotext .iotext.*)
        *(.text._ZN7talonhd3sim7collide*)
        *(.text._ZN7talonhd3sim5gauge*)
    } >ioram AT>hiload
    .phase.boss : {
        *(.text._ZN7talonhd4boss*)
        *(.rodata._ZN7talonhd4boss*)
    } >ph_boss AT>phimg
    .phase.attract : {
        *(.text._ZN7talonhd3sim6replay*)
        *(.text._ZN7talonhd3ace3see* .text._ZN7talonhd3ace6gather* .text._ZN7talonhd3ace3add* .text._ZN7talonhd3ace6choose* .text._ZN7talonhd3ace4rate*)
        *(.rodata._ZN7talonhd3ace6choose*)
    } >ph_attract AT>phimg
    .phase.motor : {
        *(.text._ZN7talonhd3ace5motor* .text._ZN7talonhd3ace6assess* .text._ZN7talonhd3ace5units*)
        *(.rodata._ZN7talonhd3ace2MX* .rodata._ZN7talonhd3ace2MY*)
    } >ph_motor AT>phimg
    .phase.ace : {
        *(.text._ZN7talonhd3ace*)
        *(.rodata._ZN7talonhd3ace*)
        /* Feature `bench` (calls nothing): BOOT has no room for it since the boot code moved there. */
        *(.text.bench .rodata.bench_table)
    } >ph_ace AT>phimg
    .phase.midboss : {
        *(.text._ZN7talonhd7midboss*)
        *(.rodata._ZN7talonhd7midboss*)
    } >ph_midboss AT>phimg
    .phase.tune : {
        *(.text._ZN7talonhd5audio4tune*)
        *(.rodata._ZN7talonhd5audio4tune*)
        *(.text._ZN11talonhd_c648boot_irq* .text._ZN11talonhd_c6410load_level* .text._ZN7talonhd3uci* .rodata._ZN7talonhd3uci* .rodata.uci)
    } >ph_tune AT>phimg
    .phase.audio : {
        *(.phase.audio.kernel)
        *(.text._ZN7talonhd5audio*)
        *(.rodata._ZN7talonhd5audio*)
    } >ph_audio AT>phimg
    .phase.ground : {
        *(.text._ZN7talonhd6ground*)
        *(.rodata._ZN7talonhd6ground*)
    } >ph_ground AT>phimg
    .phase.stage : {
        *(.text._ZN7talonhd5clear*)
        *(.rodata._ZN7talonhd5clear*)
    } >ph_stage AT>phimg
    .phase.s2 : {
        *(.text._ZN7talonhd6stage2*)
        *(.rodata._ZN7talonhd6stage2*)
        *(.text._ZN7talonhd9campanile*)
        *(.rodata._ZN7talonhd9campanile*)
    } >ph_s2 AT>phimg
    .phase.s2gnd : {
        *(.text._ZN7talonhd4gnd2*)
        *(.rodata._ZN7talonhd4gnd2*)
    } >ph_s2gnd AT>phimg
    .phase.s2air : {
        *(.text._ZN7talonhd4air2*)
        *(.rodata._ZN7talonhd4air2*)
    } >ph_s2air AT>phimg
    .phase.astro : {
        *(.text._ZN7talonhd9astrolabe*)
        *(.rodata._ZN7talonhd9astrolabe*)
    } >ph_astro AT>phimg
}
__phase_boot_load_start = LOADADDR(.phase.boot);
__phase_boot_size = SIZEOF(.phase.boot);
__hitext_start = ADDR(.hitext);
__hitext_load_start = LOADADDR(.hitext);
__hitext_pages = (SIZEOF(.hitext) + 255) / 256;
__lowtext_start = ADDR(.lowtext);
__lowtext_load_start = LOADADDR(.lowtext);
__lowtext_pages = (SIZEOF(.lowtext) + 255) / 256;
__uptext_start = ADDR(.uptext);
__uptext_load_start = LOADADDR(.uptext);
__uptext_pages = (SIZEOF(.uptext) + 255) / 256;
__iotext_start = ADDR(.iotext);
__iotext_load_start = LOADADDR(.iotext);
__iotext_pages = (SIZEOF(.iotext) + 255) / 256;
/* Boot copies .uptext ($5C00-$5FFF) before main stashes the BOOT phase from its load image: that image
 * must lie above $6000 or the copy overwrites it (a build with little .iotext would move it down). */
ASSERT(__phase_boot_load_start >= 0x6000, "BOOT phase load image under the .uptext copy: move it up")

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
