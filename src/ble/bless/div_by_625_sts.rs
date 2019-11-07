#[doc = "Reader of register DIV_BY_625_STS"]
pub type R = crate::R<u32, super::DIV_BY_625_STS>;
#[doc = "Reader of field `QUOTIENT`"]
pub type QUOTIENT_R = crate::R<u8, u8>;
#[doc = "Reader of field `REMAINDER`"]
pub type REMAINDER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Quotient value from the divider. Available 1 cycle after dividend is programmed."]
    #[inline(always)]
    pub fn quotient(&self) -> QUOTIENT_R {
        QUOTIENT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:17 - Remainder value from the divider. Available 1 cycle after dividend is programmed."]
    #[inline(always)]
    pub fn remainder(&self) -> REMAINDER_R {
        REMAINDER_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
