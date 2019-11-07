#[doc = "Reader of register EARLY_INTR_NI"]
pub type R = crate::R<u32, super::EARLY_INTR_NI>;
#[doc = "Reader of field `EARLY_INTR_NI`"]
pub type EARLY_INTR_NI_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Connection Next instant when the early interrupt is triggered"]
    #[inline(always)]
    pub fn early_intr_ni(&self) -> EARLY_INTR_NI_R {
        EARLY_INTR_NI_R::new((self.bits & 0xffff) as u16)
    }
}
