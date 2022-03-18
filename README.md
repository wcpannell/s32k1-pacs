# S32K1-PACs

This repository generates the Peripheral Access Crates (PACs) for the S32K1 series microcontrollers using [svd2rust](https://github.com/rust-embedded/svd2rust). The SVD files are pulled from the [CMSIS-PACK](https://developer.arm.com/tools-and-software/embedded/cmsis/cmsis-packs) listings. 

Each PAC supports its sub-family of microcontrollers as defined by the SVD files:
  * S32K116
  * S32K118
  * S32K142
  * S32K144
  * S32K146
  * S32K148
  * S32K142W
  * S32K144W

## Examples

Each subfamily has its own examples. These examples may also double as tests for that subfamily's PAC.

## Memory.x

An example memory.x is included for each microcontroller. These *should* be relatively correct based on information provided by the S32k1xx reference manual, but testing is limited by hardware cost/availability and time.

## Affiliation

The author(s) of this project are unaffiliated with NXP, but may (pretty-please?) receive support from NXP.

## Licensing

The SVD files are included here as a convenience and bear their own license separate from the PACs (TLDR: may only be used for NXP hardware, but I'm not a lawyer). See the opening text of each SVD for more detail. The files are included in good faith, but if this is found to be inconsistent with the license they will be removed upon request.

Excluding the SVD files and any licenses that may apply to code generated from them, all files in this project are licensed under the MIT License.
