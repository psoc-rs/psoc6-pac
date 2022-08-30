#[doc = "Register `PWR_LVD_STATUS` reader"]
pub struct R(crate::R<PWR_LVD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_LVD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_LVD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_LVD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HVLVD1_OK` reader - HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
pub type HVLVD1_OK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
    #[inline(always)]
    pub fn hvlvd1_ok(&self) -> HVLVD1_OK_R {
        HVLVD1_OK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Low Voltage Detector (LVD) Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_lvd_status](index.html) module"]
pub struct PWR_LVD_STATUS_SPEC;
impl crate::RegisterSpec for PWR_LVD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_lvd_status::R](R) reader structure"]
impl crate::Readable for PWR_LVD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_LVD_STATUS to value 0"]
impl crate::Resettable for PWR_LVD_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
