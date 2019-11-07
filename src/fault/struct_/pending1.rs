#[doc = "Reader of register PENDING1"]
pub type R = crate::R<u32, super::PENDING1>;
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0: Peripheral group 0 PPU. Bit 1: Peripheral group 1 PPU. Bit 2: Peripheral group 2 PPU. Bit 3: Peripheral group 3 PPU. Bit 4: Peripheral group 4 PPU. Bit 5: Peripheral group 5 PPU. Bit 6: Peripheral group 6 PPU. Bit 7: Peripheral group 7 PPU. ... Bit 15: Peripheral group 15 PPU. Bit 18: Flash controller, main interface, bus error."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
