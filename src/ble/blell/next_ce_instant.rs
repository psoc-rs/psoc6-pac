#[doc = "Register `NEXT_CE_INSTANT` reader"]
pub struct R(crate::R<NEXT_CE_INSTANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEXT_CE_INSTANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEXT_CE_INSTANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEXT_CE_INSTANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NEXT_CE_INSTANT` reader - 16-bit internal reference clock value at which the next connection event will occur on a connection. The connection index register must be programmed with index of the connection, before reading the register. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0xFFFF."]
pub type NEXT_CE_INSTANT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit internal reference clock value at which the next connection event will occur on a connection. The connection index register must be programmed with index of the connection, before reading the register. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0xFFFF."]
    #[inline(always)]
    pub fn next_ce_instant(&self) -> NEXT_CE_INSTANT_R {
        NEXT_CE_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Next connection event instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_ce_instant](index.html) module"]
pub struct NEXT_CE_INSTANT_SPEC;
impl crate::RegisterSpec for NEXT_CE_INSTANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [next_ce_instant::R](R) reader structure"]
impl crate::Readable for NEXT_CE_INSTANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NEXT_CE_INSTANT to value 0xffff"]
impl crate::Resettable for NEXT_CE_INSTANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
