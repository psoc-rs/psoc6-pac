#[doc = "Register `PWR_CTRL_SM_ST` reader"]
pub struct R(crate::R<PWR_CTRL_SM_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL_SM_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL_SM_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL_SM_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PWR_CTRL_SM_CURR_STATE` reader - This register reflects the current state of the LL Power Control FSM 4'h0 - IDLE 4'h1 - SLEEP 4'h2 - DEEP_SLEEP 4'h4 - WAIT_OSC_STABLE 4'h5 - INTR_GEN 4'h6 - ACTIVE 4'h7 - REQ_RF_OFF"]
pub type PWR_CTRL_SM_CURR_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - This register reflects the current state of the LL Power Control FSM 4'h0 - IDLE 4'h1 - SLEEP 4'h2 - DEEP_SLEEP 4'h4 - WAIT_OSC_STABLE 4'h5 - INTR_GEN 4'h6 - ACTIVE 4'h7 - REQ_RF_OFF"]
    #[inline(always)]
    pub fn pwr_ctrl_sm_curr_state(&self) -> PWR_CTRL_SM_CURR_STATE_R {
        PWR_CTRL_SM_CURR_STATE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Link Layer Power Control FSM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl_sm_st](index.html) module"]
pub struct PWR_CTRL_SM_ST_SPEC;
impl crate::RegisterSpec for PWR_CTRL_SM_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl_sm_st::R](R) reader structure"]
impl crate::Readable for PWR_CTRL_SM_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_CTRL_SM_ST to value 0"]
impl crate::Resettable for PWR_CTRL_SM_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
