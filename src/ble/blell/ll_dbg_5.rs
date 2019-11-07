#[doc = "Reader of register LL_DBG_5"]
pub type R = crate::R<u32, super::LL_DBG_5>;
#[doc = "Reader of field `INIT_FSM_STATE`"]
pub type INIT_FSM_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCAN_FSM_STATE`"]
pub type SCAN_FSM_STATE_R = crate::R<u8, u8>;
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
