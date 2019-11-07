#[doc = "Reader of register BIST_DATA_ACT[%s]"]
pub type R = crate::R<u32, super::BIST_DATA_ACT>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field specified the actual Flash data output that caused the BIST failure."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
