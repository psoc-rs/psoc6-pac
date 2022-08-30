#[doc = "Register `LL_DBG_10` reader"]
pub struct R(crate::R<LL_DBG_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_DBG_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_DBG_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_DBG_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RF_CHANNEL_NUM` reader - Active channel number"]
pub type RF_CHANNEL_NUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Active channel number"]
    #[inline(always)]
    pub fn rf_channel_num(&self) -> RF_CHANNEL_NUM_R {
        RF_CHANNEL_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "LL debug register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_10](index.html) module"]
pub struct LL_DBG_10_SPEC;
impl crate::RegisterSpec for LL_DBG_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_dbg_10::R](R) reader structure"]
impl crate::Readable for LL_DBG_10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_DBG_10 to value 0"]
impl crate::Resettable for LL_DBG_10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
