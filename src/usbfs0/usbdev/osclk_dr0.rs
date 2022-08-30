#[doc = "Register `OSCLK_DR0` reader"]
pub struct R(crate::R<OSCLK_DR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCLK_DR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCLK_DR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCLK_DR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDER` reader - These bits return the lower 8 bits of the oscillator locking circuits adder output."]
pub type ADDER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits return the lower 8 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder(&self) -> ADDER_R {
        ADDER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Oscillator lock data register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osclk_dr0](index.html) module"]
pub struct OSCLK_DR0_SPEC;
impl crate::RegisterSpec for OSCLK_DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osclk_dr0::R](R) reader structure"]
impl crate::Readable for OSCLK_DR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCLK_DR0 to value 0"]
impl crate::Resettable for OSCLK_DR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
