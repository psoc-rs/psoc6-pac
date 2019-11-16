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
#[doc = "Reader of field `CNT_DURATION`"]
pub type CNT_DURATION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT_DURATION`"]
pub struct CNT_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_DURATION_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REF_CLK_SEL_A {
    #[doc = "0: Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    CLK_TIMER = 0,
    #[doc = "1: IMO - Internal Main Oscillator"]
    CLK_IMO = 1,
    #[doc = "2: ECO - External-Crystal Oscillator"]
    CLK_ECO = 2,
    #[doc = "3: Low frequency clock (ILO, WCO or ALTLF).\nSelection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    CLK_LF = 3,
    #[doc = "4: High frequuency clock ('clk_hfx')."]
    CLK_HF = 4,
    #[doc = "5: Peripheral clock ('clk_peri')."]
    CLK_PERI = 5,
    #[doc = "6: N/A"]
    RSVD_6 = 6,
    #[doc = "7: N/A"]
    RSVD_7 = 7,
}
impl From<REF_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REF_CLK_SEL`"]
pub type REF_CLK_SEL_R = crate::R<u8, REF_CLK_SEL_A>;
impl REF_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_CLK_SEL_A {
        match self.bits {
            0 => REF_CLK_SEL_A::CLK_TIMER,
            1 => REF_CLK_SEL_A::CLK_IMO,
            2 => REF_CLK_SEL_A::CLK_ECO,
            3 => REF_CLK_SEL_A::CLK_LF,
            4 => REF_CLK_SEL_A::CLK_HF,
            5 => REF_CLK_SEL_A::CLK_PERI,
            6 => REF_CLK_SEL_A::RSVD_6,
            7 => REF_CLK_SEL_A::RSVD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_TIMER`"]
    #[inline(always)]
    pub fn is_clk_timer(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_TIMER
    }
    #[doc = "Checks if the value of the field is `CLK_IMO`"]
    #[inline(always)]
    pub fn is_clk_imo(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_IMO
    }
    #[doc = "Checks if the value of the field is `CLK_ECO`"]
    #[inline(always)]
    pub fn is_clk_eco(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_ECO
    }
    #[doc = "Checks if the value of the field is `CLK_LF`"]
    #[inline(always)]
    pub fn is_clk_lf(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_LF
    }
    #[doc = "Checks if the value of the field is `CLK_HF`"]
    #[inline(always)]
    pub fn is_clk_hf(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_HF
    }
    #[doc = "Checks if the value of the field is `CLK_PERI`"]
    #[inline(always)]
    pub fn is_clk_peri(&self) -> bool {
        *self == REF_CLK_SEL_A::CLK_PERI
    }
    #[doc = "Checks if the value of the field is `RSVD_6`"]
    #[inline(always)]
    pub fn is_rsvd_6(&self) -> bool {
        *self == REF_CLK_SEL_A::RSVD_6
    }
    #[doc = "Checks if the value of the field is `RSVD_7`"]
    #[inline(always)]
    pub fn is_rsvd_7(&self) -> bool {
        *self == REF_CLK_SEL_A::RSVD_7
    }
}
#[doc = "Write proxy for field `REF_CLK_SEL`"]
pub struct REF_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REF_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    #[inline(always)]
    pub fn clk_timer(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::CLK_TIMER)
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn clk_imo(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::CLK_IMO)
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn clk_eco(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::CLK_ECO)
    }
    #[doc = "Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    #[inline(always)]
    pub fn clk_lf(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::CLK_LF)
    }
    #[doc = "High frequuency clock ('clk_hfx')."]
    #[inline(always)]
    pub fn clk_hf(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::CLK_HF)
    }
    #[doc = "Peripheral clock ('clk_peri')."]
    #[inline(always)]
    pub fn clk_peri(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::CLK_PERI)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_6(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::RSVD_6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_7(self) -> &'a mut W {
        self.variant(REF_CLK_SEL_A::RSVD_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MON_SEL`"]
pub type MON_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MON_SEL`"]
pub struct MON_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    pub fn cnt_duration(&self) -> CNT_DURATION_R {
        CNT_DURATION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    pub fn ref_clk_sel(&self) -> REF_CLK_SEL_R {
        REF_CLK_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 16:22 - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    pub fn mon_sel(&self) -> MON_SEL_R {
        MON_SEL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    pub fn cnt_duration(&mut self) -> CNT_DURATION_W {
        CNT_DURATION_W { w: self }
    }
    #[doc = "Bits 4:6 - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    pub fn ref_clk_sel(&mut self) -> REF_CLK_SEL_W {
        REF_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 16:22 - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    pub fn mon_sel(&mut self) -> MON_SEL_W {
        MON_SEL_W { w: self }
    }
    #[doc = "Bit 31 - Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
