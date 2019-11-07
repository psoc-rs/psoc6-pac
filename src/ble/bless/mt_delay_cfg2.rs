#[doc = "Reader of register MT_DELAY_CFG2"]
pub type R = crate::R<u32, super::MT_DELAY_CFG2>;
#[doc = "Writer for register MT_DELAY_CFG2"]
pub type W = crate::W<u32, super::MT_DELAY_CFG2>;
#[doc = "Register MT_DELAY_CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MT_DELAY_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSC_STARTUP_DELAY_LF`"]
pub type OSC_STARTUP_DELAY_LF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSC_STARTUP_DELAY_LF`"]
pub struct OSC_STARTUP_DELAY_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_STARTUP_DELAY_LF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DSM_OFFSET_TO_WAKEUP_INSTANT_LF`"]
pub type DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSM_OFFSET_TO_WAKEUP_INSTANT_LF`"]
pub struct DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ACT_STARTUP_DELAY`"]
pub type ACT_STARTUP_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_STARTUP_DELAY`"]
pub struct ACT_STARTUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_STARTUP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIG_LDO_STARTUP_DELAY`"]
pub type DIG_LDO_STARTUP_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIG_LDO_STARTUP_DELAY`"]
pub struct DIG_LDO_STARTUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_LDO_STARTUP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register specifies the time for OSC Startup. After this delay, clock is enabled to the link layer. Clock is enabled after OSC_STARTUP_DELAY + 1 LF clock cycles. If PSoC was in DPSLP when XTAL is enabled, then the wakeup delay will be OSC_STARTUP_DELAY + 1 + PSoC Wakeup time. Minimum value to be programmed in 1. This is equivalent to Link Layer register WAKEUP_CONFIG.OSC_STARTUP_DELAY, but is specified in LF cycles"]
    #[inline(always)]
    pub fn osc_startup_delay_lf(&self) -> OSC_STARTUP_DELAY_LF_R {
        OSC_STARTUP_DELAY_LF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register specifies the pre-processing time required in Link Layer. This is esentially the time from CLK_EN (ungating clock in CYBLERD55) to the time when logic in CYBLERD55 is switched to Active mode Regulator.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. This is equivalent to Link Layer register WAKEUP_CONFIG.DSM_OFFSET_TO_WAKEUP_INSTANT_LF, but is specified in LF cycles."]
    #[inline(always)]
    pub fn dsm_offset_to_wakeup_instant_lf(&self) -> DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R {
        DSM_OFFSET_TO_WAKEUP_INSTANT_LF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register specifes the Active Regulator startup time in CYBLERD55. The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The digital LDO will be turned on after this time elapses"]
    #[inline(always)]
    pub fn act_startup_delay(&self) -> ACT_STARTUP_DELAY_R {
        ACT_STARTUP_DELAY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register specifes the Digital LDO startup time in CYBLERD55.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The logic in CYBLERD55 is switched to Active mode Regulator after this (ACT_STARTUP_DELAY + DIG_LDO_STARTUP_DELAY)"]
    #[inline(always)]
    pub fn dig_ldo_startup_delay(&self) -> DIG_LDO_STARTUP_DELAY_R {
        DIG_LDO_STARTUP_DELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the time for OSC Startup. After this delay, clock is enabled to the link layer. Clock is enabled after OSC_STARTUP_DELAY + 1 LF clock cycles. If PSoC was in DPSLP when XTAL is enabled, then the wakeup delay will be OSC_STARTUP_DELAY + 1 + PSoC Wakeup time. Minimum value to be programmed in 1. This is equivalent to Link Layer register WAKEUP_CONFIG.OSC_STARTUP_DELAY, but is specified in LF cycles"]
    #[inline(always)]
    pub fn osc_startup_delay_lf(&mut self) -> OSC_STARTUP_DELAY_LF_W {
        OSC_STARTUP_DELAY_LF_W { w: self }
    }
    #[doc = "Bits 8:15 - This register specifies the pre-processing time required in Link Layer. This is esentially the time from CLK_EN (ungating clock in CYBLERD55) to the time when logic in CYBLERD55 is switched to Active mode Regulator.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. This is equivalent to Link Layer register WAKEUP_CONFIG.DSM_OFFSET_TO_WAKEUP_INSTANT_LF, but is specified in LF cycles."]
    #[inline(always)]
    pub fn dsm_offset_to_wakeup_instant_lf(&mut self) -> DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W {
        DSM_OFFSET_TO_WAKEUP_INSTANT_LF_W { w: self }
    }
    #[doc = "Bits 16:23 - This register specifes the Active Regulator startup time in CYBLERD55. The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The digital LDO will be turned on after this time elapses"]
    #[inline(always)]
    pub fn act_startup_delay(&mut self) -> ACT_STARTUP_DELAY_W {
        ACT_STARTUP_DELAY_W { w: self }
    }
    #[doc = "Bits 24:31 - This register specifes the Digital LDO startup time in CYBLERD55.The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. The logic in CYBLERD55 is switched to Active mode Regulator after this (ACT_STARTUP_DELAY + DIG_LDO_STARTUP_DELAY)"]
    #[inline(always)]
    pub fn dig_ldo_startup_delay(&mut self) -> DIG_LDO_STARTUP_DELAY_W {
        DIG_LDO_STARTUP_DELAY_W { w: self }
    }
}
