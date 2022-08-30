#[doc = "Register `DIV_BY_625_STS` reader"]
pub struct R(crate::R<DIV_BY_625_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_BY_625_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_BY_625_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_BY_625_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUOTIENT` reader - Quotient value from the divider. Available 1 cycle after dividend is programmed."]
pub type QUOTIENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REMAINDER` reader - Remainder value from the divider. Available 1 cycle after dividend is programmed."]
pub type REMAINDER_R = crate::FieldReader<u16, u16>;
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
#[doc = "Output of divide by 625 divider\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_by_625_sts](index.html) module"]
pub struct DIV_BY_625_STS_SPEC;
impl crate::RegisterSpec for DIV_BY_625_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_by_625_sts::R](R) reader structure"]
impl crate::Readable for DIV_BY_625_STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIV_BY_625_STS to value 0x0100"]
impl crate::Resettable for DIV_BY_625_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
