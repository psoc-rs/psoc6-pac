#[doc = "Register `SOF0` reader"]
pub struct R(crate::R<SOF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_NUMBER` reader - It has the lower 8 bits \\[7:0\\]
of the SOF frame number."]
pub type FRAME_NUMBER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - It has the lower 8 bits \\[7:0\\]
of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number(&self) -> FRAME_NUMBER_R {
        FRAME_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Start Of Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof0](index.html) module"]
pub struct SOF0_SPEC;
impl crate::RegisterSpec for SOF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sof0::R](R) reader structure"]
impl crate::Readable for SOF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SOF0 to value 0"]
impl crate::Resettable for SOF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
