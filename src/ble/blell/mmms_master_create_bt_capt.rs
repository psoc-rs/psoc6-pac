#[doc = "Register `MMMS_MASTER_CREATE_BT_CAPT` reader"]
pub struct R(crate::R<MMMS_MASTER_CREATE_BT_CAPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_MASTER_CREATE_BT_CAPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_MASTER_CREATE_BT_CAPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_MASTER_CREATE_BT_CAPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BT_SLOT` reader - This register captures the BT_SLOT when master connection is created, granularity is 625us"]
pub type BT_SLOT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register captures the BT_SLOT when master connection is created, granularity is 625us"]
    #[inline(always)]
    pub fn bt_slot(&self) -> BT_SLOT_R {
        BT_SLOT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "BT slot capture for master connection creation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_master_create_bt_capt](index.html) module"]
pub struct MMMS_MASTER_CREATE_BT_CAPT_SPEC;
impl crate::RegisterSpec for MMMS_MASTER_CREATE_BT_CAPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_master_create_bt_capt::R](R) reader structure"]
impl crate::Readable for MMMS_MASTER_CREATE_BT_CAPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMMS_MASTER_CREATE_BT_CAPT to value 0"]
impl crate::Resettable for MMMS_MASTER_CREATE_BT_CAPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
