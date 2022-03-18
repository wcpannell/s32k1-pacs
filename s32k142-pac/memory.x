MEMORY
{
/* K = KiBi = 1024 Bytes */

/*
256K of Flash is split between:
    Interrupt Vectors   0x0000_0000 -> 0x0000_0400 = 1K
    Config              0x0000_0400 -> 0x0000_0410 = 16B
    Text                0x0000_0410 -> 0x0003_FFFF = <255K
*/
FLASH: ORIGIN = 0x00000000, LENGTH = 256K

/*
32K of Ram is split between:
    SRAM_L 0x1FFF_C000 -> 0x1FFF_FFFF = 16K
    SRAM_U 0x2000_0000 -> 0x2000_2FFF = 12K
    FLEX   0x1400_0000 -> 0x1400_0FFF = 4K

    There is (might be?) a weird access restriction on the number of bytes readable across the L/U boundary. Just so I don't have to think about it for now, let's use only SRAM_L. Fix later after confirming
*/

RAM: ORIGIN = 0x1FFFC000, LENGTH = 16K

}

/* Start .text section after config */
_stext = 0x00000410
