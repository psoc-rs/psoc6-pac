#[doc = "Reader of register TEST_CTL"]
pub type R = crate::R<u32, super::TEST_CTL>;
#[doc = "Writer for register TEST_CTL"]
pub type W = crate::W<u32, super::TEST_CTL>;
#[doc = "Register TEST_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEST_MODE`"]
pub type TEST_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST_MODE`"]
pub struct TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PN_CTL`"]
pub type PN_CTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PN_CTL`"]
pub struct PN_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PN_CTL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TM_PE`"]
pub type TM_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TM_PE`"]
pub struct TM_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TM_DISPOS`"]
pub type TM_DISPOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TM_DISPOS`"]
pub struct TM_DISPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_DISPOS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TM_DISNEG`"]
pub type TM_DISNEG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TM_DISNEG`"]
pub struct TM_DISNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_DISNEG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `EN_CLK_MON`"]
pub type EN_CLK_MON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_CLK_MON`"]
pub struct EN_CLK_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CLK_MON_W<'a> {
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
#[doc = "Reader of field `CSL_DEBUG`"]
pub type CSL_DEBUG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSL_DEBUG`"]
pub struct CSL_DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> CSL_DEBUG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_OSC`"]
pub type ENABLE_OSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_OSC`"]
pub struct ENABLE_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_OSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `UNSCRAMBLE_WA`"]
pub type UNSCRAMBLE_WA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNSCRAMBLE_WA`"]
pub struct UNSCRAMBLE_WA_W<'a> {
    w: &'a mut W,
}
impl<'a> UNSCRAMBLE_WA_W<'a> {
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
    #[doc = "Bits 0:4 - Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Postive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub fn pn_ctl(&self) -> PN_CTL_R {
        PN_CTL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub fn tm_pe(&self) -> TM_PE_R {
        TM_PE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Test mode positive pump disable"]
    #[inline(always)]
    pub fn tm_dispos(&self) -> TM_DISPOS_R {
        TM_DISPOS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Test mode negative pump disable"]
    #[inline(always)]
    pub fn tm_disneg(&self) -> TM_DISNEG_R {
        TM_DISNEG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1: enables the oscillator output monitor"]
    #[inline(always)]
    pub fn en_clk_mon(&self) -> EN_CLK_MON_R {
        EN_CLK_MON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Engineering Debug Register"]
    #[inline(always)]
    pub fn csl_debug(&self) -> CSL_DEBUG_R {
        CSL_DEBUG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub fn enable_osc(&self) -> ENABLE_OSC_R {
        ENABLE_OSC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 31 - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub fn unscramble_wa(&self) -> UNSCRAMBLE_WA_R {
        UNSCRAMBLE_WA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W {
        TEST_MODE_W { w: self }
    }
    #[doc = "Bit 8 - Postive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub fn pn_ctl(&mut self) -> PN_CTL_W {
        PN_CTL_W { w: self }
    }
    #[doc = "Bit 9 - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub fn tm_pe(&mut self) -> TM_PE_W {
        TM_PE_W { w: self }
    }
    #[doc = "Bit 10 - Test mode positive pump disable"]
    #[inline(always)]
    pub fn tm_dispos(&mut self) -> TM_DISPOS_W {
        TM_DISPOS_W { w: self }
    }
    #[doc = "Bit 11 - Test mode negative pump disable"]
    #[inline(always)]
    pub fn tm_disneg(&mut self) -> TM_DISNEG_W {
        TM_DISNEG_W { w: self }
    }
    #[doc = "Bit 16 - 1: enables the oscillator output monitor"]
    #[inline(always)]
    pub fn en_clk_mon(&mut self) -> EN_CLK_MON_W {
        EN_CLK_MON_W { w: self }
    }
    #[doc = "Bit 17 - Engineering Debug Register"]
    #[inline(always)]
    pub fn csl_debug(&mut self) -> CSL_DEBUG_W {
        CSL_DEBUG_W { w: self }
    }
    #[doc = "Bit 18 - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub fn enable_osc(&mut self) -> ENABLE_OSC_W {
        ENABLE_OSC_W { w: self }
    }
    #[doc = "Bit 31 - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub fn unscramble_wa(&mut self) -> UNSCRAMBLE_WA_W {
        UNSCRAMBLE_WA_W { w: self }
    }
}
