#[doc = "Reader of register ACT_DESCR_NEXT_PTR"]
pub type R = crate::R<u32, super::ACT_DESCR_NEXT_PTR>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 2:31 - Copy of DESCR_NEXT_PTR of the currently active descriptor."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
