#[doc = "Register `BT_CLOCK_CAPT` reader"]
pub struct R(crate::R<BT_CLOCK_CAPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_CLOCK_CAPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_CLOCK_CAPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_CLOCK_CAPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BT_CLOCK` reader - This field captures the LF BT clock captured on an LL DSM exit. This register is valid only when MT_STATUS.LL_CLK_STATE is set. This value may be used to manage the low power entry."]
pub type BT_CLOCK_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field captures the LF BT clock captured on an LL DSM exit. This register is valid only when MT_STATUS.LL_CLK_STATE is set. This value may be used to manage the low power entry."]
    #[inline(always)]
    pub fn bt_clock(&self) -> BT_CLOCK_R {
        BT_CLOCK_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "BT clock captured on an LL DSM exit\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_clock_capt](index.html) module"]
pub struct BT_CLOCK_CAPT_SPEC;
impl crate::RegisterSpec for BT_CLOCK_CAPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_clock_capt::R](R) reader structure"]
impl crate::Readable for BT_CLOCK_CAPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BT_CLOCK_CAPT to value 0"]
impl crate::Resettable for BT_CLOCK_CAPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
