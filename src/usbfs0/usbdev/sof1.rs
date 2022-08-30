#[doc = "Register `SOF1` reader"]
pub struct R(crate::R<SOF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_NUMBER_MSB` reader - It has the upper 3 bits \\[10:8\\]
of the SOF frame number."]
pub type FRAME_NUMBER_MSB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - It has the upper 3 bits \\[10:8\\]
of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number_msb(&self) -> FRAME_NUMBER_MSB_R {
        FRAME_NUMBER_MSB_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Start Of Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof1](index.html) module"]
pub struct SOF1_SPEC;
impl crate::RegisterSpec for SOF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sof1::R](R) reader structure"]
impl crate::Readable for SOF1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SOF1 to value 0"]
impl crate::Resettable for SOF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
