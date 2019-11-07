#[doc = "Reader of register CAL_CTL2"]
pub type R = crate::R<u32, super::CAL_CTL2>;
#[doc = "Writer for register CAL_CTL2"]
pub type W = crate::W<u32, super::CAL_CTL2>;
#[doc = "Register CAL_CTL2 `reset()`'s with value 0x7070"]
impl crate::ResetValue for super::CAL_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7070
    }
}
#[doc = "Reader of field `ICREF_TRIM_LO_HV`"]
pub type ICREF_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICREF_TRIM_LO_HV`"]
pub struct ICREF_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICREF_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ICREF_TC_TRIM_LO_HV`"]
pub type ICREF_TC_TRIM_LO_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICREF_TC_TRIM_LO_HV`"]
pub struct ICREF_TC_TRIM_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICREF_TC_TRIM_LO_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `ICREF_TRIM_HI_HV`"]
pub type ICREF_TRIM_HI_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICREF_TRIM_HI_HV`"]
pub struct ICREF_TRIM_HI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICREF_TRIM_HI_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ICREF_TC_TRIM_HI_HV`"]
pub type ICREF_TC_TRIM_HI_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICREF_TC_TRIM_HI_HV`"]
pub struct ICREF_TC_TRIM_HI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICREF_TC_TRIM_HI_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `VREF_SEL_HV`"]
pub type VREF_SEL_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREF_SEL_HV`"]
pub struct VREF_SEL_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL_HV_W<'a> {
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
#[doc = "Reader of field `IREF_SEL_HV`"]
pub type IREF_SEL_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IREF_SEL_HV`"]
pub struct IREF_SEL_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IREF_SEL_HV_W<'a> {
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
#[doc = "Reader of field `FM_ACTIVE_HV`"]
pub type FM_ACTIVE_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FM_ACTIVE_HV`"]
pub struct FM_ACTIVE_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_ACTIVE_HV_W<'a> {
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
#[doc = "Reader of field `TURBO_EXT_HV`"]
pub type TURBO_EXT_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TURBO_EXT_HV`"]
pub struct TURBO_EXT_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> TURBO_EXT_HV_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&self) -> ICREF_TRIM_LO_HV_R {
        ICREF_TRIM_LO_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&self) -> ICREF_TC_TRIM_LO_HV_R {
        ICREF_TC_TRIM_LO_HV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&self) -> ICREF_TRIM_HI_HV_R {
        ICREF_TRIM_HI_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub fn icref_tc_trim_hi_hv(&self) -> ICREF_TC_TRIM_HI_HV_R {
        ICREF_TC_TRIM_HI_HV_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
    #[inline(always)]
    pub fn vref_sel_hv(&self) -> VREF_SEL_HV_R {
        VREF_SEL_HV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Current reference: '0': internal current reference '1': external current reference"]
    #[inline(always)]
    pub fn iref_sel_hv(&self) -> IREF_SEL_HV_R {
        IREF_SEL_HV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0: No Action 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn fm_active_hv(&self) -> FM_ACTIVE_HV_R {
        FM_ACTIVE_HV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
    #[inline(always)]
    pub fn turbo_ext_hv(&self) -> TURBO_EXT_HV_R {
        TURBO_EXT_HV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&mut self) -> ICREF_TRIM_LO_HV_W {
        ICREF_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bits 5:7 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&mut self) -> ICREF_TC_TRIM_LO_HV_W {
        ICREF_TC_TRIM_LO_HV_W { w: self }
    }
    #[doc = "Bits 8:12 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&mut self) -> ICREF_TRIM_HI_HV_W {
        ICREF_TRIM_HI_HV_W { w: self }
    }
    #[doc = "Bits 13:15 - HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub fn icref_tc_trim_hi_hv(&mut self) -> ICREF_TC_TRIM_HI_HV_W {
        ICREF_TC_TRIM_HI_HV_W { w: self }
    }
    #[doc = "Bit 16 - Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
    #[inline(always)]
    pub fn vref_sel_hv(&mut self) -> VREF_SEL_HV_W {
        VREF_SEL_HV_W { w: self }
    }
    #[doc = "Bit 17 - Current reference: '0': internal current reference '1': external current reference"]
    #[inline(always)]
    pub fn iref_sel_hv(&mut self) -> IREF_SEL_HV_W {
        IREF_SEL_HV_W { w: self }
    }
    #[doc = "Bit 18 - 0: No Action 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn fm_active_hv(&mut self) -> FM_ACTIVE_HV_W {
        FM_ACTIVE_HV_W { w: self }
    }
    #[doc = "Bit 19 - 0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
    #[inline(always)]
    pub fn turbo_ext_hv(&mut self) -> TURBO_EXT_HV_W {
        TURBO_EXT_HV_W { w: self }
    }
}
