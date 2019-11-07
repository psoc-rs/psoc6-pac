#[doc = "Reader of register PWR_CTRL_SM_ST"]
pub type R = crate::R<u32, super::PWR_CTRL_SM_ST>;
#[doc = "Reader of field `PWR_CTRL_SM_CURR_STATE`"]
pub type PWR_CTRL_SM_CURR_STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - This register reflects the current state of the LL Power Control FSM 4'h0 - IDLE 4'h1 - SLEEP 4'h2 - DEEP_SLEEP 4'h4 - WAIT_OSC_STABLE 4'h5 - INTR_GEN 4'h6 - ACTIVE 4'h7 - REQ_RF_OFF"]
    #[inline(always)]
    pub fn pwr_ctrl_sm_curr_state(&self) -> PWR_CTRL_SM_CURR_STATE_R {
        PWR_CTRL_SM_CURR_STATE_R::new((self.bits & 0x0f) as u8)
    }
}
