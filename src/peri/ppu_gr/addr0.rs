#[doc = "Reader of register ADDR0"]
pub type R = crate::R<u32, super::ADDR0>;
#[doc = "Reader of field `SUBREGION_DISABLE`"]
pub type SUBREGION_DISABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADDR24`"]
pub type ADDR24_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': address of protected region. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
