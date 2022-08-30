#[doc = "Register `CH_STATUS` reader"]
pub struct R(crate::R<CH_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_CAUSE` reader - Specifies the source of the interrupt cause: '0': NO_INTR '1': COMPLETION '2': SRC_BUS_ERROR '3': DST_BUS_ERROR '4': SRC_MISAL '5': DST_MISAL '6': CURR_PTR_NULL '7': ACTIVE_CH_DISABLED '8': DESCR_BUS_ERROR '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
pub type INTR_CAUSE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Specifies the source of the interrupt cause: '0': NO_INTR '1': COMPLETION '2': SRC_BUS_ERROR '3': DST_BUS_ERROR '4': SRC_MISAL '5': DST_MISAL '6': CURR_PTR_NULL '7': ACTIVE_CH_DISABLED '8': DESCR_BUS_ERROR '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub fn intr_cause(&self) -> INTR_CAUSE_R {
        INTR_CAUSE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Channel status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_status](index.html) module"]
pub struct CH_STATUS_SPEC;
impl crate::RegisterSpec for CH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_status::R](R) reader structure"]
impl crate::Readable for CH_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH_STATUS to value 0"]
impl crate::Resettable for CH_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
