#[doc = "Reader of register ACT_DST"]
pub type R = crate::R<u32, super::ACT_DST>;
#[doc = "Reader of field `DST_ADDR`"]
pub type DST_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of destination location."]
    #[inline(always)]
    pub fn dst_addr(&self) -> DST_ADDR_R {
        DST_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
