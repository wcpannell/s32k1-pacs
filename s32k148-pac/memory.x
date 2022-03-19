MEMORY
{
/* K = KiBi = 1024 Bytes */

/*
2M of Flash is split between:
    Interrupt Vectors   0x0000_0000 -> 0x0000_0400 = 1K
    Config              0x0000_0400 -> 0x0000_0410 = 16B
    Text                0x0000_0410 -> 0x0017_FFFF = <2047K
*/

FLASH: ORIGIN = 0x00000000, LENGTH = 2048K

/*
128K of Ram is split between:
    SRAM_L 0x1FFE_0000 -> 0x1FFF_FFFF = 128K
    SRAM_U 0x2000_0000 -> 0x2001_EFFF = 124K
    FLEX   0x1400_0000 -> 0x1400_0FFF = 4K

    Per RM 31.3.1 For S32K14x, misaligned access across the SRAM_L/SRAM_U boundary is not supported
    Per RM 31.3.2 Burst access cannot cross the SRAM_L/SRAM_U boundary so memory needs to be treated as two separate ranges. FLEX *can* be used as system ram, but incurs additional latencies.
    Per RM 33.4.1 The processor code bus has direct (front-door) access to SRAM_L whereas the processor system bus has direct (front-door) access to SRAM_U, vice-versa bus accesses go through the Code Cache Memory Controller (CCM) and, on-miss, access the targeted memory via the crossbar switch using Master0/1 (respectively) ports. In other words, running code from SRAM_U or system memory from SRAM_L incurs latency penalty (see Fig. RM 31-1. SRAM accessability).
*/

RAM: ORIGIN = 0x20000000, LENGTH = 124K

}

/* Start .text section after config */
_stext = 0x00000410
