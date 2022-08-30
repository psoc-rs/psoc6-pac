#[doc = "Register `OSCLK_DR16` reader"]
pub struct R(crate::R<OSCLK_DR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCLK_DR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCLK_DR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCLK_DR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDER16` reader - These bits return the oscillator locking circuits adder output."]
pub type ADDER16_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - These bits return the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder16(&self) -> ADDER16_R {
        ADDER16_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "Oscillator lock data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osclk_dr16](index.html) module"]
pub struct OSCLK_DR16_SPEC;
impl crate::RegisterSpec for OSCLK_DR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osclk_dr16::R](R) reader structure"]
impl crate::Readable for OSCLK_DR16_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCLK_DR16 to value 0"]
impl crate::Resettable for OSCLK_DR16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
