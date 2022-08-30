#[doc = "Register `SOF16` reader"]
pub struct R(crate::R<SOF16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOF16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOF16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOF16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_NUMBER16` reader - The frame number (11b)"]
pub type FRAME_NUMBER16_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - The frame number (11b)"]
    #[inline(always)]
    pub fn frame_number16(&self) -> FRAME_NUMBER16_R {
        FRAME_NUMBER16_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Start Of Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sof16](index.html) module"]
pub struct SOF16_SPEC;
impl crate::RegisterSpec for SOF16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sof16::R](R) reader structure"]
impl crate::Readable for SOF16_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SOF16 to value 0"]
impl crate::Resettable for SOF16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
