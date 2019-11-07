#[doc = "Reader of register ACT_SRC"]
pub type R = crate::R<u32, super::ACT_SRC>;
#[doc = "Reader of field `SRC_ADDR`"]
pub type SRC_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of source location."]
    #[inline(always)]
    pub fn src_addr(&self) -> SRC_ADDR_R {
        SRC_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
