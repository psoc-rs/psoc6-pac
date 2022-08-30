#[doc = "Register `INIT_WINDOW_NI_ANCHOR_PT` reader"]
pub struct R(crate::R<INIT_WINDOW_NI_ANCHOR_PT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_WINDOW_NI_ANCHOR_PT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_WINDOW_NI_ANCHOR_PT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_WINDOW_NI_ANCHOR_PT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INIT_INT_OFF_CAPT` reader - Initiator interval offset captured at conn request. The value indicates the master connection anchor point. This value is in 625us slots"]
pub type INIT_INT_OFF_CAPT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Initiator interval offset captured at conn request. The value indicates the master connection anchor point. This value is in 625us slots"]
    #[inline(always)]
    pub fn init_int_off_capt(&self) -> INIT_INT_OFF_CAPT_R {
        INIT_INT_OFF_CAPT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Initiator Window NI anchor point captured at conn request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_window_ni_anchor_pt](index.html) module"]
pub struct INIT_WINDOW_NI_ANCHOR_PT_SPEC;
impl crate::RegisterSpec for INIT_WINDOW_NI_ANCHOR_PT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_window_ni_anchor_pt::R](R) reader structure"]
impl crate::Readable for INIT_WINDOW_NI_ANCHOR_PT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INIT_WINDOW_NI_ANCHOR_PT to value 0"]
impl crate::Resettable for INIT_WINDOW_NI_ANCHOR_PT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
