#[doc = "Register `BT_SLOT_CAPT_STATUS` reader"]
pub struct R(crate::R<BT_SLOT_CAPT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_SLOT_CAPT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_SLOT_CAPT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_SLOT_CAPT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BT_SLOT` reader - During slave connection event, HW updates this register with the captured BT_SLOT at anchor point, granularity is 625us"]
pub type BT_SLOT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - During slave connection event, HW updates this register with the captured BT_SLOT at anchor point, granularity is 625us"]
    #[inline(always)]
    pub fn bt_slot(&self) -> BT_SLOT_R {
        BT_SLOT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "BT Slot Captured Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_slot_capt_status](index.html) module"]
pub struct BT_SLOT_CAPT_STATUS_SPEC;
impl crate::RegisterSpec for BT_SLOT_CAPT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_slot_capt_status::R](R) reader structure"]
impl crate::Readable for BT_SLOT_CAPT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BT_SLOT_CAPT_STATUS to value 0"]
impl crate::Resettable for BT_SLOT_CAPT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
