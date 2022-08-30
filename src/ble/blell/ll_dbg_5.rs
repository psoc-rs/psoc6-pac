#[doc = "Register `LL_DBG_5` reader"]
pub struct R(crate::R<LL_DBG_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_DBG_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_DBG_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_DBG_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INIT_FSM_STATE` reader - Initiator FSM state"]
pub type INIT_FSM_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCAN_FSM_STATE` reader - Scanner FSM state"]
pub type SCAN_FSM_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Initiator FSM state"]
    #[inline(always)]
    pub fn init_fsm_state(&self) -> INIT_FSM_STATE_R {
        INIT_FSM_STATE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Scanner FSM state"]
    #[inline(always)]
    pub fn scan_fsm_state(&self) -> SCAN_FSM_STATE_R {
        SCAN_FSM_STATE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
#[doc = "LL debug register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_5](index.html) module"]
pub struct LL_DBG_5_SPEC;
impl crate::RegisterSpec for LL_DBG_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_dbg_5::R](R) reader structure"]
impl crate::Readable for LL_DBG_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_DBG_5 to value 0"]
impl crate::Resettable for LL_DBG_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
