#[doc = "Register `DP_STATUS` reader"]
pub struct R(crate::R<DP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWJ_CONNECTED` reader - Specifies if the SWJ debug port is connected; i.e. debug host interface is active: '0': Not connected/not active. '1': Connected/active."]
pub type SWJ_CONNECTED_R = crate::BitReader<bool>;
#[doc = "Field `SWJ_DEBUG_EN` reader - Specifies if SWJ debug is enabled, i.e. CDBGPWRUPACK is '1' and thus debug clocks are on: '0': Disabled. '1': Enabled."]
pub type SWJ_DEBUG_EN_R = crate::BitReader<bool>;
#[doc = "Field `SWJ_JTAG_SEL` reader - Specifies if the JTAG or SWD interface is selected. This signal is valid when DP_CTL.PTM_SEL is '0' (SWJ mode selected) and SWJ_CONNECTED is '1' (SWJ is connected). '0': SWD selected. '1': JTAG selected."]
pub type SWJ_JTAG_SEL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Specifies if the SWJ debug port is connected; i.e. debug host interface is active: '0': Not connected/not active. '1': Connected/active."]
    #[inline(always)]
    pub fn swj_connected(&self) -> SWJ_CONNECTED_R {
        SWJ_CONNECTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies if SWJ debug is enabled, i.e. CDBGPWRUPACK is '1' and thus debug clocks are on: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn swj_debug_en(&self) -> SWJ_DEBUG_EN_R {
        SWJ_DEBUG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specifies if the JTAG or SWD interface is selected. This signal is valid when DP_CTL.PTM_SEL is '0' (SWJ mode selected) and SWJ_CONNECTED is '1' (SWJ is connected). '0': SWD selected. '1': JTAG selected."]
    #[inline(always)]
    pub fn swj_jtag_sel(&self) -> SWJ_JTAG_SEL_R {
        SWJ_JTAG_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Debug port status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dp_status](index.html) module"]
pub struct DP_STATUS_SPEC;
impl crate::RegisterSpec for DP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dp_status::R](R) reader structure"]
impl crate::Readable for DP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DP_STATUS to value 0x04"]
impl crate::Resettable for DP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
