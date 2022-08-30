#[doc = "Register `LL_DBG_9` reader"]
pub struct R(crate::R<LL_DBG_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_DBG_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_DBG_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_DBG_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WINDOW_WIDEN` reader - Window Widening value in us. The reset value of this register is 0x0000. After reset de-assertion, at the first clock cycle, the value 0x0010 is assigned to the register."]
pub type WINDOW_WIDEN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Widening value in us. The reset value of this register is 0x0000. After reset de-assertion, at the first clock cycle, the value 0x0010 is assigned to the register."]
    #[inline(always)]
    pub fn window_widen(&self) -> WINDOW_WIDEN_R {
        WINDOW_WIDEN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LL debug register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_9](index.html) module"]
pub struct LL_DBG_9_SPEC;
impl crate::RegisterSpec for LL_DBG_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_dbg_9::R](R) reader structure"]
impl crate::Readable for LL_DBG_9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_DBG_9 to value 0x10"]
impl crate::Resettable for LL_DBG_9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
