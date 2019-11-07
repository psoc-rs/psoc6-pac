#[doc = "Reader of register COMP_STAT"]
pub type R = crate::R<u32, super::COMP_STAT>;
#[doc = "Reader of field `OA0_COMP`"]
pub type OA0_COMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `OA1_COMP`"]
pub type OA1_COMP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Opamp0 current comparator status"]
    #[inline(always)]
    pub fn oa0_comp(&self) -> OA0_COMP_R {
        OA0_COMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Opamp1 current comparator status"]
    #[inline(always)]
    pub fn oa1_comp(&self) -> OA1_COMP_R {
        OA1_COMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
