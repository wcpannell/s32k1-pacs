#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0__embedded_ram0hl: [u8; 0x04],
    _reserved_1__embedded_ram1hl: [u8; 0x04],
    _reserved_2__embedded_ram2hl: [u8; 0x04],
    _reserved_3__embedded_ram3hl: [u8; 0x04],
    _reserved_4__embedded_ram4hl: [u8; 0x04],
    _reserved_5__embedded_ram5hl: [u8; 0x04],
    _reserved_6__embedded_ram6hl: [u8; 0x04],
    _reserved_7__embedded_ram7hl: [u8; 0x04],
    _reserved_8__embedded_ram8hl: [u8; 0x04],
    _reserved_9__embedded_ram9hl: [u8; 0x04],
    _reserved_10__embedded_ram10hl: [u8; 0x04],
    _reserved_11__embedded_ram11hl: [u8; 0x04],
    _reserved_12__embedded_ram12hl: [u8; 0x04],
    _reserved_13__embedded_ram13hl: [u8; 0x04],
    _reserved_14__embedded_ram14hl: [u8; 0x04],
    _reserved_15__embedded_ram15hl: [u8; 0x04],
    _reserved_16__embedded_ram16hl: [u8; 0x04],
    _reserved_17__embedded_ram17hl: [u8; 0x04],
    _reserved_18__embedded_ram18hl: [u8; 0x04],
    _reserved_19__embedded_ram19hl: [u8; 0x04],
    _reserved_20__embedded_ram20hl: [u8; 0x04],
    _reserved_21__embedded_ram21hl: [u8; 0x04],
    _reserved_22__embedded_ram22hl: [u8; 0x04],
    _reserved_23__embedded_ram23hl: [u8; 0x04],
    _reserved_24__embedded_ram24hl: [u8; 0x04],
    _reserved_25__embedded_ram25hl: [u8; 0x04],
    _reserved_26__embedded_ram26hl: [u8; 0x04],
    _reserved_27__embedded_ram27hl: [u8; 0x04],
    _reserved_28__embedded_ram28hl: [u8; 0x04],
    _reserved_29__embedded_ram29hl: [u8; 0x04],
    _reserved_30__embedded_ram30hl: [u8; 0x04],
    _reserved_31__embedded_ram31hl: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - CSE PRAM0LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram0ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram0ll::CSE_PRAM__EMBEDDEDRAM0LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<cse_pram__embedded_ram0ll::CSE_PRAM__EMBEDDEDRAM0LL_SPEC>)
        }
    }
    #[doc = "0x00 - CSE PRAM 0 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram0(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram0::CSE_PRAM__EMBEDDEDRAM0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<cse_pram__embedded_ram0::CSE_PRAM__EMBEDDEDRAM0_SPEC>)
        }
    }
    #[doc = "0x01 - CSE PRAM0LU register."]
    #[inline(always)]
    pub fn _embedded_ram0lu(&self) -> &crate::Reg<_embedded_ram0lu::_EMBEDDEDRAM0LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1usize)
                as *const crate::Reg<_embedded_ram0lu::_EMBEDDEDRAM0LU_SPEC>)
        }
    }
    #[doc = "0x02 - CSE PRAM0HL register."]
    #[inline(always)]
    pub fn _embedded_ram0hl(&self) -> &crate::Reg<_embedded_ram0hl::_EMBEDDEDRAM0HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<_embedded_ram0hl::_EMBEDDEDRAM0HL_SPEC>)
        }
    }
    #[doc = "0x03 - CSE PRAM0HU register."]
    #[inline(always)]
    pub fn _embedded_ram0hu(&self) -> &crate::Reg<_embedded_ram0hu::_EMBEDDEDRAM0HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3usize)
                as *const crate::Reg<_embedded_ram0hu::_EMBEDDEDRAM0HU_SPEC>)
        }
    }
    #[doc = "0x04 - CSE PRAM1LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram1ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram1ll::CSE_PRAM__EMBEDDEDRAM1LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<cse_pram__embedded_ram1ll::CSE_PRAM__EMBEDDEDRAM1LL_SPEC>)
        }
    }
    #[doc = "0x04 - CSE PRAM 1 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram1(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram1::CSE_PRAM__EMBEDDEDRAM1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<cse_pram__embedded_ram1::CSE_PRAM__EMBEDDEDRAM1_SPEC>)
        }
    }
    #[doc = "0x05 - CSE PRAM1LU register."]
    #[inline(always)]
    pub fn _embedded_ram1lu(&self) -> &crate::Reg<_embedded_ram1lu::_EMBEDDEDRAM1LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5usize)
                as *const crate::Reg<_embedded_ram1lu::_EMBEDDEDRAM1LU_SPEC>)
        }
    }
    #[doc = "0x06 - CSE PRAM1HL register."]
    #[inline(always)]
    pub fn _embedded_ram1hl(&self) -> &crate::Reg<_embedded_ram1hl::_EMBEDDEDRAM1HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(6usize)
                as *const crate::Reg<_embedded_ram1hl::_EMBEDDEDRAM1HL_SPEC>)
        }
    }
    #[doc = "0x07 - CSE PRAM1HU register."]
    #[inline(always)]
    pub fn _embedded_ram1hu(&self) -> &crate::Reg<_embedded_ram1hu::_EMBEDDEDRAM1HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(7usize)
                as *const crate::Reg<_embedded_ram1hu::_EMBEDDEDRAM1HU_SPEC>)
        }
    }
    #[doc = "0x08 - CSE PRAM2LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram2ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram2ll::CSE_PRAM__EMBEDDEDRAM2LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<cse_pram__embedded_ram2ll::CSE_PRAM__EMBEDDEDRAM2LL_SPEC>)
        }
    }
    #[doc = "0x08 - CSE PRAM 2 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram2(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram2::CSE_PRAM__EMBEDDEDRAM2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<cse_pram__embedded_ram2::CSE_PRAM__EMBEDDEDRAM2_SPEC>)
        }
    }
    #[doc = "0x09 - CSE PRAM2LU register."]
    #[inline(always)]
    pub fn _embedded_ram2lu(&self) -> &crate::Reg<_embedded_ram2lu::_EMBEDDEDRAM2LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(9usize)
                as *const crate::Reg<_embedded_ram2lu::_EMBEDDEDRAM2LU_SPEC>)
        }
    }
    #[doc = "0x0a - CSE PRAM2HL register."]
    #[inline(always)]
    pub fn _embedded_ram2hl(&self) -> &crate::Reg<_embedded_ram2hl::_EMBEDDEDRAM2HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(10usize)
                as *const crate::Reg<_embedded_ram2hl::_EMBEDDEDRAM2HL_SPEC>)
        }
    }
    #[doc = "0x0b - CSE PRAM2HU register."]
    #[inline(always)]
    pub fn _embedded_ram2hu(&self) -> &crate::Reg<_embedded_ram2hu::_EMBEDDEDRAM2HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(11usize)
                as *const crate::Reg<_embedded_ram2hu::_EMBEDDEDRAM2HU_SPEC>)
        }
    }
    #[doc = "0x0c - CSE PRAM3LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram3ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram3ll::CSE_PRAM__EMBEDDEDRAM3LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<cse_pram__embedded_ram3ll::CSE_PRAM__EMBEDDEDRAM3LL_SPEC>)
        }
    }
    #[doc = "0x0c - CSE PRAM 3 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram3(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram3::CSE_PRAM__EMBEDDEDRAM3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<cse_pram__embedded_ram3::CSE_PRAM__EMBEDDEDRAM3_SPEC>)
        }
    }
    #[doc = "0x0d - CSE PRAM3LU register."]
    #[inline(always)]
    pub fn _embedded_ram3lu(&self) -> &crate::Reg<_embedded_ram3lu::_EMBEDDEDRAM3LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(13usize)
                as *const crate::Reg<_embedded_ram3lu::_EMBEDDEDRAM3LU_SPEC>)
        }
    }
    #[doc = "0x0e - CSE PRAM3HL register."]
    #[inline(always)]
    pub fn _embedded_ram3hl(&self) -> &crate::Reg<_embedded_ram3hl::_EMBEDDEDRAM3HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(14usize)
                as *const crate::Reg<_embedded_ram3hl::_EMBEDDEDRAM3HL_SPEC>)
        }
    }
    #[doc = "0x0f - CSE PRAM3HU register."]
    #[inline(always)]
    pub fn _embedded_ram3hu(&self) -> &crate::Reg<_embedded_ram3hu::_EMBEDDEDRAM3HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(15usize)
                as *const crate::Reg<_embedded_ram3hu::_EMBEDDEDRAM3HU_SPEC>)
        }
    }
    #[doc = "0x10 - CSE PRAM4LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram4ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram4ll::CSE_PRAM__EMBEDDEDRAM4LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<cse_pram__embedded_ram4ll::CSE_PRAM__EMBEDDEDRAM4LL_SPEC>)
        }
    }
    #[doc = "0x10 - CSE PRAM 4 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram4(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram4::CSE_PRAM__EMBEDDEDRAM4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<cse_pram__embedded_ram4::CSE_PRAM__EMBEDDEDRAM4_SPEC>)
        }
    }
    #[doc = "0x11 - CSE PRAM4LU register."]
    #[inline(always)]
    pub fn _embedded_ram4lu(&self) -> &crate::Reg<_embedded_ram4lu::_EMBEDDEDRAM4LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(17usize)
                as *const crate::Reg<_embedded_ram4lu::_EMBEDDEDRAM4LU_SPEC>)
        }
    }
    #[doc = "0x12 - CSE PRAM4HL register."]
    #[inline(always)]
    pub fn _embedded_ram4hl(&self) -> &crate::Reg<_embedded_ram4hl::_EMBEDDEDRAM4HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(18usize)
                as *const crate::Reg<_embedded_ram4hl::_EMBEDDEDRAM4HL_SPEC>)
        }
    }
    #[doc = "0x13 - CSE PRAM4HU register."]
    #[inline(always)]
    pub fn _embedded_ram4hu(&self) -> &crate::Reg<_embedded_ram4hu::_EMBEDDEDRAM4HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(19usize)
                as *const crate::Reg<_embedded_ram4hu::_EMBEDDEDRAM4HU_SPEC>)
        }
    }
    #[doc = "0x14 - CSE PRAM5LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram5ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram5ll::CSE_PRAM__EMBEDDEDRAM5LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<cse_pram__embedded_ram5ll::CSE_PRAM__EMBEDDEDRAM5LL_SPEC>)
        }
    }
    #[doc = "0x14 - CSE PRAM 5 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram5(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram5::CSE_PRAM__EMBEDDEDRAM5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<cse_pram__embedded_ram5::CSE_PRAM__EMBEDDEDRAM5_SPEC>)
        }
    }
    #[doc = "0x15 - CSE PRAM5LU register."]
    #[inline(always)]
    pub fn _embedded_ram5lu(&self) -> &crate::Reg<_embedded_ram5lu::_EMBEDDEDRAM5LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(21usize)
                as *const crate::Reg<_embedded_ram5lu::_EMBEDDEDRAM5LU_SPEC>)
        }
    }
    #[doc = "0x16 - CSE PRAM5HL register."]
    #[inline(always)]
    pub fn _embedded_ram5hl(&self) -> &crate::Reg<_embedded_ram5hl::_EMBEDDEDRAM5HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(22usize)
                as *const crate::Reg<_embedded_ram5hl::_EMBEDDEDRAM5HL_SPEC>)
        }
    }
    #[doc = "0x17 - CSE PRAM5HU register."]
    #[inline(always)]
    pub fn _embedded_ram5hu(&self) -> &crate::Reg<_embedded_ram5hu::_EMBEDDEDRAM5HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(23usize)
                as *const crate::Reg<_embedded_ram5hu::_EMBEDDEDRAM5HU_SPEC>)
        }
    }
    #[doc = "0x18 - CSE PRAM6LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram6ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram6ll::CSE_PRAM__EMBEDDEDRAM6LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<cse_pram__embedded_ram6ll::CSE_PRAM__EMBEDDEDRAM6LL_SPEC>)
        }
    }
    #[doc = "0x18 - CSE PRAM 6 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram6(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram6::CSE_PRAM__EMBEDDEDRAM6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<cse_pram__embedded_ram6::CSE_PRAM__EMBEDDEDRAM6_SPEC>)
        }
    }
    #[doc = "0x19 - CSE PRAM6LU register."]
    #[inline(always)]
    pub fn _embedded_ram6lu(&self) -> &crate::Reg<_embedded_ram6lu::_EMBEDDEDRAM6LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(25usize)
                as *const crate::Reg<_embedded_ram6lu::_EMBEDDEDRAM6LU_SPEC>)
        }
    }
    #[doc = "0x1a - CSE PRAM6HL register."]
    #[inline(always)]
    pub fn _embedded_ram6hl(&self) -> &crate::Reg<_embedded_ram6hl::_EMBEDDEDRAM6HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(26usize)
                as *const crate::Reg<_embedded_ram6hl::_EMBEDDEDRAM6HL_SPEC>)
        }
    }
    #[doc = "0x1b - CSE PRAM6HU register."]
    #[inline(always)]
    pub fn _embedded_ram6hu(&self) -> &crate::Reg<_embedded_ram6hu::_EMBEDDEDRAM6HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(27usize)
                as *const crate::Reg<_embedded_ram6hu::_EMBEDDEDRAM6HU_SPEC>)
        }
    }
    #[doc = "0x1c - CSE PRAM7LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram7ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram7ll::CSE_PRAM__EMBEDDEDRAM7LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<cse_pram__embedded_ram7ll::CSE_PRAM__EMBEDDEDRAM7LL_SPEC>)
        }
    }
    #[doc = "0x1c - CSE PRAM 7 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram7(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram7::CSE_PRAM__EMBEDDEDRAM7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<cse_pram__embedded_ram7::CSE_PRAM__EMBEDDEDRAM7_SPEC>)
        }
    }
    #[doc = "0x1d - CSE PRAM7LU register."]
    #[inline(always)]
    pub fn _embedded_ram7lu(&self) -> &crate::Reg<_embedded_ram7lu::_EMBEDDEDRAM7LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(29usize)
                as *const crate::Reg<_embedded_ram7lu::_EMBEDDEDRAM7LU_SPEC>)
        }
    }
    #[doc = "0x1e - CSE PRAM7HL register."]
    #[inline(always)]
    pub fn _embedded_ram7hl(&self) -> &crate::Reg<_embedded_ram7hl::_EMBEDDEDRAM7HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(30usize)
                as *const crate::Reg<_embedded_ram7hl::_EMBEDDEDRAM7HL_SPEC>)
        }
    }
    #[doc = "0x1f - CSE PRAM7HU register."]
    #[inline(always)]
    pub fn _embedded_ram7hu(&self) -> &crate::Reg<_embedded_ram7hu::_EMBEDDEDRAM7HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(31usize)
                as *const crate::Reg<_embedded_ram7hu::_EMBEDDEDRAM7HU_SPEC>)
        }
    }
    #[doc = "0x20 - CSE PRAM8LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram8ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram8ll::CSE_PRAM__EMBEDDEDRAM8LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<cse_pram__embedded_ram8ll::CSE_PRAM__EMBEDDEDRAM8LL_SPEC>)
        }
    }
    #[doc = "0x20 - CSE PRAM 8 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram8(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram8::CSE_PRAM__EMBEDDEDRAM8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<cse_pram__embedded_ram8::CSE_PRAM__EMBEDDEDRAM8_SPEC>)
        }
    }
    #[doc = "0x21 - CSE PRAM8LU register."]
    #[inline(always)]
    pub fn _embedded_ram8lu(&self) -> &crate::Reg<_embedded_ram8lu::_EMBEDDEDRAM8LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(33usize)
                as *const crate::Reg<_embedded_ram8lu::_EMBEDDEDRAM8LU_SPEC>)
        }
    }
    #[doc = "0x22 - CSE PRAM8HL register."]
    #[inline(always)]
    pub fn _embedded_ram8hl(&self) -> &crate::Reg<_embedded_ram8hl::_EMBEDDEDRAM8HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(34usize)
                as *const crate::Reg<_embedded_ram8hl::_EMBEDDEDRAM8HL_SPEC>)
        }
    }
    #[doc = "0x23 - CSE PRAM8HU register."]
    #[inline(always)]
    pub fn _embedded_ram8hu(&self) -> &crate::Reg<_embedded_ram8hu::_EMBEDDEDRAM8HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(35usize)
                as *const crate::Reg<_embedded_ram8hu::_EMBEDDEDRAM8HU_SPEC>)
        }
    }
    #[doc = "0x24 - CSE PRAM9LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram9ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram9ll::CSE_PRAM__EMBEDDEDRAM9LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<cse_pram__embedded_ram9ll::CSE_PRAM__EMBEDDEDRAM9LL_SPEC>)
        }
    }
    #[doc = "0x24 - CSE PRAM 9 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram9(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram9::CSE_PRAM__EMBEDDEDRAM9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<cse_pram__embedded_ram9::CSE_PRAM__EMBEDDEDRAM9_SPEC>)
        }
    }
    #[doc = "0x25 - CSE PRAM9LU register."]
    #[inline(always)]
    pub fn _embedded_ram9lu(&self) -> &crate::Reg<_embedded_ram9lu::_EMBEDDEDRAM9LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(37usize)
                as *const crate::Reg<_embedded_ram9lu::_EMBEDDEDRAM9LU_SPEC>)
        }
    }
    #[doc = "0x26 - CSE PRAM9HL register."]
    #[inline(always)]
    pub fn _embedded_ram9hl(&self) -> &crate::Reg<_embedded_ram9hl::_EMBEDDEDRAM9HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(38usize)
                as *const crate::Reg<_embedded_ram9hl::_EMBEDDEDRAM9HL_SPEC>)
        }
    }
    #[doc = "0x27 - CSE PRAM9HU register."]
    #[inline(always)]
    pub fn _embedded_ram9hu(&self) -> &crate::Reg<_embedded_ram9hu::_EMBEDDEDRAM9HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(39usize)
                as *const crate::Reg<_embedded_ram9hu::_EMBEDDEDRAM9HU_SPEC>)
        }
    }
    #[doc = "0x28 - CSE PRAM10LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram10ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram10ll::CSE_PRAM__EMBEDDEDRAM10LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<cse_pram__embedded_ram10ll::CSE_PRAM__EMBEDDEDRAM10LL_SPEC>)
        }
    }
    #[doc = "0x28 - CSE PRAM 10 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram10(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram10::CSE_PRAM__EMBEDDEDRAM10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<cse_pram__embedded_ram10::CSE_PRAM__EMBEDDEDRAM10_SPEC>)
        }
    }
    #[doc = "0x29 - CSE PRAM10LU register."]
    #[inline(always)]
    pub fn _embedded_ram10lu(&self) -> &crate::Reg<_embedded_ram10lu::_EMBEDDEDRAM10LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(41usize)
                as *const crate::Reg<_embedded_ram10lu::_EMBEDDEDRAM10LU_SPEC>)
        }
    }
    #[doc = "0x2a - CSE PRAM10HL register."]
    #[inline(always)]
    pub fn _embedded_ram10hl(&self) -> &crate::Reg<_embedded_ram10hl::_EMBEDDEDRAM10HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(42usize)
                as *const crate::Reg<_embedded_ram10hl::_EMBEDDEDRAM10HL_SPEC>)
        }
    }
    #[doc = "0x2b - CSE PRAM10HU register."]
    #[inline(always)]
    pub fn _embedded_ram10hu(&self) -> &crate::Reg<_embedded_ram10hu::_EMBEDDEDRAM10HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(43usize)
                as *const crate::Reg<_embedded_ram10hu::_EMBEDDEDRAM10HU_SPEC>)
        }
    }
    #[doc = "0x2c - CSE PRAM11LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram11ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram11ll::CSE_PRAM__EMBEDDEDRAM11LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<cse_pram__embedded_ram11ll::CSE_PRAM__EMBEDDEDRAM11LL_SPEC>)
        }
    }
    #[doc = "0x2c - CSE PRAM 11 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram11(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram11::CSE_PRAM__EMBEDDEDRAM11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<cse_pram__embedded_ram11::CSE_PRAM__EMBEDDEDRAM11_SPEC>)
        }
    }
    #[doc = "0x2d - CSE PRAM11LU register."]
    #[inline(always)]
    pub fn _embedded_ram11lu(&self) -> &crate::Reg<_embedded_ram11lu::_EMBEDDEDRAM11LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(45usize)
                as *const crate::Reg<_embedded_ram11lu::_EMBEDDEDRAM11LU_SPEC>)
        }
    }
    #[doc = "0x2e - CSE PRAM11HL register."]
    #[inline(always)]
    pub fn _embedded_ram11hl(&self) -> &crate::Reg<_embedded_ram11hl::_EMBEDDEDRAM11HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(46usize)
                as *const crate::Reg<_embedded_ram11hl::_EMBEDDEDRAM11HL_SPEC>)
        }
    }
    #[doc = "0x2f - CSE PRAM11HU register."]
    #[inline(always)]
    pub fn _embedded_ram11hu(&self) -> &crate::Reg<_embedded_ram11hu::_EMBEDDEDRAM11HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(47usize)
                as *const crate::Reg<_embedded_ram11hu::_EMBEDDEDRAM11HU_SPEC>)
        }
    }
    #[doc = "0x30 - CSE PRAM12LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram12ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram12ll::CSE_PRAM__EMBEDDEDRAM12LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<cse_pram__embedded_ram12ll::CSE_PRAM__EMBEDDEDRAM12LL_SPEC>)
        }
    }
    #[doc = "0x30 - CSE PRAM 12 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram12(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram12::CSE_PRAM__EMBEDDEDRAM12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<cse_pram__embedded_ram12::CSE_PRAM__EMBEDDEDRAM12_SPEC>)
        }
    }
    #[doc = "0x31 - CSE PRAM12LU register."]
    #[inline(always)]
    pub fn _embedded_ram12lu(&self) -> &crate::Reg<_embedded_ram12lu::_EMBEDDEDRAM12LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(49usize)
                as *const crate::Reg<_embedded_ram12lu::_EMBEDDEDRAM12LU_SPEC>)
        }
    }
    #[doc = "0x32 - CSE PRAM12HL register."]
    #[inline(always)]
    pub fn _embedded_ram12hl(&self) -> &crate::Reg<_embedded_ram12hl::_EMBEDDEDRAM12HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(50usize)
                as *const crate::Reg<_embedded_ram12hl::_EMBEDDEDRAM12HL_SPEC>)
        }
    }
    #[doc = "0x33 - CSE PRAM12HU register."]
    #[inline(always)]
    pub fn _embedded_ram12hu(&self) -> &crate::Reg<_embedded_ram12hu::_EMBEDDEDRAM12HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(51usize)
                as *const crate::Reg<_embedded_ram12hu::_EMBEDDEDRAM12HU_SPEC>)
        }
    }
    #[doc = "0x34 - CSE PRAM13LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram13ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram13ll::CSE_PRAM__EMBEDDEDRAM13LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<cse_pram__embedded_ram13ll::CSE_PRAM__EMBEDDEDRAM13LL_SPEC>)
        }
    }
    #[doc = "0x34 - CSE PRAM 13 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram13(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram13::CSE_PRAM__EMBEDDEDRAM13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<cse_pram__embedded_ram13::CSE_PRAM__EMBEDDEDRAM13_SPEC>)
        }
    }
    #[doc = "0x35 - CSE PRAM13LU register."]
    #[inline(always)]
    pub fn _embedded_ram13lu(&self) -> &crate::Reg<_embedded_ram13lu::_EMBEDDEDRAM13LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(53usize)
                as *const crate::Reg<_embedded_ram13lu::_EMBEDDEDRAM13LU_SPEC>)
        }
    }
    #[doc = "0x36 - CSE PRAM13HL register."]
    #[inline(always)]
    pub fn _embedded_ram13hl(&self) -> &crate::Reg<_embedded_ram13hl::_EMBEDDEDRAM13HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(54usize)
                as *const crate::Reg<_embedded_ram13hl::_EMBEDDEDRAM13HL_SPEC>)
        }
    }
    #[doc = "0x37 - CSE PRAM13HU register."]
    #[inline(always)]
    pub fn _embedded_ram13hu(&self) -> &crate::Reg<_embedded_ram13hu::_EMBEDDEDRAM13HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(55usize)
                as *const crate::Reg<_embedded_ram13hu::_EMBEDDEDRAM13HU_SPEC>)
        }
    }
    #[doc = "0x38 - CSE PRAM14LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram14ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram14ll::CSE_PRAM__EMBEDDEDRAM14LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<cse_pram__embedded_ram14ll::CSE_PRAM__EMBEDDEDRAM14LL_SPEC>)
        }
    }
    #[doc = "0x38 - CSE PRAM 14 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram14(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram14::CSE_PRAM__EMBEDDEDRAM14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<cse_pram__embedded_ram14::CSE_PRAM__EMBEDDEDRAM14_SPEC>)
        }
    }
    #[doc = "0x39 - CSE PRAM14LU register."]
    #[inline(always)]
    pub fn _embedded_ram14lu(&self) -> &crate::Reg<_embedded_ram14lu::_EMBEDDEDRAM14LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(57usize)
                as *const crate::Reg<_embedded_ram14lu::_EMBEDDEDRAM14LU_SPEC>)
        }
    }
    #[doc = "0x3a - CSE PRAM14HL register."]
    #[inline(always)]
    pub fn _embedded_ram14hl(&self) -> &crate::Reg<_embedded_ram14hl::_EMBEDDEDRAM14HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(58usize)
                as *const crate::Reg<_embedded_ram14hl::_EMBEDDEDRAM14HL_SPEC>)
        }
    }
    #[doc = "0x3b - CSE PRAM14HU register."]
    #[inline(always)]
    pub fn _embedded_ram14hu(&self) -> &crate::Reg<_embedded_ram14hu::_EMBEDDEDRAM14HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(59usize)
                as *const crate::Reg<_embedded_ram14hu::_EMBEDDEDRAM14HU_SPEC>)
        }
    }
    #[doc = "0x3c - CSE PRAM15LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram15ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram15ll::CSE_PRAM__EMBEDDEDRAM15LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize)
                as *const crate::Reg<cse_pram__embedded_ram15ll::CSE_PRAM__EMBEDDEDRAM15LL_SPEC>)
        }
    }
    #[doc = "0x3c - CSE PRAM 15 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram15(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram15::CSE_PRAM__EMBEDDEDRAM15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(60usize)
                as *const crate::Reg<cse_pram__embedded_ram15::CSE_PRAM__EMBEDDEDRAM15_SPEC>)
        }
    }
    #[doc = "0x3d - CSE PRAM15LU register."]
    #[inline(always)]
    pub fn _embedded_ram15lu(&self) -> &crate::Reg<_embedded_ram15lu::_EMBEDDEDRAM15LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(61usize)
                as *const crate::Reg<_embedded_ram15lu::_EMBEDDEDRAM15LU_SPEC>)
        }
    }
    #[doc = "0x3e - CSE PRAM15HL register."]
    #[inline(always)]
    pub fn _embedded_ram15hl(&self) -> &crate::Reg<_embedded_ram15hl::_EMBEDDEDRAM15HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(62usize)
                as *const crate::Reg<_embedded_ram15hl::_EMBEDDEDRAM15HL_SPEC>)
        }
    }
    #[doc = "0x3f - CSE PRAM15HU register."]
    #[inline(always)]
    pub fn _embedded_ram15hu(&self) -> &crate::Reg<_embedded_ram15hu::_EMBEDDEDRAM15HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(63usize)
                as *const crate::Reg<_embedded_ram15hu::_EMBEDDEDRAM15HU_SPEC>)
        }
    }
    #[doc = "0x40 - CSE PRAM16LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram16ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram16ll::CSE_PRAM__EMBEDDEDRAM16LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<cse_pram__embedded_ram16ll::CSE_PRAM__EMBEDDEDRAM16LL_SPEC>)
        }
    }
    #[doc = "0x40 - CSE PRAM 16 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram16(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram16::CSE_PRAM__EMBEDDEDRAM16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<cse_pram__embedded_ram16::CSE_PRAM__EMBEDDEDRAM16_SPEC>)
        }
    }
    #[doc = "0x41 - CSE PRAM16LU register."]
    #[inline(always)]
    pub fn _embedded_ram16lu(&self) -> &crate::Reg<_embedded_ram16lu::_EMBEDDEDRAM16LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(65usize)
                as *const crate::Reg<_embedded_ram16lu::_EMBEDDEDRAM16LU_SPEC>)
        }
    }
    #[doc = "0x42 - CSE PRAM16HL register."]
    #[inline(always)]
    pub fn _embedded_ram16hl(&self) -> &crate::Reg<_embedded_ram16hl::_EMBEDDEDRAM16HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(66usize)
                as *const crate::Reg<_embedded_ram16hl::_EMBEDDEDRAM16HL_SPEC>)
        }
    }
    #[doc = "0x43 - CSE PRAM16HU register."]
    #[inline(always)]
    pub fn _embedded_ram16hu(&self) -> &crate::Reg<_embedded_ram16hu::_EMBEDDEDRAM16HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(67usize)
                as *const crate::Reg<_embedded_ram16hu::_EMBEDDEDRAM16HU_SPEC>)
        }
    }
    #[doc = "0x44 - CSE PRAM17LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram17ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram17ll::CSE_PRAM__EMBEDDEDRAM17LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<cse_pram__embedded_ram17ll::CSE_PRAM__EMBEDDEDRAM17LL_SPEC>)
        }
    }
    #[doc = "0x44 - CSE PRAM 17 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram17(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram17::CSE_PRAM__EMBEDDEDRAM17_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<cse_pram__embedded_ram17::CSE_PRAM__EMBEDDEDRAM17_SPEC>)
        }
    }
    #[doc = "0x45 - CSE PRAM17LU register."]
    #[inline(always)]
    pub fn _embedded_ram17lu(&self) -> &crate::Reg<_embedded_ram17lu::_EMBEDDEDRAM17LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(69usize)
                as *const crate::Reg<_embedded_ram17lu::_EMBEDDEDRAM17LU_SPEC>)
        }
    }
    #[doc = "0x46 - CSE PRAM17HL register."]
    #[inline(always)]
    pub fn _embedded_ram17hl(&self) -> &crate::Reg<_embedded_ram17hl::_EMBEDDEDRAM17HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(70usize)
                as *const crate::Reg<_embedded_ram17hl::_EMBEDDEDRAM17HL_SPEC>)
        }
    }
    #[doc = "0x47 - CSE PRAM17HU register."]
    #[inline(always)]
    pub fn _embedded_ram17hu(&self) -> &crate::Reg<_embedded_ram17hu::_EMBEDDEDRAM17HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(71usize)
                as *const crate::Reg<_embedded_ram17hu::_EMBEDDEDRAM17HU_SPEC>)
        }
    }
    #[doc = "0x48 - CSE PRAM18LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram18ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram18ll::CSE_PRAM__EMBEDDEDRAM18LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const crate::Reg<cse_pram__embedded_ram18ll::CSE_PRAM__EMBEDDEDRAM18LL_SPEC>)
        }
    }
    #[doc = "0x48 - CSE PRAM 18 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram18(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram18::CSE_PRAM__EMBEDDEDRAM18_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const crate::Reg<cse_pram__embedded_ram18::CSE_PRAM__EMBEDDEDRAM18_SPEC>)
        }
    }
    #[doc = "0x49 - CSE PRAM18LU register."]
    #[inline(always)]
    pub fn _embedded_ram18lu(&self) -> &crate::Reg<_embedded_ram18lu::_EMBEDDEDRAM18LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(73usize)
                as *const crate::Reg<_embedded_ram18lu::_EMBEDDEDRAM18LU_SPEC>)
        }
    }
    #[doc = "0x4a - CSE PRAM18HL register."]
    #[inline(always)]
    pub fn _embedded_ram18hl(&self) -> &crate::Reg<_embedded_ram18hl::_EMBEDDEDRAM18HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(74usize)
                as *const crate::Reg<_embedded_ram18hl::_EMBEDDEDRAM18HL_SPEC>)
        }
    }
    #[doc = "0x4b - CSE PRAM18HU register."]
    #[inline(always)]
    pub fn _embedded_ram18hu(&self) -> &crate::Reg<_embedded_ram18hu::_EMBEDDEDRAM18HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(75usize)
                as *const crate::Reg<_embedded_ram18hu::_EMBEDDEDRAM18HU_SPEC>)
        }
    }
    #[doc = "0x4c - CSE PRAM19LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram19ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram19ll::CSE_PRAM__EMBEDDEDRAM19LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<cse_pram__embedded_ram19ll::CSE_PRAM__EMBEDDEDRAM19LL_SPEC>)
        }
    }
    #[doc = "0x4c - CSE PRAM 19 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram19(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram19::CSE_PRAM__EMBEDDEDRAM19_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const crate::Reg<cse_pram__embedded_ram19::CSE_PRAM__EMBEDDEDRAM19_SPEC>)
        }
    }
    #[doc = "0x4d - CSE PRAM19LU register."]
    #[inline(always)]
    pub fn _embedded_ram19lu(&self) -> &crate::Reg<_embedded_ram19lu::_EMBEDDEDRAM19LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(77usize)
                as *const crate::Reg<_embedded_ram19lu::_EMBEDDEDRAM19LU_SPEC>)
        }
    }
    #[doc = "0x4e - CSE PRAM19HL register."]
    #[inline(always)]
    pub fn _embedded_ram19hl(&self) -> &crate::Reg<_embedded_ram19hl::_EMBEDDEDRAM19HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(78usize)
                as *const crate::Reg<_embedded_ram19hl::_EMBEDDEDRAM19HL_SPEC>)
        }
    }
    #[doc = "0x4f - CSE PRAM19HU register."]
    #[inline(always)]
    pub fn _embedded_ram19hu(&self) -> &crate::Reg<_embedded_ram19hu::_EMBEDDEDRAM19HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(79usize)
                as *const crate::Reg<_embedded_ram19hu::_EMBEDDEDRAM19HU_SPEC>)
        }
    }
    #[doc = "0x50 - CSE PRAM20LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram20ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram20ll::CSE_PRAM__EMBEDDEDRAM20LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize)
                as *const crate::Reg<cse_pram__embedded_ram20ll::CSE_PRAM__EMBEDDEDRAM20LL_SPEC>)
        }
    }
    #[doc = "0x50 - CSE PRAM 20 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram20(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram20::CSE_PRAM__EMBEDDEDRAM20_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize)
                as *const crate::Reg<cse_pram__embedded_ram20::CSE_PRAM__EMBEDDEDRAM20_SPEC>)
        }
    }
    #[doc = "0x51 - CSE PRAM20LU register."]
    #[inline(always)]
    pub fn _embedded_ram20lu(&self) -> &crate::Reg<_embedded_ram20lu::_EMBEDDEDRAM20LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(81usize)
                as *const crate::Reg<_embedded_ram20lu::_EMBEDDEDRAM20LU_SPEC>)
        }
    }
    #[doc = "0x52 - CSE PRAM20HL register."]
    #[inline(always)]
    pub fn _embedded_ram20hl(&self) -> &crate::Reg<_embedded_ram20hl::_EMBEDDEDRAM20HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(82usize)
                as *const crate::Reg<_embedded_ram20hl::_EMBEDDEDRAM20HL_SPEC>)
        }
    }
    #[doc = "0x53 - CSE PRAM20HU register."]
    #[inline(always)]
    pub fn _embedded_ram20hu(&self) -> &crate::Reg<_embedded_ram20hu::_EMBEDDEDRAM20HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(83usize)
                as *const crate::Reg<_embedded_ram20hu::_EMBEDDEDRAM20HU_SPEC>)
        }
    }
    #[doc = "0x54 - CSE PRAM21LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram21ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram21ll::CSE_PRAM__EMBEDDEDRAM21LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const crate::Reg<cse_pram__embedded_ram21ll::CSE_PRAM__EMBEDDEDRAM21LL_SPEC>)
        }
    }
    #[doc = "0x54 - CSE PRAM 21 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram21(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram21::CSE_PRAM__EMBEDDEDRAM21_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const crate::Reg<cse_pram__embedded_ram21::CSE_PRAM__EMBEDDEDRAM21_SPEC>)
        }
    }
    #[doc = "0x55 - CSE PRAM21LU register."]
    #[inline(always)]
    pub fn _embedded_ram21lu(&self) -> &crate::Reg<_embedded_ram21lu::_EMBEDDEDRAM21LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(85usize)
                as *const crate::Reg<_embedded_ram21lu::_EMBEDDEDRAM21LU_SPEC>)
        }
    }
    #[doc = "0x56 - CSE PRAM21HL register."]
    #[inline(always)]
    pub fn _embedded_ram21hl(&self) -> &crate::Reg<_embedded_ram21hl::_EMBEDDEDRAM21HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(86usize)
                as *const crate::Reg<_embedded_ram21hl::_EMBEDDEDRAM21HL_SPEC>)
        }
    }
    #[doc = "0x57 - CSE PRAM21HU register."]
    #[inline(always)]
    pub fn _embedded_ram21hu(&self) -> &crate::Reg<_embedded_ram21hu::_EMBEDDEDRAM21HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(87usize)
                as *const crate::Reg<_embedded_ram21hu::_EMBEDDEDRAM21HU_SPEC>)
        }
    }
    #[doc = "0x58 - CSE PRAM22LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram22ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram22ll::CSE_PRAM__EMBEDDEDRAM22LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize)
                as *const crate::Reg<cse_pram__embedded_ram22ll::CSE_PRAM__EMBEDDEDRAM22LL_SPEC>)
        }
    }
    #[doc = "0x58 - CSE PRAM 22 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram22(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram22::CSE_PRAM__EMBEDDEDRAM22_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize)
                as *const crate::Reg<cse_pram__embedded_ram22::CSE_PRAM__EMBEDDEDRAM22_SPEC>)
        }
    }
    #[doc = "0x59 - CSE PRAM22LU register."]
    #[inline(always)]
    pub fn _embedded_ram22lu(&self) -> &crate::Reg<_embedded_ram22lu::_EMBEDDEDRAM22LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(89usize)
                as *const crate::Reg<_embedded_ram22lu::_EMBEDDEDRAM22LU_SPEC>)
        }
    }
    #[doc = "0x5a - CSE PRAM22HL register."]
    #[inline(always)]
    pub fn _embedded_ram22hl(&self) -> &crate::Reg<_embedded_ram22hl::_EMBEDDEDRAM22HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(90usize)
                as *const crate::Reg<_embedded_ram22hl::_EMBEDDEDRAM22HL_SPEC>)
        }
    }
    #[doc = "0x5b - CSE PRAM22HU register."]
    #[inline(always)]
    pub fn _embedded_ram22hu(&self) -> &crate::Reg<_embedded_ram22hu::_EMBEDDEDRAM22HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(91usize)
                as *const crate::Reg<_embedded_ram22hu::_EMBEDDEDRAM22HU_SPEC>)
        }
    }
    #[doc = "0x5c - CSE PRAM23LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram23ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram23ll::CSE_PRAM__EMBEDDEDRAM23LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize)
                as *const crate::Reg<cse_pram__embedded_ram23ll::CSE_PRAM__EMBEDDEDRAM23LL_SPEC>)
        }
    }
    #[doc = "0x5c - CSE PRAM 23 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram23(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram23::CSE_PRAM__EMBEDDEDRAM23_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize)
                as *const crate::Reg<cse_pram__embedded_ram23::CSE_PRAM__EMBEDDEDRAM23_SPEC>)
        }
    }
    #[doc = "0x5d - CSE PRAM23LU register."]
    #[inline(always)]
    pub fn _embedded_ram23lu(&self) -> &crate::Reg<_embedded_ram23lu::_EMBEDDEDRAM23LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(93usize)
                as *const crate::Reg<_embedded_ram23lu::_EMBEDDEDRAM23LU_SPEC>)
        }
    }
    #[doc = "0x5e - CSE PRAM23HL register."]
    #[inline(always)]
    pub fn _embedded_ram23hl(&self) -> &crate::Reg<_embedded_ram23hl::_EMBEDDEDRAM23HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(94usize)
                as *const crate::Reg<_embedded_ram23hl::_EMBEDDEDRAM23HL_SPEC>)
        }
    }
    #[doc = "0x5f - CSE PRAM23HU register."]
    #[inline(always)]
    pub fn _embedded_ram23hu(&self) -> &crate::Reg<_embedded_ram23hu::_EMBEDDEDRAM23HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(95usize)
                as *const crate::Reg<_embedded_ram23hu::_EMBEDDEDRAM23HU_SPEC>)
        }
    }
    #[doc = "0x60 - CSE PRAM24LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram24ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram24ll::CSE_PRAM__EMBEDDEDRAM24LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<cse_pram__embedded_ram24ll::CSE_PRAM__EMBEDDEDRAM24LL_SPEC>)
        }
    }
    #[doc = "0x60 - CSE PRAM 24 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram24(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram24::CSE_PRAM__EMBEDDEDRAM24_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(96usize)
                as *const crate::Reg<cse_pram__embedded_ram24::CSE_PRAM__EMBEDDEDRAM24_SPEC>)
        }
    }
    #[doc = "0x61 - CSE PRAM24LU register."]
    #[inline(always)]
    pub fn _embedded_ram24lu(&self) -> &crate::Reg<_embedded_ram24lu::_EMBEDDEDRAM24LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(97usize)
                as *const crate::Reg<_embedded_ram24lu::_EMBEDDEDRAM24LU_SPEC>)
        }
    }
    #[doc = "0x62 - CSE PRAM24HL register."]
    #[inline(always)]
    pub fn _embedded_ram24hl(&self) -> &crate::Reg<_embedded_ram24hl::_EMBEDDEDRAM24HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(98usize)
                as *const crate::Reg<_embedded_ram24hl::_EMBEDDEDRAM24HL_SPEC>)
        }
    }
    #[doc = "0x63 - CSE PRAM24HU register."]
    #[inline(always)]
    pub fn _embedded_ram24hu(&self) -> &crate::Reg<_embedded_ram24hu::_EMBEDDEDRAM24HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(99usize)
                as *const crate::Reg<_embedded_ram24hu::_EMBEDDEDRAM24HU_SPEC>)
        }
    }
    #[doc = "0x64 - CSE PRAM25LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram25ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram25ll::CSE_PRAM__EMBEDDEDRAM25LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(100usize)
                as *const crate::Reg<cse_pram__embedded_ram25ll::CSE_PRAM__EMBEDDEDRAM25LL_SPEC>)
        }
    }
    #[doc = "0x64 - CSE PRAM 25 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram25(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram25::CSE_PRAM__EMBEDDEDRAM25_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(100usize)
                as *const crate::Reg<cse_pram__embedded_ram25::CSE_PRAM__EMBEDDEDRAM25_SPEC>)
        }
    }
    #[doc = "0x65 - CSE PRAM25LU register."]
    #[inline(always)]
    pub fn _embedded_ram25lu(&self) -> &crate::Reg<_embedded_ram25lu::_EMBEDDEDRAM25LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(101usize)
                as *const crate::Reg<_embedded_ram25lu::_EMBEDDEDRAM25LU_SPEC>)
        }
    }
    #[doc = "0x66 - CSE PRAM25HL register."]
    #[inline(always)]
    pub fn _embedded_ram25hl(&self) -> &crate::Reg<_embedded_ram25hl::_EMBEDDEDRAM25HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(102usize)
                as *const crate::Reg<_embedded_ram25hl::_EMBEDDEDRAM25HL_SPEC>)
        }
    }
    #[doc = "0x67 - CSE PRAM25HU register."]
    #[inline(always)]
    pub fn _embedded_ram25hu(&self) -> &crate::Reg<_embedded_ram25hu::_EMBEDDEDRAM25HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(103usize)
                as *const crate::Reg<_embedded_ram25hu::_EMBEDDEDRAM25HU_SPEC>)
        }
    }
    #[doc = "0x68 - CSE PRAM26LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram26ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram26ll::CSE_PRAM__EMBEDDEDRAM26LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(104usize)
                as *const crate::Reg<cse_pram__embedded_ram26ll::CSE_PRAM__EMBEDDEDRAM26LL_SPEC>)
        }
    }
    #[doc = "0x68 - CSE PRAM 26 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram26(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram26::CSE_PRAM__EMBEDDEDRAM26_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(104usize)
                as *const crate::Reg<cse_pram__embedded_ram26::CSE_PRAM__EMBEDDEDRAM26_SPEC>)
        }
    }
    #[doc = "0x69 - CSE PRAM26LU register."]
    #[inline(always)]
    pub fn _embedded_ram26lu(&self) -> &crate::Reg<_embedded_ram26lu::_EMBEDDEDRAM26LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(105usize)
                as *const crate::Reg<_embedded_ram26lu::_EMBEDDEDRAM26LU_SPEC>)
        }
    }
    #[doc = "0x6a - CSE PRAM26HL register."]
    #[inline(always)]
    pub fn _embedded_ram26hl(&self) -> &crate::Reg<_embedded_ram26hl::_EMBEDDEDRAM26HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(106usize)
                as *const crate::Reg<_embedded_ram26hl::_EMBEDDEDRAM26HL_SPEC>)
        }
    }
    #[doc = "0x6b - CSE PRAM26HU register."]
    #[inline(always)]
    pub fn _embedded_ram26hu(&self) -> &crate::Reg<_embedded_ram26hu::_EMBEDDEDRAM26HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(107usize)
                as *const crate::Reg<_embedded_ram26hu::_EMBEDDEDRAM26HU_SPEC>)
        }
    }
    #[doc = "0x6c - CSE PRAM27LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram27ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram27ll::CSE_PRAM__EMBEDDEDRAM27LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<cse_pram__embedded_ram27ll::CSE_PRAM__EMBEDDEDRAM27LL_SPEC>)
        }
    }
    #[doc = "0x6c - CSE PRAM 27 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram27(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram27::CSE_PRAM__EMBEDDEDRAM27_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize)
                as *const crate::Reg<cse_pram__embedded_ram27::CSE_PRAM__EMBEDDEDRAM27_SPEC>)
        }
    }
    #[doc = "0x6d - CSE PRAM27LU register."]
    #[inline(always)]
    pub fn _embedded_ram27lu(&self) -> &crate::Reg<_embedded_ram27lu::_EMBEDDEDRAM27LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(109usize)
                as *const crate::Reg<_embedded_ram27lu::_EMBEDDEDRAM27LU_SPEC>)
        }
    }
    #[doc = "0x6e - CSE PRAM27HL register."]
    #[inline(always)]
    pub fn _embedded_ram27hl(&self) -> &crate::Reg<_embedded_ram27hl::_EMBEDDEDRAM27HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(110usize)
                as *const crate::Reg<_embedded_ram27hl::_EMBEDDEDRAM27HL_SPEC>)
        }
    }
    #[doc = "0x6f - CSE PRAM27HU register."]
    #[inline(always)]
    pub fn _embedded_ram27hu(&self) -> &crate::Reg<_embedded_ram27hu::_EMBEDDEDRAM27HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(111usize)
                as *const crate::Reg<_embedded_ram27hu::_EMBEDDEDRAM27HU_SPEC>)
        }
    }
    #[doc = "0x70 - CSE PRAM28LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram28ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram28ll::CSE_PRAM__EMBEDDEDRAM28LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const crate::Reg<cse_pram__embedded_ram28ll::CSE_PRAM__EMBEDDEDRAM28LL_SPEC>)
        }
    }
    #[doc = "0x70 - CSE PRAM 28 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram28(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram28::CSE_PRAM__EMBEDDEDRAM28_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize)
                as *const crate::Reg<cse_pram__embedded_ram28::CSE_PRAM__EMBEDDEDRAM28_SPEC>)
        }
    }
    #[doc = "0x71 - CSE PRAM28LU register."]
    #[inline(always)]
    pub fn _embedded_ram28lu(&self) -> &crate::Reg<_embedded_ram28lu::_EMBEDDEDRAM28LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(113usize)
                as *const crate::Reg<_embedded_ram28lu::_EMBEDDEDRAM28LU_SPEC>)
        }
    }
    #[doc = "0x72 - CSE PRAM28HL register."]
    #[inline(always)]
    pub fn _embedded_ram28hl(&self) -> &crate::Reg<_embedded_ram28hl::_EMBEDDEDRAM28HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(114usize)
                as *const crate::Reg<_embedded_ram28hl::_EMBEDDEDRAM28HL_SPEC>)
        }
    }
    #[doc = "0x73 - CSE PRAM28HU register."]
    #[inline(always)]
    pub fn _embedded_ram28hu(&self) -> &crate::Reg<_embedded_ram28hu::_EMBEDDEDRAM28HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(115usize)
                as *const crate::Reg<_embedded_ram28hu::_EMBEDDEDRAM28HU_SPEC>)
        }
    }
    #[doc = "0x74 - CSE PRAM29LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram29ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram29ll::CSE_PRAM__EMBEDDEDRAM29LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(116usize)
                as *const crate::Reg<cse_pram__embedded_ram29ll::CSE_PRAM__EMBEDDEDRAM29LL_SPEC>)
        }
    }
    #[doc = "0x74 - CSE PRAM 29 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram29(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram29::CSE_PRAM__EMBEDDEDRAM29_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(116usize)
                as *const crate::Reg<cse_pram__embedded_ram29::CSE_PRAM__EMBEDDEDRAM29_SPEC>)
        }
    }
    #[doc = "0x75 - CSE PRAM29LU register."]
    #[inline(always)]
    pub fn _embedded_ram29lu(&self) -> &crate::Reg<_embedded_ram29lu::_EMBEDDEDRAM29LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(117usize)
                as *const crate::Reg<_embedded_ram29lu::_EMBEDDEDRAM29LU_SPEC>)
        }
    }
    #[doc = "0x76 - CSE PRAM29HL register."]
    #[inline(always)]
    pub fn _embedded_ram29hl(&self) -> &crate::Reg<_embedded_ram29hl::_EMBEDDEDRAM29HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(118usize)
                as *const crate::Reg<_embedded_ram29hl::_EMBEDDEDRAM29HL_SPEC>)
        }
    }
    #[doc = "0x77 - CSE PRAM29HU register."]
    #[inline(always)]
    pub fn _embedded_ram29hu(&self) -> &crate::Reg<_embedded_ram29hu::_EMBEDDEDRAM29HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(119usize)
                as *const crate::Reg<_embedded_ram29hu::_EMBEDDEDRAM29HU_SPEC>)
        }
    }
    #[doc = "0x78 - CSE PRAM30LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram30ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram30ll::CSE_PRAM__EMBEDDEDRAM30LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(120usize)
                as *const crate::Reg<cse_pram__embedded_ram30ll::CSE_PRAM__EMBEDDEDRAM30LL_SPEC>)
        }
    }
    #[doc = "0x78 - CSE PRAM 30 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram30(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram30::CSE_PRAM__EMBEDDEDRAM30_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(120usize)
                as *const crate::Reg<cse_pram__embedded_ram30::CSE_PRAM__EMBEDDEDRAM30_SPEC>)
        }
    }
    #[doc = "0x79 - CSE PRAM30LU register."]
    #[inline(always)]
    pub fn _embedded_ram30lu(&self) -> &crate::Reg<_embedded_ram30lu::_EMBEDDEDRAM30LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(121usize)
                as *const crate::Reg<_embedded_ram30lu::_EMBEDDEDRAM30LU_SPEC>)
        }
    }
    #[doc = "0x7a - CSE PRAM30HL register."]
    #[inline(always)]
    pub fn _embedded_ram30hl(&self) -> &crate::Reg<_embedded_ram30hl::_EMBEDDEDRAM30HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(122usize)
                as *const crate::Reg<_embedded_ram30hl::_EMBEDDEDRAM30HL_SPEC>)
        }
    }
    #[doc = "0x7b - CSE PRAM30HU register."]
    #[inline(always)]
    pub fn _embedded_ram30hu(&self) -> &crate::Reg<_embedded_ram30hu::_EMBEDDEDRAM30HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(123usize)
                as *const crate::Reg<_embedded_ram30hu::_EMBEDDEDRAM30HU_SPEC>)
        }
    }
    #[doc = "0x7c - CSE PRAM31LL register."]
    #[inline(always)]
    pub fn cse_pram__embedded_ram31ll(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram31ll::CSE_PRAM__EMBEDDEDRAM31LL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(124usize)
                as *const crate::Reg<cse_pram__embedded_ram31ll::CSE_PRAM__EMBEDDEDRAM31LL_SPEC>)
        }
    }
    #[doc = "0x7c - CSE PRAM 31 Register"]
    #[inline(always)]
    pub fn cse_pram__embedded_ram31(
        &self,
    ) -> &crate::Reg<cse_pram__embedded_ram31::CSE_PRAM__EMBEDDEDRAM31_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(124usize)
                as *const crate::Reg<cse_pram__embedded_ram31::CSE_PRAM__EMBEDDEDRAM31_SPEC>)
        }
    }
    #[doc = "0x7d - CSE PRAM31LU register."]
    #[inline(always)]
    pub fn _embedded_ram31lu(&self) -> &crate::Reg<_embedded_ram31lu::_EMBEDDEDRAM31LU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(125usize)
                as *const crate::Reg<_embedded_ram31lu::_EMBEDDEDRAM31LU_SPEC>)
        }
    }
    #[doc = "0x7e - CSE PRAM31HL register."]
    #[inline(always)]
    pub fn _embedded_ram31hl(&self) -> &crate::Reg<_embedded_ram31hl::_EMBEDDEDRAM31HL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(126usize)
                as *const crate::Reg<_embedded_ram31hl::_EMBEDDEDRAM31HL_SPEC>)
        }
    }
    #[doc = "0x7f - CSE PRAM31HU register."]
    #[inline(always)]
    pub fn _embedded_ram31hu(&self) -> &crate::Reg<_embedded_ram31hu::_EMBEDDEDRAM31HU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(127usize)
                as *const crate::Reg<_embedded_ram31hu::_EMBEDDEDRAM31HU_SPEC>)
        }
    }
}
#[doc = "CSE_PRAM__EmbeddedRAM0 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM0_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM0 = crate::Reg<cse_pram__embedded_ram0::CSE_PRAM__EMBEDDEDRAM0_SPEC>;
#[doc = "CSE PRAM 0 Register"]
pub mod cse_pram__embedded_ram0;
#[doc = "CSE_PRAM__EmbeddedRAM0LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM0LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM0LL =
    crate::Reg<cse_pram__embedded_ram0ll::CSE_PRAM__EMBEDDEDRAM0LL_SPEC>;
#[doc = "CSE PRAM0LL register."]
pub mod cse_pram__embedded_ram0ll;
#[doc = "_EmbeddedRAM0LU register accessor: an alias for `Reg<_EMBEDDEDRAM0LU_SPEC>`"]
pub type _EMBEDDEDRAM0LU = crate::Reg<_embedded_ram0lu::_EMBEDDEDRAM0LU_SPEC>;
#[doc = "CSE PRAM0LU register."]
pub mod _embedded_ram0lu;
#[doc = "_EmbeddedRAM0HL register accessor: an alias for `Reg<_EMBEDDEDRAM0HL_SPEC>`"]
pub type _EMBEDDEDRAM0HL = crate::Reg<_embedded_ram0hl::_EMBEDDEDRAM0HL_SPEC>;
#[doc = "CSE PRAM0HL register."]
pub mod _embedded_ram0hl;
#[doc = "_EmbeddedRAM0HU register accessor: an alias for `Reg<_EMBEDDEDRAM0HU_SPEC>`"]
pub type _EMBEDDEDRAM0HU = crate::Reg<_embedded_ram0hu::_EMBEDDEDRAM0HU_SPEC>;
#[doc = "CSE PRAM0HU register."]
pub mod _embedded_ram0hu;
#[doc = "CSE_PRAM__EmbeddedRAM1 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM1_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM1 = crate::Reg<cse_pram__embedded_ram1::CSE_PRAM__EMBEDDEDRAM1_SPEC>;
#[doc = "CSE PRAM 1 Register"]
pub mod cse_pram__embedded_ram1;
#[doc = "CSE_PRAM__EmbeddedRAM1LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM1LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM1LL =
    crate::Reg<cse_pram__embedded_ram1ll::CSE_PRAM__EMBEDDEDRAM1LL_SPEC>;
#[doc = "CSE PRAM1LL register."]
pub mod cse_pram__embedded_ram1ll;
#[doc = "_EmbeddedRAM1LU register accessor: an alias for `Reg<_EMBEDDEDRAM1LU_SPEC>`"]
pub type _EMBEDDEDRAM1LU = crate::Reg<_embedded_ram1lu::_EMBEDDEDRAM1LU_SPEC>;
#[doc = "CSE PRAM1LU register."]
pub mod _embedded_ram1lu;
#[doc = "_EmbeddedRAM1HL register accessor: an alias for `Reg<_EMBEDDEDRAM1HL_SPEC>`"]
pub type _EMBEDDEDRAM1HL = crate::Reg<_embedded_ram1hl::_EMBEDDEDRAM1HL_SPEC>;
#[doc = "CSE PRAM1HL register."]
pub mod _embedded_ram1hl;
#[doc = "_EmbeddedRAM1HU register accessor: an alias for `Reg<_EMBEDDEDRAM1HU_SPEC>`"]
pub type _EMBEDDEDRAM1HU = crate::Reg<_embedded_ram1hu::_EMBEDDEDRAM1HU_SPEC>;
#[doc = "CSE PRAM1HU register."]
pub mod _embedded_ram1hu;
#[doc = "CSE_PRAM__EmbeddedRAM2 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM2_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM2 = crate::Reg<cse_pram__embedded_ram2::CSE_PRAM__EMBEDDEDRAM2_SPEC>;
#[doc = "CSE PRAM 2 Register"]
pub mod cse_pram__embedded_ram2;
#[doc = "CSE_PRAM__EmbeddedRAM2LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM2LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM2LL =
    crate::Reg<cse_pram__embedded_ram2ll::CSE_PRAM__EMBEDDEDRAM2LL_SPEC>;
#[doc = "CSE PRAM2LL register."]
pub mod cse_pram__embedded_ram2ll;
#[doc = "_EmbeddedRAM2LU register accessor: an alias for `Reg<_EMBEDDEDRAM2LU_SPEC>`"]
pub type _EMBEDDEDRAM2LU = crate::Reg<_embedded_ram2lu::_EMBEDDEDRAM2LU_SPEC>;
#[doc = "CSE PRAM2LU register."]
pub mod _embedded_ram2lu;
#[doc = "_EmbeddedRAM2HL register accessor: an alias for `Reg<_EMBEDDEDRAM2HL_SPEC>`"]
pub type _EMBEDDEDRAM2HL = crate::Reg<_embedded_ram2hl::_EMBEDDEDRAM2HL_SPEC>;
#[doc = "CSE PRAM2HL register."]
pub mod _embedded_ram2hl;
#[doc = "_EmbeddedRAM2HU register accessor: an alias for `Reg<_EMBEDDEDRAM2HU_SPEC>`"]
pub type _EMBEDDEDRAM2HU = crate::Reg<_embedded_ram2hu::_EMBEDDEDRAM2HU_SPEC>;
#[doc = "CSE PRAM2HU register."]
pub mod _embedded_ram2hu;
#[doc = "CSE_PRAM__EmbeddedRAM3 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM3_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM3 = crate::Reg<cse_pram__embedded_ram3::CSE_PRAM__EMBEDDEDRAM3_SPEC>;
#[doc = "CSE PRAM 3 Register"]
pub mod cse_pram__embedded_ram3;
#[doc = "CSE_PRAM__EmbeddedRAM3LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM3LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM3LL =
    crate::Reg<cse_pram__embedded_ram3ll::CSE_PRAM__EMBEDDEDRAM3LL_SPEC>;
#[doc = "CSE PRAM3LL register."]
pub mod cse_pram__embedded_ram3ll;
#[doc = "_EmbeddedRAM3LU register accessor: an alias for `Reg<_EMBEDDEDRAM3LU_SPEC>`"]
pub type _EMBEDDEDRAM3LU = crate::Reg<_embedded_ram3lu::_EMBEDDEDRAM3LU_SPEC>;
#[doc = "CSE PRAM3LU register."]
pub mod _embedded_ram3lu;
#[doc = "_EmbeddedRAM3HL register accessor: an alias for `Reg<_EMBEDDEDRAM3HL_SPEC>`"]
pub type _EMBEDDEDRAM3HL = crate::Reg<_embedded_ram3hl::_EMBEDDEDRAM3HL_SPEC>;
#[doc = "CSE PRAM3HL register."]
pub mod _embedded_ram3hl;
#[doc = "_EmbeddedRAM3HU register accessor: an alias for `Reg<_EMBEDDEDRAM3HU_SPEC>`"]
pub type _EMBEDDEDRAM3HU = crate::Reg<_embedded_ram3hu::_EMBEDDEDRAM3HU_SPEC>;
#[doc = "CSE PRAM3HU register."]
pub mod _embedded_ram3hu;
#[doc = "CSE_PRAM__EmbeddedRAM4 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM4_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM4 = crate::Reg<cse_pram__embedded_ram4::CSE_PRAM__EMBEDDEDRAM4_SPEC>;
#[doc = "CSE PRAM 4 Register"]
pub mod cse_pram__embedded_ram4;
#[doc = "CSE_PRAM__EmbeddedRAM4LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM4LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM4LL =
    crate::Reg<cse_pram__embedded_ram4ll::CSE_PRAM__EMBEDDEDRAM4LL_SPEC>;
#[doc = "CSE PRAM4LL register."]
pub mod cse_pram__embedded_ram4ll;
#[doc = "_EmbeddedRAM4LU register accessor: an alias for `Reg<_EMBEDDEDRAM4LU_SPEC>`"]
pub type _EMBEDDEDRAM4LU = crate::Reg<_embedded_ram4lu::_EMBEDDEDRAM4LU_SPEC>;
#[doc = "CSE PRAM4LU register."]
pub mod _embedded_ram4lu;
#[doc = "_EmbeddedRAM4HL register accessor: an alias for `Reg<_EMBEDDEDRAM4HL_SPEC>`"]
pub type _EMBEDDEDRAM4HL = crate::Reg<_embedded_ram4hl::_EMBEDDEDRAM4HL_SPEC>;
#[doc = "CSE PRAM4HL register."]
pub mod _embedded_ram4hl;
#[doc = "_EmbeddedRAM4HU register accessor: an alias for `Reg<_EMBEDDEDRAM4HU_SPEC>`"]
pub type _EMBEDDEDRAM4HU = crate::Reg<_embedded_ram4hu::_EMBEDDEDRAM4HU_SPEC>;
#[doc = "CSE PRAM4HU register."]
pub mod _embedded_ram4hu;
#[doc = "CSE_PRAM__EmbeddedRAM5 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM5_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM5 = crate::Reg<cse_pram__embedded_ram5::CSE_PRAM__EMBEDDEDRAM5_SPEC>;
#[doc = "CSE PRAM 5 Register"]
pub mod cse_pram__embedded_ram5;
#[doc = "CSE_PRAM__EmbeddedRAM5LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM5LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM5LL =
    crate::Reg<cse_pram__embedded_ram5ll::CSE_PRAM__EMBEDDEDRAM5LL_SPEC>;
#[doc = "CSE PRAM5LL register."]
pub mod cse_pram__embedded_ram5ll;
#[doc = "_EmbeddedRAM5LU register accessor: an alias for `Reg<_EMBEDDEDRAM5LU_SPEC>`"]
pub type _EMBEDDEDRAM5LU = crate::Reg<_embedded_ram5lu::_EMBEDDEDRAM5LU_SPEC>;
#[doc = "CSE PRAM5LU register."]
pub mod _embedded_ram5lu;
#[doc = "_EmbeddedRAM5HL register accessor: an alias for `Reg<_EMBEDDEDRAM5HL_SPEC>`"]
pub type _EMBEDDEDRAM5HL = crate::Reg<_embedded_ram5hl::_EMBEDDEDRAM5HL_SPEC>;
#[doc = "CSE PRAM5HL register."]
pub mod _embedded_ram5hl;
#[doc = "_EmbeddedRAM5HU register accessor: an alias for `Reg<_EMBEDDEDRAM5HU_SPEC>`"]
pub type _EMBEDDEDRAM5HU = crate::Reg<_embedded_ram5hu::_EMBEDDEDRAM5HU_SPEC>;
#[doc = "CSE PRAM5HU register."]
pub mod _embedded_ram5hu;
#[doc = "CSE_PRAM__EmbeddedRAM6 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM6_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM6 = crate::Reg<cse_pram__embedded_ram6::CSE_PRAM__EMBEDDEDRAM6_SPEC>;
#[doc = "CSE PRAM 6 Register"]
pub mod cse_pram__embedded_ram6;
#[doc = "CSE_PRAM__EmbeddedRAM6LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM6LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM6LL =
    crate::Reg<cse_pram__embedded_ram6ll::CSE_PRAM__EMBEDDEDRAM6LL_SPEC>;
#[doc = "CSE PRAM6LL register."]
pub mod cse_pram__embedded_ram6ll;
#[doc = "_EmbeddedRAM6LU register accessor: an alias for `Reg<_EMBEDDEDRAM6LU_SPEC>`"]
pub type _EMBEDDEDRAM6LU = crate::Reg<_embedded_ram6lu::_EMBEDDEDRAM6LU_SPEC>;
#[doc = "CSE PRAM6LU register."]
pub mod _embedded_ram6lu;
#[doc = "_EmbeddedRAM6HL register accessor: an alias for `Reg<_EMBEDDEDRAM6HL_SPEC>`"]
pub type _EMBEDDEDRAM6HL = crate::Reg<_embedded_ram6hl::_EMBEDDEDRAM6HL_SPEC>;
#[doc = "CSE PRAM6HL register."]
pub mod _embedded_ram6hl;
#[doc = "_EmbeddedRAM6HU register accessor: an alias for `Reg<_EMBEDDEDRAM6HU_SPEC>`"]
pub type _EMBEDDEDRAM6HU = crate::Reg<_embedded_ram6hu::_EMBEDDEDRAM6HU_SPEC>;
#[doc = "CSE PRAM6HU register."]
pub mod _embedded_ram6hu;
#[doc = "CSE_PRAM__EmbeddedRAM7 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM7_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM7 = crate::Reg<cse_pram__embedded_ram7::CSE_PRAM__EMBEDDEDRAM7_SPEC>;
#[doc = "CSE PRAM 7 Register"]
pub mod cse_pram__embedded_ram7;
#[doc = "CSE_PRAM__EmbeddedRAM7LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM7LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM7LL =
    crate::Reg<cse_pram__embedded_ram7ll::CSE_PRAM__EMBEDDEDRAM7LL_SPEC>;
#[doc = "CSE PRAM7LL register."]
pub mod cse_pram__embedded_ram7ll;
#[doc = "_EmbeddedRAM7LU register accessor: an alias for `Reg<_EMBEDDEDRAM7LU_SPEC>`"]
pub type _EMBEDDEDRAM7LU = crate::Reg<_embedded_ram7lu::_EMBEDDEDRAM7LU_SPEC>;
#[doc = "CSE PRAM7LU register."]
pub mod _embedded_ram7lu;
#[doc = "_EmbeddedRAM7HL register accessor: an alias for `Reg<_EMBEDDEDRAM7HL_SPEC>`"]
pub type _EMBEDDEDRAM7HL = crate::Reg<_embedded_ram7hl::_EMBEDDEDRAM7HL_SPEC>;
#[doc = "CSE PRAM7HL register."]
pub mod _embedded_ram7hl;
#[doc = "_EmbeddedRAM7HU register accessor: an alias for `Reg<_EMBEDDEDRAM7HU_SPEC>`"]
pub type _EMBEDDEDRAM7HU = crate::Reg<_embedded_ram7hu::_EMBEDDEDRAM7HU_SPEC>;
#[doc = "CSE PRAM7HU register."]
pub mod _embedded_ram7hu;
#[doc = "CSE_PRAM__EmbeddedRAM8 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM8_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM8 = crate::Reg<cse_pram__embedded_ram8::CSE_PRAM__EMBEDDEDRAM8_SPEC>;
#[doc = "CSE PRAM 8 Register"]
pub mod cse_pram__embedded_ram8;
#[doc = "CSE_PRAM__EmbeddedRAM8LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM8LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM8LL =
    crate::Reg<cse_pram__embedded_ram8ll::CSE_PRAM__EMBEDDEDRAM8LL_SPEC>;
#[doc = "CSE PRAM8LL register."]
pub mod cse_pram__embedded_ram8ll;
#[doc = "_EmbeddedRAM8LU register accessor: an alias for `Reg<_EMBEDDEDRAM8LU_SPEC>`"]
pub type _EMBEDDEDRAM8LU = crate::Reg<_embedded_ram8lu::_EMBEDDEDRAM8LU_SPEC>;
#[doc = "CSE PRAM8LU register."]
pub mod _embedded_ram8lu;
#[doc = "_EmbeddedRAM8HL register accessor: an alias for `Reg<_EMBEDDEDRAM8HL_SPEC>`"]
pub type _EMBEDDEDRAM8HL = crate::Reg<_embedded_ram8hl::_EMBEDDEDRAM8HL_SPEC>;
#[doc = "CSE PRAM8HL register."]
pub mod _embedded_ram8hl;
#[doc = "_EmbeddedRAM8HU register accessor: an alias for `Reg<_EMBEDDEDRAM8HU_SPEC>`"]
pub type _EMBEDDEDRAM8HU = crate::Reg<_embedded_ram8hu::_EMBEDDEDRAM8HU_SPEC>;
#[doc = "CSE PRAM8HU register."]
pub mod _embedded_ram8hu;
#[doc = "CSE_PRAM__EmbeddedRAM9 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM9_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM9 = crate::Reg<cse_pram__embedded_ram9::CSE_PRAM__EMBEDDEDRAM9_SPEC>;
#[doc = "CSE PRAM 9 Register"]
pub mod cse_pram__embedded_ram9;
#[doc = "CSE_PRAM__EmbeddedRAM9LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM9LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM9LL =
    crate::Reg<cse_pram__embedded_ram9ll::CSE_PRAM__EMBEDDEDRAM9LL_SPEC>;
#[doc = "CSE PRAM9LL register."]
pub mod cse_pram__embedded_ram9ll;
#[doc = "_EmbeddedRAM9LU register accessor: an alias for `Reg<_EMBEDDEDRAM9LU_SPEC>`"]
pub type _EMBEDDEDRAM9LU = crate::Reg<_embedded_ram9lu::_EMBEDDEDRAM9LU_SPEC>;
#[doc = "CSE PRAM9LU register."]
pub mod _embedded_ram9lu;
#[doc = "_EmbeddedRAM9HL register accessor: an alias for `Reg<_EMBEDDEDRAM9HL_SPEC>`"]
pub type _EMBEDDEDRAM9HL = crate::Reg<_embedded_ram9hl::_EMBEDDEDRAM9HL_SPEC>;
#[doc = "CSE PRAM9HL register."]
pub mod _embedded_ram9hl;
#[doc = "_EmbeddedRAM9HU register accessor: an alias for `Reg<_EMBEDDEDRAM9HU_SPEC>`"]
pub type _EMBEDDEDRAM9HU = crate::Reg<_embedded_ram9hu::_EMBEDDEDRAM9HU_SPEC>;
#[doc = "CSE PRAM9HU register."]
pub mod _embedded_ram9hu;
#[doc = "CSE_PRAM__EmbeddedRAM10 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM10_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM10 =
    crate::Reg<cse_pram__embedded_ram10::CSE_PRAM__EMBEDDEDRAM10_SPEC>;
#[doc = "CSE PRAM 10 Register"]
pub mod cse_pram__embedded_ram10;
#[doc = "CSE_PRAM__EmbeddedRAM10LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM10LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM10LL =
    crate::Reg<cse_pram__embedded_ram10ll::CSE_PRAM__EMBEDDEDRAM10LL_SPEC>;
#[doc = "CSE PRAM10LL register."]
pub mod cse_pram__embedded_ram10ll;
#[doc = "_EmbeddedRAM10LU register accessor: an alias for `Reg<_EMBEDDEDRAM10LU_SPEC>`"]
pub type _EMBEDDEDRAM10LU = crate::Reg<_embedded_ram10lu::_EMBEDDEDRAM10LU_SPEC>;
#[doc = "CSE PRAM10LU register."]
pub mod _embedded_ram10lu;
#[doc = "_EmbeddedRAM10HL register accessor: an alias for `Reg<_EMBEDDEDRAM10HL_SPEC>`"]
pub type _EMBEDDEDRAM10HL = crate::Reg<_embedded_ram10hl::_EMBEDDEDRAM10HL_SPEC>;
#[doc = "CSE PRAM10HL register."]
pub mod _embedded_ram10hl;
#[doc = "_EmbeddedRAM10HU register accessor: an alias for `Reg<_EMBEDDEDRAM10HU_SPEC>`"]
pub type _EMBEDDEDRAM10HU = crate::Reg<_embedded_ram10hu::_EMBEDDEDRAM10HU_SPEC>;
#[doc = "CSE PRAM10HU register."]
pub mod _embedded_ram10hu;
#[doc = "CSE_PRAM__EmbeddedRAM11 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM11_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM11 =
    crate::Reg<cse_pram__embedded_ram11::CSE_PRAM__EMBEDDEDRAM11_SPEC>;
#[doc = "CSE PRAM 11 Register"]
pub mod cse_pram__embedded_ram11;
#[doc = "CSE_PRAM__EmbeddedRAM11LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM11LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM11LL =
    crate::Reg<cse_pram__embedded_ram11ll::CSE_PRAM__EMBEDDEDRAM11LL_SPEC>;
#[doc = "CSE PRAM11LL register."]
pub mod cse_pram__embedded_ram11ll;
#[doc = "_EmbeddedRAM11LU register accessor: an alias for `Reg<_EMBEDDEDRAM11LU_SPEC>`"]
pub type _EMBEDDEDRAM11LU = crate::Reg<_embedded_ram11lu::_EMBEDDEDRAM11LU_SPEC>;
#[doc = "CSE PRAM11LU register."]
pub mod _embedded_ram11lu;
#[doc = "_EmbeddedRAM11HL register accessor: an alias for `Reg<_EMBEDDEDRAM11HL_SPEC>`"]
pub type _EMBEDDEDRAM11HL = crate::Reg<_embedded_ram11hl::_EMBEDDEDRAM11HL_SPEC>;
#[doc = "CSE PRAM11HL register."]
pub mod _embedded_ram11hl;
#[doc = "_EmbeddedRAM11HU register accessor: an alias for `Reg<_EMBEDDEDRAM11HU_SPEC>`"]
pub type _EMBEDDEDRAM11HU = crate::Reg<_embedded_ram11hu::_EMBEDDEDRAM11HU_SPEC>;
#[doc = "CSE PRAM11HU register."]
pub mod _embedded_ram11hu;
#[doc = "CSE_PRAM__EmbeddedRAM12 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM12_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM12 =
    crate::Reg<cse_pram__embedded_ram12::CSE_PRAM__EMBEDDEDRAM12_SPEC>;
#[doc = "CSE PRAM 12 Register"]
pub mod cse_pram__embedded_ram12;
#[doc = "CSE_PRAM__EmbeddedRAM12LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM12LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM12LL =
    crate::Reg<cse_pram__embedded_ram12ll::CSE_PRAM__EMBEDDEDRAM12LL_SPEC>;
#[doc = "CSE PRAM12LL register."]
pub mod cse_pram__embedded_ram12ll;
#[doc = "_EmbeddedRAM12LU register accessor: an alias for `Reg<_EMBEDDEDRAM12LU_SPEC>`"]
pub type _EMBEDDEDRAM12LU = crate::Reg<_embedded_ram12lu::_EMBEDDEDRAM12LU_SPEC>;
#[doc = "CSE PRAM12LU register."]
pub mod _embedded_ram12lu;
#[doc = "_EmbeddedRAM12HL register accessor: an alias for `Reg<_EMBEDDEDRAM12HL_SPEC>`"]
pub type _EMBEDDEDRAM12HL = crate::Reg<_embedded_ram12hl::_EMBEDDEDRAM12HL_SPEC>;
#[doc = "CSE PRAM12HL register."]
pub mod _embedded_ram12hl;
#[doc = "_EmbeddedRAM12HU register accessor: an alias for `Reg<_EMBEDDEDRAM12HU_SPEC>`"]
pub type _EMBEDDEDRAM12HU = crate::Reg<_embedded_ram12hu::_EMBEDDEDRAM12HU_SPEC>;
#[doc = "CSE PRAM12HU register."]
pub mod _embedded_ram12hu;
#[doc = "CSE_PRAM__EmbeddedRAM13 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM13_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM13 =
    crate::Reg<cse_pram__embedded_ram13::CSE_PRAM__EMBEDDEDRAM13_SPEC>;
#[doc = "CSE PRAM 13 Register"]
pub mod cse_pram__embedded_ram13;
#[doc = "CSE_PRAM__EmbeddedRAM13LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM13LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM13LL =
    crate::Reg<cse_pram__embedded_ram13ll::CSE_PRAM__EMBEDDEDRAM13LL_SPEC>;
#[doc = "CSE PRAM13LL register."]
pub mod cse_pram__embedded_ram13ll;
#[doc = "_EmbeddedRAM13LU register accessor: an alias for `Reg<_EMBEDDEDRAM13LU_SPEC>`"]
pub type _EMBEDDEDRAM13LU = crate::Reg<_embedded_ram13lu::_EMBEDDEDRAM13LU_SPEC>;
#[doc = "CSE PRAM13LU register."]
pub mod _embedded_ram13lu;
#[doc = "_EmbeddedRAM13HL register accessor: an alias for `Reg<_EMBEDDEDRAM13HL_SPEC>`"]
pub type _EMBEDDEDRAM13HL = crate::Reg<_embedded_ram13hl::_EMBEDDEDRAM13HL_SPEC>;
#[doc = "CSE PRAM13HL register."]
pub mod _embedded_ram13hl;
#[doc = "_EmbeddedRAM13HU register accessor: an alias for `Reg<_EMBEDDEDRAM13HU_SPEC>`"]
pub type _EMBEDDEDRAM13HU = crate::Reg<_embedded_ram13hu::_EMBEDDEDRAM13HU_SPEC>;
#[doc = "CSE PRAM13HU register."]
pub mod _embedded_ram13hu;
#[doc = "CSE_PRAM__EmbeddedRAM14 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM14_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM14 =
    crate::Reg<cse_pram__embedded_ram14::CSE_PRAM__EMBEDDEDRAM14_SPEC>;
#[doc = "CSE PRAM 14 Register"]
pub mod cse_pram__embedded_ram14;
#[doc = "CSE_PRAM__EmbeddedRAM14LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM14LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM14LL =
    crate::Reg<cse_pram__embedded_ram14ll::CSE_PRAM__EMBEDDEDRAM14LL_SPEC>;
#[doc = "CSE PRAM14LL register."]
pub mod cse_pram__embedded_ram14ll;
#[doc = "_EmbeddedRAM14LU register accessor: an alias for `Reg<_EMBEDDEDRAM14LU_SPEC>`"]
pub type _EMBEDDEDRAM14LU = crate::Reg<_embedded_ram14lu::_EMBEDDEDRAM14LU_SPEC>;
#[doc = "CSE PRAM14LU register."]
pub mod _embedded_ram14lu;
#[doc = "_EmbeddedRAM14HL register accessor: an alias for `Reg<_EMBEDDEDRAM14HL_SPEC>`"]
pub type _EMBEDDEDRAM14HL = crate::Reg<_embedded_ram14hl::_EMBEDDEDRAM14HL_SPEC>;
#[doc = "CSE PRAM14HL register."]
pub mod _embedded_ram14hl;
#[doc = "_EmbeddedRAM14HU register accessor: an alias for `Reg<_EMBEDDEDRAM14HU_SPEC>`"]
pub type _EMBEDDEDRAM14HU = crate::Reg<_embedded_ram14hu::_EMBEDDEDRAM14HU_SPEC>;
#[doc = "CSE PRAM14HU register."]
pub mod _embedded_ram14hu;
#[doc = "CSE_PRAM__EmbeddedRAM15 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM15_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM15 =
    crate::Reg<cse_pram__embedded_ram15::CSE_PRAM__EMBEDDEDRAM15_SPEC>;
#[doc = "CSE PRAM 15 Register"]
pub mod cse_pram__embedded_ram15;
#[doc = "CSE_PRAM__EmbeddedRAM15LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM15LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM15LL =
    crate::Reg<cse_pram__embedded_ram15ll::CSE_PRAM__EMBEDDEDRAM15LL_SPEC>;
#[doc = "CSE PRAM15LL register."]
pub mod cse_pram__embedded_ram15ll;
#[doc = "_EmbeddedRAM15LU register accessor: an alias for `Reg<_EMBEDDEDRAM15LU_SPEC>`"]
pub type _EMBEDDEDRAM15LU = crate::Reg<_embedded_ram15lu::_EMBEDDEDRAM15LU_SPEC>;
#[doc = "CSE PRAM15LU register."]
pub mod _embedded_ram15lu;
#[doc = "_EmbeddedRAM15HL register accessor: an alias for `Reg<_EMBEDDEDRAM15HL_SPEC>`"]
pub type _EMBEDDEDRAM15HL = crate::Reg<_embedded_ram15hl::_EMBEDDEDRAM15HL_SPEC>;
#[doc = "CSE PRAM15HL register."]
pub mod _embedded_ram15hl;
#[doc = "_EmbeddedRAM15HU register accessor: an alias for `Reg<_EMBEDDEDRAM15HU_SPEC>`"]
pub type _EMBEDDEDRAM15HU = crate::Reg<_embedded_ram15hu::_EMBEDDEDRAM15HU_SPEC>;
#[doc = "CSE PRAM15HU register."]
pub mod _embedded_ram15hu;
#[doc = "CSE_PRAM__EmbeddedRAM16 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM16_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM16 =
    crate::Reg<cse_pram__embedded_ram16::CSE_PRAM__EMBEDDEDRAM16_SPEC>;
#[doc = "CSE PRAM 16 Register"]
pub mod cse_pram__embedded_ram16;
#[doc = "CSE_PRAM__EmbeddedRAM16LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM16LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM16LL =
    crate::Reg<cse_pram__embedded_ram16ll::CSE_PRAM__EMBEDDEDRAM16LL_SPEC>;
#[doc = "CSE PRAM16LL register."]
pub mod cse_pram__embedded_ram16ll;
#[doc = "_EmbeddedRAM16LU register accessor: an alias for `Reg<_EMBEDDEDRAM16LU_SPEC>`"]
pub type _EMBEDDEDRAM16LU = crate::Reg<_embedded_ram16lu::_EMBEDDEDRAM16LU_SPEC>;
#[doc = "CSE PRAM16LU register."]
pub mod _embedded_ram16lu;
#[doc = "_EmbeddedRAM16HL register accessor: an alias for `Reg<_EMBEDDEDRAM16HL_SPEC>`"]
pub type _EMBEDDEDRAM16HL = crate::Reg<_embedded_ram16hl::_EMBEDDEDRAM16HL_SPEC>;
#[doc = "CSE PRAM16HL register."]
pub mod _embedded_ram16hl;
#[doc = "_EmbeddedRAM16HU register accessor: an alias for `Reg<_EMBEDDEDRAM16HU_SPEC>`"]
pub type _EMBEDDEDRAM16HU = crate::Reg<_embedded_ram16hu::_EMBEDDEDRAM16HU_SPEC>;
#[doc = "CSE PRAM16HU register."]
pub mod _embedded_ram16hu;
#[doc = "CSE_PRAM__EmbeddedRAM17 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM17_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM17 =
    crate::Reg<cse_pram__embedded_ram17::CSE_PRAM__EMBEDDEDRAM17_SPEC>;
#[doc = "CSE PRAM 17 Register"]
pub mod cse_pram__embedded_ram17;
#[doc = "CSE_PRAM__EmbeddedRAM17LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM17LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM17LL =
    crate::Reg<cse_pram__embedded_ram17ll::CSE_PRAM__EMBEDDEDRAM17LL_SPEC>;
#[doc = "CSE PRAM17LL register."]
pub mod cse_pram__embedded_ram17ll;
#[doc = "_EmbeddedRAM17LU register accessor: an alias for `Reg<_EMBEDDEDRAM17LU_SPEC>`"]
pub type _EMBEDDEDRAM17LU = crate::Reg<_embedded_ram17lu::_EMBEDDEDRAM17LU_SPEC>;
#[doc = "CSE PRAM17LU register."]
pub mod _embedded_ram17lu;
#[doc = "_EmbeddedRAM17HL register accessor: an alias for `Reg<_EMBEDDEDRAM17HL_SPEC>`"]
pub type _EMBEDDEDRAM17HL = crate::Reg<_embedded_ram17hl::_EMBEDDEDRAM17HL_SPEC>;
#[doc = "CSE PRAM17HL register."]
pub mod _embedded_ram17hl;
#[doc = "_EmbeddedRAM17HU register accessor: an alias for `Reg<_EMBEDDEDRAM17HU_SPEC>`"]
pub type _EMBEDDEDRAM17HU = crate::Reg<_embedded_ram17hu::_EMBEDDEDRAM17HU_SPEC>;
#[doc = "CSE PRAM17HU register."]
pub mod _embedded_ram17hu;
#[doc = "CSE_PRAM__EmbeddedRAM18 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM18_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM18 =
    crate::Reg<cse_pram__embedded_ram18::CSE_PRAM__EMBEDDEDRAM18_SPEC>;
#[doc = "CSE PRAM 18 Register"]
pub mod cse_pram__embedded_ram18;
#[doc = "CSE_PRAM__EmbeddedRAM18LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM18LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM18LL =
    crate::Reg<cse_pram__embedded_ram18ll::CSE_PRAM__EMBEDDEDRAM18LL_SPEC>;
#[doc = "CSE PRAM18LL register."]
pub mod cse_pram__embedded_ram18ll;
#[doc = "_EmbeddedRAM18LU register accessor: an alias for `Reg<_EMBEDDEDRAM18LU_SPEC>`"]
pub type _EMBEDDEDRAM18LU = crate::Reg<_embedded_ram18lu::_EMBEDDEDRAM18LU_SPEC>;
#[doc = "CSE PRAM18LU register."]
pub mod _embedded_ram18lu;
#[doc = "_EmbeddedRAM18HL register accessor: an alias for `Reg<_EMBEDDEDRAM18HL_SPEC>`"]
pub type _EMBEDDEDRAM18HL = crate::Reg<_embedded_ram18hl::_EMBEDDEDRAM18HL_SPEC>;
#[doc = "CSE PRAM18HL register."]
pub mod _embedded_ram18hl;
#[doc = "_EmbeddedRAM18HU register accessor: an alias for `Reg<_EMBEDDEDRAM18HU_SPEC>`"]
pub type _EMBEDDEDRAM18HU = crate::Reg<_embedded_ram18hu::_EMBEDDEDRAM18HU_SPEC>;
#[doc = "CSE PRAM18HU register."]
pub mod _embedded_ram18hu;
#[doc = "CSE_PRAM__EmbeddedRAM19 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM19_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM19 =
    crate::Reg<cse_pram__embedded_ram19::CSE_PRAM__EMBEDDEDRAM19_SPEC>;
#[doc = "CSE PRAM 19 Register"]
pub mod cse_pram__embedded_ram19;
#[doc = "CSE_PRAM__EmbeddedRAM19LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM19LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM19LL =
    crate::Reg<cse_pram__embedded_ram19ll::CSE_PRAM__EMBEDDEDRAM19LL_SPEC>;
#[doc = "CSE PRAM19LL register."]
pub mod cse_pram__embedded_ram19ll;
#[doc = "_EmbeddedRAM19LU register accessor: an alias for `Reg<_EMBEDDEDRAM19LU_SPEC>`"]
pub type _EMBEDDEDRAM19LU = crate::Reg<_embedded_ram19lu::_EMBEDDEDRAM19LU_SPEC>;
#[doc = "CSE PRAM19LU register."]
pub mod _embedded_ram19lu;
#[doc = "_EmbeddedRAM19HL register accessor: an alias for `Reg<_EMBEDDEDRAM19HL_SPEC>`"]
pub type _EMBEDDEDRAM19HL = crate::Reg<_embedded_ram19hl::_EMBEDDEDRAM19HL_SPEC>;
#[doc = "CSE PRAM19HL register."]
pub mod _embedded_ram19hl;
#[doc = "_EmbeddedRAM19HU register accessor: an alias for `Reg<_EMBEDDEDRAM19HU_SPEC>`"]
pub type _EMBEDDEDRAM19HU = crate::Reg<_embedded_ram19hu::_EMBEDDEDRAM19HU_SPEC>;
#[doc = "CSE PRAM19HU register."]
pub mod _embedded_ram19hu;
#[doc = "CSE_PRAM__EmbeddedRAM20 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM20_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM20 =
    crate::Reg<cse_pram__embedded_ram20::CSE_PRAM__EMBEDDEDRAM20_SPEC>;
#[doc = "CSE PRAM 20 Register"]
pub mod cse_pram__embedded_ram20;
#[doc = "CSE_PRAM__EmbeddedRAM20LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM20LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM20LL =
    crate::Reg<cse_pram__embedded_ram20ll::CSE_PRAM__EMBEDDEDRAM20LL_SPEC>;
#[doc = "CSE PRAM20LL register."]
pub mod cse_pram__embedded_ram20ll;
#[doc = "_EmbeddedRAM20LU register accessor: an alias for `Reg<_EMBEDDEDRAM20LU_SPEC>`"]
pub type _EMBEDDEDRAM20LU = crate::Reg<_embedded_ram20lu::_EMBEDDEDRAM20LU_SPEC>;
#[doc = "CSE PRAM20LU register."]
pub mod _embedded_ram20lu;
#[doc = "_EmbeddedRAM20HL register accessor: an alias for `Reg<_EMBEDDEDRAM20HL_SPEC>`"]
pub type _EMBEDDEDRAM20HL = crate::Reg<_embedded_ram20hl::_EMBEDDEDRAM20HL_SPEC>;
#[doc = "CSE PRAM20HL register."]
pub mod _embedded_ram20hl;
#[doc = "_EmbeddedRAM20HU register accessor: an alias for `Reg<_EMBEDDEDRAM20HU_SPEC>`"]
pub type _EMBEDDEDRAM20HU = crate::Reg<_embedded_ram20hu::_EMBEDDEDRAM20HU_SPEC>;
#[doc = "CSE PRAM20HU register."]
pub mod _embedded_ram20hu;
#[doc = "CSE_PRAM__EmbeddedRAM21 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM21_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM21 =
    crate::Reg<cse_pram__embedded_ram21::CSE_PRAM__EMBEDDEDRAM21_SPEC>;
#[doc = "CSE PRAM 21 Register"]
pub mod cse_pram__embedded_ram21;
#[doc = "CSE_PRAM__EmbeddedRAM21LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM21LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM21LL =
    crate::Reg<cse_pram__embedded_ram21ll::CSE_PRAM__EMBEDDEDRAM21LL_SPEC>;
#[doc = "CSE PRAM21LL register."]
pub mod cse_pram__embedded_ram21ll;
#[doc = "_EmbeddedRAM21LU register accessor: an alias for `Reg<_EMBEDDEDRAM21LU_SPEC>`"]
pub type _EMBEDDEDRAM21LU = crate::Reg<_embedded_ram21lu::_EMBEDDEDRAM21LU_SPEC>;
#[doc = "CSE PRAM21LU register."]
pub mod _embedded_ram21lu;
#[doc = "_EmbeddedRAM21HL register accessor: an alias for `Reg<_EMBEDDEDRAM21HL_SPEC>`"]
pub type _EMBEDDEDRAM21HL = crate::Reg<_embedded_ram21hl::_EMBEDDEDRAM21HL_SPEC>;
#[doc = "CSE PRAM21HL register."]
pub mod _embedded_ram21hl;
#[doc = "_EmbeddedRAM21HU register accessor: an alias for `Reg<_EMBEDDEDRAM21HU_SPEC>`"]
pub type _EMBEDDEDRAM21HU = crate::Reg<_embedded_ram21hu::_EMBEDDEDRAM21HU_SPEC>;
#[doc = "CSE PRAM21HU register."]
pub mod _embedded_ram21hu;
#[doc = "CSE_PRAM__EmbeddedRAM22 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM22_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM22 =
    crate::Reg<cse_pram__embedded_ram22::CSE_PRAM__EMBEDDEDRAM22_SPEC>;
#[doc = "CSE PRAM 22 Register"]
pub mod cse_pram__embedded_ram22;
#[doc = "CSE_PRAM__EmbeddedRAM22LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM22LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM22LL =
    crate::Reg<cse_pram__embedded_ram22ll::CSE_PRAM__EMBEDDEDRAM22LL_SPEC>;
#[doc = "CSE PRAM22LL register."]
pub mod cse_pram__embedded_ram22ll;
#[doc = "_EmbeddedRAM22LU register accessor: an alias for `Reg<_EMBEDDEDRAM22LU_SPEC>`"]
pub type _EMBEDDEDRAM22LU = crate::Reg<_embedded_ram22lu::_EMBEDDEDRAM22LU_SPEC>;
#[doc = "CSE PRAM22LU register."]
pub mod _embedded_ram22lu;
#[doc = "_EmbeddedRAM22HL register accessor: an alias for `Reg<_EMBEDDEDRAM22HL_SPEC>`"]
pub type _EMBEDDEDRAM22HL = crate::Reg<_embedded_ram22hl::_EMBEDDEDRAM22HL_SPEC>;
#[doc = "CSE PRAM22HL register."]
pub mod _embedded_ram22hl;
#[doc = "_EmbeddedRAM22HU register accessor: an alias for `Reg<_EMBEDDEDRAM22HU_SPEC>`"]
pub type _EMBEDDEDRAM22HU = crate::Reg<_embedded_ram22hu::_EMBEDDEDRAM22HU_SPEC>;
#[doc = "CSE PRAM22HU register."]
pub mod _embedded_ram22hu;
#[doc = "CSE_PRAM__EmbeddedRAM23 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM23_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM23 =
    crate::Reg<cse_pram__embedded_ram23::CSE_PRAM__EMBEDDEDRAM23_SPEC>;
#[doc = "CSE PRAM 23 Register"]
pub mod cse_pram__embedded_ram23;
#[doc = "CSE_PRAM__EmbeddedRAM23LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM23LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM23LL =
    crate::Reg<cse_pram__embedded_ram23ll::CSE_PRAM__EMBEDDEDRAM23LL_SPEC>;
#[doc = "CSE PRAM23LL register."]
pub mod cse_pram__embedded_ram23ll;
#[doc = "_EmbeddedRAM23LU register accessor: an alias for `Reg<_EMBEDDEDRAM23LU_SPEC>`"]
pub type _EMBEDDEDRAM23LU = crate::Reg<_embedded_ram23lu::_EMBEDDEDRAM23LU_SPEC>;
#[doc = "CSE PRAM23LU register."]
pub mod _embedded_ram23lu;
#[doc = "_EmbeddedRAM23HL register accessor: an alias for `Reg<_EMBEDDEDRAM23HL_SPEC>`"]
pub type _EMBEDDEDRAM23HL = crate::Reg<_embedded_ram23hl::_EMBEDDEDRAM23HL_SPEC>;
#[doc = "CSE PRAM23HL register."]
pub mod _embedded_ram23hl;
#[doc = "_EmbeddedRAM23HU register accessor: an alias for `Reg<_EMBEDDEDRAM23HU_SPEC>`"]
pub type _EMBEDDEDRAM23HU = crate::Reg<_embedded_ram23hu::_EMBEDDEDRAM23HU_SPEC>;
#[doc = "CSE PRAM23HU register."]
pub mod _embedded_ram23hu;
#[doc = "CSE_PRAM__EmbeddedRAM24 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM24_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM24 =
    crate::Reg<cse_pram__embedded_ram24::CSE_PRAM__EMBEDDEDRAM24_SPEC>;
#[doc = "CSE PRAM 24 Register"]
pub mod cse_pram__embedded_ram24;
#[doc = "CSE_PRAM__EmbeddedRAM24LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM24LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM24LL =
    crate::Reg<cse_pram__embedded_ram24ll::CSE_PRAM__EMBEDDEDRAM24LL_SPEC>;
#[doc = "CSE PRAM24LL register."]
pub mod cse_pram__embedded_ram24ll;
#[doc = "_EmbeddedRAM24LU register accessor: an alias for `Reg<_EMBEDDEDRAM24LU_SPEC>`"]
pub type _EMBEDDEDRAM24LU = crate::Reg<_embedded_ram24lu::_EMBEDDEDRAM24LU_SPEC>;
#[doc = "CSE PRAM24LU register."]
pub mod _embedded_ram24lu;
#[doc = "_EmbeddedRAM24HL register accessor: an alias for `Reg<_EMBEDDEDRAM24HL_SPEC>`"]
pub type _EMBEDDEDRAM24HL = crate::Reg<_embedded_ram24hl::_EMBEDDEDRAM24HL_SPEC>;
#[doc = "CSE PRAM24HL register."]
pub mod _embedded_ram24hl;
#[doc = "_EmbeddedRAM24HU register accessor: an alias for `Reg<_EMBEDDEDRAM24HU_SPEC>`"]
pub type _EMBEDDEDRAM24HU = crate::Reg<_embedded_ram24hu::_EMBEDDEDRAM24HU_SPEC>;
#[doc = "CSE PRAM24HU register."]
pub mod _embedded_ram24hu;
#[doc = "CSE_PRAM__EmbeddedRAM25 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM25_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM25 =
    crate::Reg<cse_pram__embedded_ram25::CSE_PRAM__EMBEDDEDRAM25_SPEC>;
#[doc = "CSE PRAM 25 Register"]
pub mod cse_pram__embedded_ram25;
#[doc = "CSE_PRAM__EmbeddedRAM25LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM25LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM25LL =
    crate::Reg<cse_pram__embedded_ram25ll::CSE_PRAM__EMBEDDEDRAM25LL_SPEC>;
#[doc = "CSE PRAM25LL register."]
pub mod cse_pram__embedded_ram25ll;
#[doc = "_EmbeddedRAM25LU register accessor: an alias for `Reg<_EMBEDDEDRAM25LU_SPEC>`"]
pub type _EMBEDDEDRAM25LU = crate::Reg<_embedded_ram25lu::_EMBEDDEDRAM25LU_SPEC>;
#[doc = "CSE PRAM25LU register."]
pub mod _embedded_ram25lu;
#[doc = "_EmbeddedRAM25HL register accessor: an alias for `Reg<_EMBEDDEDRAM25HL_SPEC>`"]
pub type _EMBEDDEDRAM25HL = crate::Reg<_embedded_ram25hl::_EMBEDDEDRAM25HL_SPEC>;
#[doc = "CSE PRAM25HL register."]
pub mod _embedded_ram25hl;
#[doc = "_EmbeddedRAM25HU register accessor: an alias for `Reg<_EMBEDDEDRAM25HU_SPEC>`"]
pub type _EMBEDDEDRAM25HU = crate::Reg<_embedded_ram25hu::_EMBEDDEDRAM25HU_SPEC>;
#[doc = "CSE PRAM25HU register."]
pub mod _embedded_ram25hu;
#[doc = "CSE_PRAM__EmbeddedRAM26 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM26_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM26 =
    crate::Reg<cse_pram__embedded_ram26::CSE_PRAM__EMBEDDEDRAM26_SPEC>;
#[doc = "CSE PRAM 26 Register"]
pub mod cse_pram__embedded_ram26;
#[doc = "CSE_PRAM__EmbeddedRAM26LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM26LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM26LL =
    crate::Reg<cse_pram__embedded_ram26ll::CSE_PRAM__EMBEDDEDRAM26LL_SPEC>;
#[doc = "CSE PRAM26LL register."]
pub mod cse_pram__embedded_ram26ll;
#[doc = "_EmbeddedRAM26LU register accessor: an alias for `Reg<_EMBEDDEDRAM26LU_SPEC>`"]
pub type _EMBEDDEDRAM26LU = crate::Reg<_embedded_ram26lu::_EMBEDDEDRAM26LU_SPEC>;
#[doc = "CSE PRAM26LU register."]
pub mod _embedded_ram26lu;
#[doc = "_EmbeddedRAM26HL register accessor: an alias for `Reg<_EMBEDDEDRAM26HL_SPEC>`"]
pub type _EMBEDDEDRAM26HL = crate::Reg<_embedded_ram26hl::_EMBEDDEDRAM26HL_SPEC>;
#[doc = "CSE PRAM26HL register."]
pub mod _embedded_ram26hl;
#[doc = "_EmbeddedRAM26HU register accessor: an alias for `Reg<_EMBEDDEDRAM26HU_SPEC>`"]
pub type _EMBEDDEDRAM26HU = crate::Reg<_embedded_ram26hu::_EMBEDDEDRAM26HU_SPEC>;
#[doc = "CSE PRAM26HU register."]
pub mod _embedded_ram26hu;
#[doc = "CSE_PRAM__EmbeddedRAM27 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM27_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM27 =
    crate::Reg<cse_pram__embedded_ram27::CSE_PRAM__EMBEDDEDRAM27_SPEC>;
#[doc = "CSE PRAM 27 Register"]
pub mod cse_pram__embedded_ram27;
#[doc = "CSE_PRAM__EmbeddedRAM27LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM27LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM27LL =
    crate::Reg<cse_pram__embedded_ram27ll::CSE_PRAM__EMBEDDEDRAM27LL_SPEC>;
#[doc = "CSE PRAM27LL register."]
pub mod cse_pram__embedded_ram27ll;
#[doc = "_EmbeddedRAM27LU register accessor: an alias for `Reg<_EMBEDDEDRAM27LU_SPEC>`"]
pub type _EMBEDDEDRAM27LU = crate::Reg<_embedded_ram27lu::_EMBEDDEDRAM27LU_SPEC>;
#[doc = "CSE PRAM27LU register."]
pub mod _embedded_ram27lu;
#[doc = "_EmbeddedRAM27HL register accessor: an alias for `Reg<_EMBEDDEDRAM27HL_SPEC>`"]
pub type _EMBEDDEDRAM27HL = crate::Reg<_embedded_ram27hl::_EMBEDDEDRAM27HL_SPEC>;
#[doc = "CSE PRAM27HL register."]
pub mod _embedded_ram27hl;
#[doc = "_EmbeddedRAM27HU register accessor: an alias for `Reg<_EMBEDDEDRAM27HU_SPEC>`"]
pub type _EMBEDDEDRAM27HU = crate::Reg<_embedded_ram27hu::_EMBEDDEDRAM27HU_SPEC>;
#[doc = "CSE PRAM27HU register."]
pub mod _embedded_ram27hu;
#[doc = "CSE_PRAM__EmbeddedRAM28 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM28_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM28 =
    crate::Reg<cse_pram__embedded_ram28::CSE_PRAM__EMBEDDEDRAM28_SPEC>;
#[doc = "CSE PRAM 28 Register"]
pub mod cse_pram__embedded_ram28;
#[doc = "CSE_PRAM__EmbeddedRAM28LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM28LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM28LL =
    crate::Reg<cse_pram__embedded_ram28ll::CSE_PRAM__EMBEDDEDRAM28LL_SPEC>;
#[doc = "CSE PRAM28LL register."]
pub mod cse_pram__embedded_ram28ll;
#[doc = "_EmbeddedRAM28LU register accessor: an alias for `Reg<_EMBEDDEDRAM28LU_SPEC>`"]
pub type _EMBEDDEDRAM28LU = crate::Reg<_embedded_ram28lu::_EMBEDDEDRAM28LU_SPEC>;
#[doc = "CSE PRAM28LU register."]
pub mod _embedded_ram28lu;
#[doc = "_EmbeddedRAM28HL register accessor: an alias for `Reg<_EMBEDDEDRAM28HL_SPEC>`"]
pub type _EMBEDDEDRAM28HL = crate::Reg<_embedded_ram28hl::_EMBEDDEDRAM28HL_SPEC>;
#[doc = "CSE PRAM28HL register."]
pub mod _embedded_ram28hl;
#[doc = "_EmbeddedRAM28HU register accessor: an alias for `Reg<_EMBEDDEDRAM28HU_SPEC>`"]
pub type _EMBEDDEDRAM28HU = crate::Reg<_embedded_ram28hu::_EMBEDDEDRAM28HU_SPEC>;
#[doc = "CSE PRAM28HU register."]
pub mod _embedded_ram28hu;
#[doc = "CSE_PRAM__EmbeddedRAM29 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM29_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM29 =
    crate::Reg<cse_pram__embedded_ram29::CSE_PRAM__EMBEDDEDRAM29_SPEC>;
#[doc = "CSE PRAM 29 Register"]
pub mod cse_pram__embedded_ram29;
#[doc = "CSE_PRAM__EmbeddedRAM29LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM29LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM29LL =
    crate::Reg<cse_pram__embedded_ram29ll::CSE_PRAM__EMBEDDEDRAM29LL_SPEC>;
#[doc = "CSE PRAM29LL register."]
pub mod cse_pram__embedded_ram29ll;
#[doc = "_EmbeddedRAM29LU register accessor: an alias for `Reg<_EMBEDDEDRAM29LU_SPEC>`"]
pub type _EMBEDDEDRAM29LU = crate::Reg<_embedded_ram29lu::_EMBEDDEDRAM29LU_SPEC>;
#[doc = "CSE PRAM29LU register."]
pub mod _embedded_ram29lu;
#[doc = "_EmbeddedRAM29HL register accessor: an alias for `Reg<_EMBEDDEDRAM29HL_SPEC>`"]
pub type _EMBEDDEDRAM29HL = crate::Reg<_embedded_ram29hl::_EMBEDDEDRAM29HL_SPEC>;
#[doc = "CSE PRAM29HL register."]
pub mod _embedded_ram29hl;
#[doc = "_EmbeddedRAM29HU register accessor: an alias for `Reg<_EMBEDDEDRAM29HU_SPEC>`"]
pub type _EMBEDDEDRAM29HU = crate::Reg<_embedded_ram29hu::_EMBEDDEDRAM29HU_SPEC>;
#[doc = "CSE PRAM29HU register."]
pub mod _embedded_ram29hu;
#[doc = "CSE_PRAM__EmbeddedRAM30 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM30_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM30 =
    crate::Reg<cse_pram__embedded_ram30::CSE_PRAM__EMBEDDEDRAM30_SPEC>;
#[doc = "CSE PRAM 30 Register"]
pub mod cse_pram__embedded_ram30;
#[doc = "CSE_PRAM__EmbeddedRAM30LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM30LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM30LL =
    crate::Reg<cse_pram__embedded_ram30ll::CSE_PRAM__EMBEDDEDRAM30LL_SPEC>;
#[doc = "CSE PRAM30LL register."]
pub mod cse_pram__embedded_ram30ll;
#[doc = "_EmbeddedRAM30LU register accessor: an alias for `Reg<_EMBEDDEDRAM30LU_SPEC>`"]
pub type _EMBEDDEDRAM30LU = crate::Reg<_embedded_ram30lu::_EMBEDDEDRAM30LU_SPEC>;
#[doc = "CSE PRAM30LU register."]
pub mod _embedded_ram30lu;
#[doc = "_EmbeddedRAM30HL register accessor: an alias for `Reg<_EMBEDDEDRAM30HL_SPEC>`"]
pub type _EMBEDDEDRAM30HL = crate::Reg<_embedded_ram30hl::_EMBEDDEDRAM30HL_SPEC>;
#[doc = "CSE PRAM30HL register."]
pub mod _embedded_ram30hl;
#[doc = "_EmbeddedRAM30HU register accessor: an alias for `Reg<_EMBEDDEDRAM30HU_SPEC>`"]
pub type _EMBEDDEDRAM30HU = crate::Reg<_embedded_ram30hu::_EMBEDDEDRAM30HU_SPEC>;
#[doc = "CSE PRAM30HU register."]
pub mod _embedded_ram30hu;
#[doc = "CSE_PRAM__EmbeddedRAM31 register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM31_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM31 =
    crate::Reg<cse_pram__embedded_ram31::CSE_PRAM__EMBEDDEDRAM31_SPEC>;
#[doc = "CSE PRAM 31 Register"]
pub mod cse_pram__embedded_ram31;
#[doc = "CSE_PRAM__EmbeddedRAM31LL register accessor: an alias for `Reg<CSE_PRAM__EMBEDDEDRAM31LL_SPEC>`"]
pub type CSE_PRAM__EMBEDDEDRAM31LL =
    crate::Reg<cse_pram__embedded_ram31ll::CSE_PRAM__EMBEDDEDRAM31LL_SPEC>;
#[doc = "CSE PRAM31LL register."]
pub mod cse_pram__embedded_ram31ll;
#[doc = "_EmbeddedRAM31LU register accessor: an alias for `Reg<_EMBEDDEDRAM31LU_SPEC>`"]
pub type _EMBEDDEDRAM31LU = crate::Reg<_embedded_ram31lu::_EMBEDDEDRAM31LU_SPEC>;
#[doc = "CSE PRAM31LU register."]
pub mod _embedded_ram31lu;
#[doc = "_EmbeddedRAM31HL register accessor: an alias for `Reg<_EMBEDDEDRAM31HL_SPEC>`"]
pub type _EMBEDDEDRAM31HL = crate::Reg<_embedded_ram31hl::_EMBEDDEDRAM31HL_SPEC>;
#[doc = "CSE PRAM31HL register."]
pub mod _embedded_ram31hl;
#[doc = "_EmbeddedRAM31HU register accessor: an alias for `Reg<_EMBEDDEDRAM31HU_SPEC>`"]
pub type _EMBEDDEDRAM31HU = crate::Reg<_embedded_ram31hu::_EMBEDDEDRAM31HU_SPEC>;
#[doc = "CSE PRAM31HU register."]
pub mod _embedded_ram31hu;
