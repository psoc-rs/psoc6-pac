#[doc = "Register `MONITOR_STATUS` reader"]
pub struct R(crate::R<MONITOR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONITOR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONITOR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONITOR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POS_PUMP_VLO` reader - POS pump VLO"]
pub type POS_PUMP_VLO_R = crate::BitReader<bool>;
#[doc = "Field `NEG_PUMP_VHI` reader - NEG pump VHI"]
pub type NEG_PUMP_VHI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - POS pump VLO"]
    #[inline(always)]
    pub fn pos_pump_vlo(&self) -> POS_PUMP_VLO_R {
        POS_PUMP_VLO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NEG pump VHI"]
    #[inline(always)]
    pub fn neg_pump_vhi(&self) -> NEG_PUMP_VHI_R {
        NEG_PUMP_VHI_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Monitor Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_status](index.html) module"]
pub struct MONITOR_STATUS_SPEC;
impl crate::RegisterSpec for MONITOR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monitor_status::R](R) reader structure"]
impl crate::Readable for MONITOR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MONITOR_STATUS to value 0x04"]
impl crate::Resettable for MONITOR_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
