#[doc = "Register `INIT_WINDOW_OFFSET` reader"]
pub struct R(crate::R<INIT_WINDOW_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_WINDOW_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_WINDOW_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_WINDOW_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INIT_WINDOW_NI` reader - Initiator Window offset captured at conn request. This value is in 1.25ms slots"]
pub type INIT_WINDOW_NI_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Initiator Window offset captured at conn request. This value is in 1.25ms slots"]
    #[inline(always)]
    pub fn init_window_ni(&self) -> INIT_WINDOW_NI_R {
        INIT_WINDOW_NI_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Initiator Window offset captured at conn request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_window_offset](index.html) module"]
pub struct INIT_WINDOW_OFFSET_SPEC;
impl crate::RegisterSpec for INIT_WINDOW_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_window_offset::R](R) reader structure"]
impl crate::Readable for INIT_WINDOW_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INIT_WINDOW_OFFSET to value 0"]
impl crate::Resettable for INIT_WINDOW_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
