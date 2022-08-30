#[doc = "Register `MT_STATUS` reader"]
pub struct R(crate::R<MT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLESS_STATE` reader - 1'b0 - BLESS in DPSLP state 1'b1 - BLESS in ACTIVE state"]
pub type BLESS_STATE_R = crate::BitReader<bool>;
#[doc = "Field `MT_CURR_STATE` reader - This register reflects the current state of the MT FSM 4'h0 - IDLE 4'h1 - BLERD_DEEPSLEEP 4'h2 - HVLDO_STARTUP 4'h3 - WAIT_CLK 4'h4 - BLERD_IDLE 4'h5 - SWITCH_EN 4'h6 - ACTIVE 4'h7 - ISOLATE 4'h8 - WAIT_IDLE 4'h9 - XTAL_DISABLE 4'hA - HVLDO_DISABLE"]
pub type MT_CURR_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVLDO_STARTUP_CURR_STATE` reader - This register reflects the current state of the HVLDO Startup FSM 3'h0 - HVLDO_OFF 3'h1 - HVLDO_WAIT 3'h2 - HVLDO_SAMPLE 3'h3 - HVLDO_ENABLED 3'h4 - HVLDO_SET_BYPASS"]
pub type HVLDO_STARTUP_CURR_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LL_CLK_STATE` reader - This bit indicates when the Link Layer registers are accessible upon a DSM exit. This bit should not be used after a DSM entry command has been issued. 1'b0 - Link Layer clock is not available 1'b1 - Link Layer clock is active"]
pub type LL_CLK_STATE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 1'b0 - BLESS in DPSLP state 1'b1 - BLESS in ACTIVE state"]
    #[inline(always)]
    pub fn bless_state(&self) -> BLESS_STATE_R {
        BLESS_STATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This register reflects the current state of the MT FSM 4'h0 - IDLE 4'h1 - BLERD_DEEPSLEEP 4'h2 - HVLDO_STARTUP 4'h3 - WAIT_CLK 4'h4 - BLERD_IDLE 4'h5 - SWITCH_EN 4'h6 - ACTIVE 4'h7 - ISOLATE 4'h8 - WAIT_IDLE 4'h9 - XTAL_DISABLE 4'hA - HVLDO_DISABLE"]
    #[inline(always)]
    pub fn mt_curr_state(&self) -> MT_CURR_STATE_R {
        MT_CURR_STATE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - This register reflects the current state of the HVLDO Startup FSM 3'h0 - HVLDO_OFF 3'h1 - HVLDO_WAIT 3'h2 - HVLDO_SAMPLE 3'h3 - HVLDO_ENABLED 3'h4 - HVLDO_SET_BYPASS"]
    #[inline(always)]
    pub fn hvldo_startup_curr_state(&self) -> HVLDO_STARTUP_CURR_STATE_R {
        HVLDO_STARTUP_CURR_STATE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - This bit indicates when the Link Layer registers are accessible upon a DSM exit. This bit should not be used after a DSM entry command has been issued. 1'b0 - Link Layer clock is not available 1'b1 - Link Layer clock is active"]
    #[inline(always)]
    pub fn ll_clk_state(&self) -> LL_CLK_STATE_R {
        LL_CLK_STATE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "MT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_status](index.html) module"]
pub struct MT_STATUS_SPEC;
impl crate::RegisterSpec for MT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mt_status::R](R) reader structure"]
impl crate::Readable for MT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MT_STATUS to value 0"]
impl crate::Resettable for MT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
