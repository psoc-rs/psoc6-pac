#[doc = "Reader of register LL_DBG_4"]
pub type R = crate::R<u32, super::LL_DBG_4>;
#[doc = "Reader of field `CONNECTION_FSM_STATE`"]
pub type CONNECTION_FSM_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SLAVE_LATENCY_FSM_STATE`"]
pub type SLAVE_LATENCY_FSM_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADVERTISER_FSM_STATE`"]
pub type ADVERTISER_FSM_STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Connection FSM state"]
    #[inline(always)]
    pub fn connection_fsm_state(&self) -> CONNECTION_FSM_STATE_R {
        CONNECTION_FSM_STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Slave Latency FSM state"]
    #[inline(always)]
    pub fn slave_latency_fsm_state(&self) -> SLAVE_LATENCY_FSM_STATE_R {
        SLAVE_LATENCY_FSM_STATE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:10 - Advertiser FSM state"]
    #[inline(always)]
    pub fn advertiser_fsm_state(&self) -> ADVERTISER_FSM_STATE_R {
        ADVERTISER_FSM_STATE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
