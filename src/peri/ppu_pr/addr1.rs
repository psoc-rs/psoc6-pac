#[doc = "Reader of register ADDR1"]
pub type R = crate::R<u32, super::ADDR1>;
#[doc = "Reader of field `SUBREGION_DISABLE`"]
pub type SUBREGION_DISABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADDR24`"]
pub type ADDR24_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - See corresponding field for PPU structure with programmable address. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
