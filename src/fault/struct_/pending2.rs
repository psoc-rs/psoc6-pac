#[doc = "Reader of register PENDING2"]
pub type R = crate::R<u32, super::PENDING2>;
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0 - 31: TBD."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
