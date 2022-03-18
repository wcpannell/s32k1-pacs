# S32K1-PACs

This repository generates the Peripheral Access Crates (PACs) for the S32K1 series microcontrollers using [svd2rust](https://github.com/rust-embedded/svd2rust). The SVD files are pulled from the [CMSIS-PACK](https://developer.arm.com/tools-and-software/embedded/cmsis/cmsis-packs) listings. 

Each PAC supports its sub-family of microcontrollers as defined by the SVD files:

  * S32K116 [![docs.rs](https://docs.rs/s32k116-pac/badge.svg)](https://docs.rs/s32k116) [![crates.io](https://img.shields.io/crates/v/s32k116-pac.svg)](https://crates.io/crates/s32k116) [![crates.io](https://img.shields.io/crates/d/s32k116-pac.svg)](https://crates.io/crates/s32k116)  
  * S32K118 [![docs.rs](https://docs.rs/s32k118-pac/badge.svg)](https://docs.rs/s32k118) [![crates.io](https://img.shields.io/crates/v/s32k118-pac.svg)](https://crates.io/crates/s32k118) [![crates.io](https://img.shields.io/crates/d/s32k118-pac.svg)](https://crates.io/crates/s32k118)

  * S32K142 [![docs.rs](https://docs.rs/s32k142-pac/badge.svg)](https://docs.rs/s32k142) [![crates.io](https://img.shields.io/crates/v/s32k142-pac.svg)](https://crates.io/crates/s32k142) [![crates.io](https://img.shields.io/crates/d/s32k142-pac.svg)](https://crates.io/crates/s32k142)

  * S32K144 [![docs.rs](https://docs.rs/s32k144-pac/badge.svg)](https://docs.rs/s32k144) [![crates.io](https://img.shields.io/crates/v/s32k144-pac.svg)](https://crates.io/crates/s32k144) [![crates.io](https://img.shields.io/crates/d/s32k144-pac.svg)](https://crates.io/crates/s32k144)

  * S32K146 [![docs.rs](https://docs.rs/s32k146-pac/badge.svg)](https://docs.rs/s32k146) [![crates.io](https://img.shields.io/crates/v/s32k146-pac.svg)](https://crates.io/crates/s32k146) [![crates.io](https://img.shields.io/crates/d/s32k146-pac.svg)](https://crates.io/crates/s32k146)

  * S32K148 [![docs.rs](https://docs.rs/s32k148-pac/badge.svg)](https://docs.rs/s32k148) [![crates.io](https://img.shields.io/crates/v/s32k148-pac.svg)](https://crates.io/crates/s32k148) [![crates.io](https://img.shields.io/crates/d/s32k148-pac.svg)](https://crates.io/crates/s32k148)

  * S32K142W [![docs.rs](https://docs.rs/s32k142w-pac/badge.svg)](https://docs.rs/s32k142w) [![crates.io](https://img.shields.io/crates/v/s32k142w-pac.svg)](https://crates.io/crates/s32k142w) [![crates.io](https://img.shields.io/crates/d/s32k142w-pac.svg)](https://crates.io/crates/s32k142w)

* S32K144W [![docs.rs](https://docs.rs/s32k144w-pac/badge.svg)](https://docs.rs/s32k144w) [![crates.io](https://img.shields.io/crates/v/s32k144w-pac.svg)](https://crates.io/crates/s32k144w) [![crates.io](https://img.shields.io/crates/d/s32k144w-pac.svg)](https://crates.io/crates/s32k144w)

## Examples

Each subfamily has its own examples. These examples may also double as tests for that subfamily's PAC.

## Memory.x

An example memory.x is included for each microcontroller. These *should* be relatively correct based on information provided by the S32k1xx reference manual, but testing is limited by hardware cost/availability and time.

## Affiliation

The author(s) of this project are unaffiliated with NXP, but may (pretty-please?) receive support from NXP.

## Licensing

The SVD files are included here as a convenience and bear their own license separate from the PACs (TLDR: may only be used for NXP hardware, but I'm not a lawyer). See the opening text of each SVD for more detail. The files are included in good faith, but if this is found to be inconsistent with the license they will be removed upon request.

Excluding the SVD files and any licenses that may apply to code generated from them, all files in this project are licensed under the MIT License.
