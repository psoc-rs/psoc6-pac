#[doc = "Reader of register DP_STATUS"]
pub type R = crate::R<u32, super::DP_STATUS>;
#[doc = "Reader of field `SWJ_CONNECTED`"]
pub type SWJ_CONNECTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWJ_DEBUG_EN`"]
pub type SWJ_DEBUG_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWJ_JTAG_SEL`"]
pub type SWJ_JTAG_SEL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Specifies if the SWJ debug port is connected; i.e. debug host interface is active: '0': Not connected/not active. '1': Connected/active."]
    #[inline(always)]
    pub fn swj_connected(&self) -> SWJ_CONNECTED_R {
        SWJ_CONNECTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Specifies if SWJ debug is enabled, i.e. CDBGPWRUPACK is '1' and thus debug clocks are on: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn swj_debug_en(&self) -> SWJ_DEBUG_EN_R {
        SWJ_DEBUG_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Specifies if the JTAG or SWD interface is selected. This signal is valid when DP_CTL.PTM_SEL is '0' (SWJ mode selected) and SWJ_CONNECTED is '1' (SWJ is connected). '0': SWD selected. '1': JTAG selected."]
    #[inline(always)]
    pub fn swj_jtag_sel(&self) -> SWJ_JTAG_SEL_R {
        SWJ_JTAG_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
