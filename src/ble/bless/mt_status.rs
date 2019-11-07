#[doc = "Reader of register MT_STATUS"]
pub type R = crate::R<u32, super::MT_STATUS>;
#[doc = "Reader of field `BLESS_STATE`"]
pub type BLESS_STATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MT_CURR_STATE`"]
pub type MT_CURR_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `HVLDO_STARTUP_CURR_STATE`"]
pub type HVLDO_STARTUP_CURR_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `LL_CLK_STATE`"]
pub type LL_CLK_STATE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 1'b0 - BLESS in DPSLP state 1'b1 - BLESS in ACTIVE state"]
    #[inline(always)]
    pub fn bless_state(&self) -> BLESS_STATE_R {
        BLESS_STATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - This register reflects the current state of the MT FSM 4'h0 - IDLE 4'h1 - BLERD_DEEPSLEEP 4'h2 - HVLDO_STARTUP 4'h3 - WAIT_CLK 4'h4 - BLERD_IDLE 4'h5 - SWITCH_EN 4'h6 - ACTIVE 4'h7 - ISOLATE 4'h8 - WAIT_IDLE 4'h9 - XTAL_DISABLE 4'hA - HVLDO_DISABLE"]
    #[inline(always)]
    pub fn mt_curr_state(&self) -> MT_CURR_STATE_R {
        MT_CURR_STATE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - This register reflects the current state of the HVLDO Startup FSM 3'h0 - HVLDO_OFF 3'h1 - HVLDO_WAIT 3'h2 - HVLDO_SAMPLE 3'h3 - HVLDO_ENABLED 3'h4 - HVLDO_SET_BYPASS"]
    #[inline(always)]
    pub fn hvldo_startup_curr_state(&self) -> HVLDO_STARTUP_CURR_STATE_R {
        HVLDO_STARTUP_CURR_STATE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - This bit indicates when the Link Layer registers are accessible upon a DSM exit. This bit should not be used after a DSM entry command has been issued. 1'b0 - Link Layer clock is not available 1'b1 - Link Layer clock is active"]
    #[inline(always)]
    pub fn ll_clk_state(&self) -> LL_CLK_STATE_R {
        LL_CLK_STATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
