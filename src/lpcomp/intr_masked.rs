#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `COMP0_MASKED`"]
pub type COMP0_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP1_MASKED`"]
pub type COMP1_MASKED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp0_masked(&self) -> COMP0_MASKED_R {
        COMP0_MASKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp1_masked(&self) -> COMP1_MASKED_R {
        COMP1_MASKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
