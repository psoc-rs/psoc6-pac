#[doc = "Register `MMMS_SLAVE_CREATE_BT_CAPT` reader"]
pub struct R(crate::R<MMMS_SLAVE_CREATE_BT_CAPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_SLAVE_CREATE_BT_CAPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_SLAVE_CREATE_BT_CAPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_SLAVE_CREATE_BT_CAPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `US_CAPT` reader - This register captures the BT_SLOT when slave connection is created, granularity is 625us"]
pub type US_CAPT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - This register captures the BT_SLOT when slave connection is created, granularity is 625us"]
    #[inline(always)]
    pub fn us_capt(&self) -> US_CAPT_R {
        US_CAPT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "BT slot capture for slave connection creation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_slave_create_bt_capt](index.html) module"]
pub struct MMMS_SLAVE_CREATE_BT_CAPT_SPEC;
impl crate::RegisterSpec for MMMS_SLAVE_CREATE_BT_CAPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_slave_create_bt_capt::R](R) reader structure"]
impl crate::Readable for MMMS_SLAVE_CREATE_BT_CAPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMMS_SLAVE_CREATE_BT_CAPT to value 0"]
impl crate::Resettable for MMMS_SLAVE_CREATE_BT_CAPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
