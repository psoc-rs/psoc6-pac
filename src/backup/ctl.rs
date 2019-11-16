#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WCO_EN`"]
pub type WCO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WCO_EN`"]
pub struct WCO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WCO_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Clock select for BAK clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SEL_A {
    #[doc = "0: Watch-crystal oscillator input."]
    WCO = 0,
    #[doc = "1: This allows to use the LFCLK selection as an alternate backup domain clock.  Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped.  For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    ALTBAK = 1,
}
impl From<CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SEL`"]
pub type CLK_SEL_R = crate::R<u8, CLK_SEL_A>;
impl CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_SEL_A::WCO),
            1 => Val(CLK_SEL_A::ALTBAK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCO`"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == CLK_SEL_A::WCO
    }
    #[doc = "Checks if the value of the field is `ALTBAK`"]
    #[inline(always)]
    pub fn is_altbak(&self) -> bool {
        *self == CLK_SEL_A::ALTBAK
    }
}
#[doc = "Write proxy for field `CLK_SEL`"]
pub struct CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Watch-crystal oscillator input."]
    #[inline(always)]
    pub fn wco(self) -> &'a mut W {
        self.variant(CLK_SEL_A::WCO)
    }
    #[doc = "This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    #[inline(always)]
    pub fn altbak(self) -> &'a mut W {
        self.variant(CLK_SEL_A::ALTBAK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `WCO_BYPASS`"]
pub type WCO_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WCO_BYPASS`"]
pub struct WCO_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> WCO_BYPASS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `VDDBAK_CTL`"]
pub type VDDBAK_CTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDBAK_CTL`"]
pub struct VDDBAK_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDBAK_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `VBACKUP_MEAS`"]
pub type VBACKUP_MEAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBACKUP_MEAS`"]
pub struct VBACKUP_MEAS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBACKUP_MEAS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `EN_CHARGE_KEY`"]
pub type EN_CHARGE_KEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EN_CHARGE_KEY`"]
pub struct EN_CHARGE_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CHARGE_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs."]
    #[inline(always)]
    pub fn wco_en(&self) -> WCO_EN_R {
        WCO_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Clock select for BAK clock"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    pub fn wco_bypass(&self) -> WCO_BYPASS_R {
        WCO_BYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    pub fn vddbak_ctl(&self) -> VDDBAK_CTL_R {
        VDDBAK_CTL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled by 40 percent so it is within the supply range of the ADC."]
    #[inline(always)]
    pub fn vbackup_meas(&self) -> VBACKUP_MEAS_R {
        VBACKUP_MEAS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    pub fn en_charge_key(&self) -> EN_CHARGE_KEY_R {
        EN_CHARGE_KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs."]
    #[inline(always)]
    pub fn wco_en(&mut self) -> WCO_EN_W {
        WCO_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - Clock select for BAK clock"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Bit 16 - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    pub fn wco_bypass(&mut self) -> WCO_BYPASS_W {
        WCO_BYPASS_W { w: self }
    }
    #[doc = "Bits 17:18 - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    pub fn vddbak_ctl(&mut self) -> VDDBAK_CTL_W {
        VDDBAK_CTL_W { w: self }
    }
    #[doc = "Bit 19 - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled by 40 percent so it is within the supply range of the ADC."]
    #[inline(always)]
    pub fn vbackup_meas(&mut self) -> VBACKUP_MEAS_W {
        VBACKUP_MEAS_W { w: self }
    }
    #[doc = "Bits 24:31 - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    pub fn en_charge_key(&mut self) -> EN_CHARGE_KEY_W {
        EN_CHARGE_KEY_W { w: self }
    }
}
