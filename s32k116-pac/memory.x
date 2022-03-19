MEMORY
{
/* K = KiBi = 1024 Bytes */

/*
128K of Flash is split between:
    Interrupt Vectors   0x0000_0000 -> 0x0000_0400 = 1K
    Config              0x0000_0400 -> 0x0000_0410 = 16B
    Text                0x0000_0410 -> 0x0001_FFFF = <127K
*/

FLASH: ORIGIN = 0x00000000, LENGTH = 128K

/*
17K of Ram is split between:
    SRAM_L 0x1FFF_FC00 -> 0x1FFF_FFFF = 1K
    SRAM_U 0x2000_0000 -> 0x2000_37FF = 14K
    FLEX   0x1400_0000 -> 0x1400_07FF = 2K

    Per RM 31.3.1 for S32K11x SRAM_L is intended for the CoreSight Micro Trace Buffer (MTB, see RM 9.1), but can be used as system ram w/o ECC (non-critical use only)
    Per RM 31.3.2 Burst access cannot cross the SRAM_L/SRAM_U boundary so memory needs to be treated as two separate ranges. FLEX *can* be used as system ram, but incurs additional latencies.
*/

RAM: ORIGIN = 0x20000000, LENGTH = 14K

}

/* Start .text section after config */
_stext = 0x00000410
