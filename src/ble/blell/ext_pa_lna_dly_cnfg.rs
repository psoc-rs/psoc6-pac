#[doc = "Reader of register EXT_PA_LNA_DLY_CNFG"]
pub type R = crate::R<u32, super::EXT_PA_LNA_DLY_CNFG>;
#[doc = "Writer for register EXT_PA_LNA_DLY_CNFG"]
pub type W = crate::W<u32, super::EXT_PA_LNA_DLY_CNFG>;
#[doc = "Register EXT_PA_LNA_DLY_CNFG `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_PA_LNA_DLY_CNFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LNA_CTL_DELAY`"]
pub type LNA_CTL_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNA_CTL_DELAY`"]
pub struct LNA_CTL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CTL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PA_CTL_DELAY`"]
pub type PA_CTL_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PA_CTL_DELAY`"]
pub struct PA_CTL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_CTL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The delay used to assert LNA_CTL, LNA_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the external Low Noise Amplifier. The value to be programmed to the lna_ctl_delay \\[7:0\\] = rx_on_delay - LNA_tRamp rx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[7:0\\]) LNA_tRamp = External Low Noise Amplifier startup time"]
    #[inline(always)]
    pub fn lna_ctl_delay(&self) -> LNA_CTL_DELAY_R {
        LNA_CTL_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The delay used to assert PA_CTL exactly PA_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the external power amplifier. The value to be programmed to the pa_ctl_delay \\[7:0\\] = tx_on_delay - PA_tRamp tx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[15:8\\]) PA_tRamp = External Power Amplifier ramp time"]
    #[inline(always)]
    pub fn pa_ctl_delay(&self) -> PA_CTL_DELAY_R {
        PA_CTL_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The delay used to assert LNA_CTL, LNA_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the external Low Noise Amplifier. The value to be programmed to the lna_ctl_delay \\[7:0\\] = rx_on_delay - LNA_tRamp rx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[7:0\\]) LNA_tRamp = External Low Noise Amplifier startup time"]
    #[inline(always)]
    pub fn lna_ctl_delay(&mut self) -> LNA_CTL_DELAY_W {
        LNA_CTL_DELAY_W { w: self }
    }
    #[doc = "Bits 8:15 - The delay used to assert PA_CTL exactly PA_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the external power amplifier. The value to be programmed to the pa_ctl_delay \\[7:0\\] = tx_on_delay - PA_tRamp tx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[15:8\\]) PA_tRamp = External Power Amplifier ramp time"]
    #[inline(always)]
    pub fn pa_ctl_delay(&mut self) -> PA_CTL_DELAY_W {
        PA_CTL_DELAY_W { w: self }
    }
}
