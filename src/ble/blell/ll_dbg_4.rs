#[doc = "Register `LL_DBG_4` reader"]
pub struct R(crate::R<LL_DBG_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_DBG_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_DBG_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_DBG_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONNECTION_FSM_STATE` reader - Connection FSM state"]
pub type CONNECTION_FSM_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVE_LATENCY_FSM_STATE` reader - Slave Latency FSM state"]
pub type SLAVE_LATENCY_FSM_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADVERTISER_FSM_STATE` reader - Advertiser FSM state"]
pub type ADVERTISER_FSM_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Connection FSM state"]
    #[inline(always)]
    pub fn connection_fsm_state(&self) -> CONNECTION_FSM_STATE_R {
        CONNECTION_FSM_STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Slave Latency FSM state"]
    #[inline(always)]
    pub fn slave_latency_fsm_state(&self) -> SLAVE_LATENCY_FSM_STATE_R {
        SLAVE_LATENCY_FSM_STATE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:10 - Advertiser FSM state"]
    #[inline(always)]
    pub fn advertiser_fsm_state(&self) -> ADVERTISER_FSM_STATE_R {
        ADVERTISER_FSM_STATE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
#[doc = "LL debug register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_4](index.html) module"]
pub struct LL_DBG_4_SPEC;
impl crate::RegisterSpec for LL_DBG_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_dbg_4::R](R) reader structure"]
impl crate::Readable for LL_DBG_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_DBG_4 to value 0"]
impl crate::Resettable for LL_DBG_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
