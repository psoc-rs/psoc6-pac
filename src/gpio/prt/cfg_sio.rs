#[doc = "Reader of register CFG_SIO"]
pub type R = crate::R<u32, super::CFG_SIO>;
#[doc = "Writer for register CFG_SIO"]
pub type W = crate::W<u32, super::CFG_SIO>;
#[doc = "Register CFG_SIO `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG_SIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREG_EN01`"]
pub type VREG_EN01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREG_EN01`"]
pub struct VREG_EN01_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN01_W<'a> {
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
#[doc = "Reader of field `IBUF_SEL01`"]
pub type IBUF_SEL01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBUF_SEL01`"]
pub struct IBUF_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL01_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `VTRIP_SEL01`"]
pub type VTRIP_SEL01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL01`"]
pub struct VTRIP_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL01_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `VREF_SEL01`"]
pub type VREF_SEL01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREF_SEL01`"]
pub struct VREF_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `VOH_SEL01`"]
pub type VOH_SEL01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VOH_SEL01`"]
pub struct VOH_SEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `VREG_EN23`"]
pub type VREG_EN23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREG_EN23`"]
pub struct VREG_EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN23_W<'a> {
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
#[doc = "Reader of field `IBUF_SEL23`"]
pub type IBUF_SEL23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBUF_SEL23`"]
pub struct IBUF_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL23_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL23`"]
pub type VTRIP_SEL23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL23`"]
pub struct VTRIP_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL23_W<'a> {
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
#[doc = "Reader of field `VREF_SEL23`"]
pub type VREF_SEL23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREF_SEL23`"]
pub struct VREF_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `VOH_SEL23`"]
pub type VOH_SEL23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VOH_SEL23`"]
pub struct VOH_SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `VREG_EN45`"]
pub type VREG_EN45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREG_EN45`"]
pub struct VREG_EN45_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN45_W<'a> {
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
#[doc = "Reader of field `IBUF_SEL45`"]
pub type IBUF_SEL45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBUF_SEL45`"]
pub struct IBUF_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL45_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL45`"]
pub type VTRIP_SEL45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL45`"]
pub struct VTRIP_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL45_W<'a> {
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
#[doc = "Reader of field `VREF_SEL45`"]
pub type VREF_SEL45_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREF_SEL45`"]
pub struct VREF_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `VOH_SEL45`"]
pub type VOH_SEL45_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VOH_SEL45`"]
pub struct VOH_SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `VREG_EN67`"]
pub type VREG_EN67_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREG_EN67`"]
pub struct VREG_EN67_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG_EN67_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `IBUF_SEL67`"]
pub type IBUF_SEL67_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBUF_SEL67`"]
pub struct IBUF_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUF_SEL67_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `VTRIP_SEL67`"]
pub type VTRIP_SEL67_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL67`"]
pub struct VTRIP_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL67_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `VREF_SEL67`"]
pub type VREF_SEL67_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREF_SEL67`"]
pub struct VREF_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_SEL67_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `VOH_SEL67`"]
pub type VOH_SEL67_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VOH_SEL67`"]
pub struct VOH_SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> VOH_SEL67_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&self) -> VREG_EN01_R {
        VREG_EN01_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel01(&self) -> IBUF_SEL01_R {
        IBUF_SEL01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel01(&self) -> VTRIP_SEL01_R {
        VTRIP_SEL01_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn vref_sel01(&self) -> VREF_SEL01_R {
        VREF_SEL01_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\] -> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip_point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\] -> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\] -> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip_point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\] -> Input buffer functions as LVTTL input buffer."]
    #[inline(always)]
    pub fn voh_sel01(&self) -> VOH_SEL01_R {
        VOH_SEL01_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn vreg_en23(&self) -> VREG_EN23_R {
        VREG_EN23_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel23(&self) -> IBUF_SEL23_R {
        IBUF_SEL23_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel23(&self) -> VTRIP_SEL23_R {
        VTRIP_SEL23_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    pub fn vref_sel23(&self) -> VREF_SEL23_R {
        VREF_SEL23_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - N/A"]
    #[inline(always)]
    pub fn voh_sel23(&self) -> VOH_SEL23_R {
        VOH_SEL23_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn vreg_en45(&self) -> VREG_EN45_R {
        VREG_EN45_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel45(&self) -> IBUF_SEL45_R {
        IBUF_SEL45_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel45(&self) -> VTRIP_SEL45_R {
        VTRIP_SEL45_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - N/A"]
    #[inline(always)]
    pub fn vref_sel45(&self) -> VREF_SEL45_R {
        VREF_SEL45_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:23 - N/A"]
    #[inline(always)]
    pub fn voh_sel45(&self) -> VOH_SEL45_R {
        VOH_SEL45_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn vreg_en67(&self) -> VREG_EN67_R {
        VREG_EN67_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel67(&self) -> IBUF_SEL67_R {
        IBUF_SEL67_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel67(&self) -> VTRIP_SEL67_R {
        VTRIP_SEL67_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - N/A"]
    #[inline(always)]
    pub fn vref_sel67(&self) -> VREF_SEL67_R {
        VREF_SEL67_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 29:31 - N/A"]
    #[inline(always)]
    pub fn voh_sel67(&self) -> VOH_SEL67_R {
        VOH_SEL67_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&mut self) -> VREG_EN01_W {
        VREG_EN01_W { w: self }
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel01(&mut self) -> IBUF_SEL01_W {
        IBUF_SEL01_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel01(&mut self) -> VTRIP_SEL01_W {
        VTRIP_SEL01_W { w: self }
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn vref_sel01(&mut self) -> VREF_SEL01_W {
        VREF_SEL01_W { w: self }
    }
    #[doc = "Bits 5:7 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\] -> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip_point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\] -> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\] -> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip_point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\] -> Input buffer functions as LVTTL input buffer."]
    #[inline(always)]
    pub fn voh_sel01(&mut self) -> VOH_SEL01_W {
        VOH_SEL01_W { w: self }
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn vreg_en23(&mut self) -> VREG_EN23_W {
        VREG_EN23_W { w: self }
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel23(&mut self) -> IBUF_SEL23_W {
        IBUF_SEL23_W { w: self }
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel23(&mut self) -> VTRIP_SEL23_W {
        VTRIP_SEL23_W { w: self }
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    pub fn vref_sel23(&mut self) -> VREF_SEL23_W {
        VREF_SEL23_W { w: self }
    }
    #[doc = "Bits 13:15 - N/A"]
    #[inline(always)]
    pub fn voh_sel23(&mut self) -> VOH_SEL23_W {
        VOH_SEL23_W { w: self }
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn vreg_en45(&mut self) -> VREG_EN45_W {
        VREG_EN45_W { w: self }
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel45(&mut self) -> IBUF_SEL45_W {
        IBUF_SEL45_W { w: self }
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel45(&mut self) -> VTRIP_SEL45_W {
        VTRIP_SEL45_W { w: self }
    }
    #[doc = "Bits 19:20 - N/A"]
    #[inline(always)]
    pub fn vref_sel45(&mut self) -> VREF_SEL45_W {
        VREF_SEL45_W { w: self }
    }
    #[doc = "Bits 21:23 - N/A"]
    #[inline(always)]
    pub fn voh_sel45(&mut self) -> VOH_SEL45_W {
        VOH_SEL45_W { w: self }
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn vreg_en67(&mut self) -> VREG_EN67_W {
        VREG_EN67_W { w: self }
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel67(&mut self) -> IBUF_SEL67_W {
        IBUF_SEL67_W { w: self }
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel67(&mut self) -> VTRIP_SEL67_W {
        VTRIP_SEL67_W { w: self }
    }
    #[doc = "Bits 27:28 - N/A"]
    #[inline(always)]
    pub fn vref_sel67(&mut self) -> VREF_SEL67_W {
        VREF_SEL67_W { w: self }
    }
    #[doc = "Bits 29:31 - N/A"]
    #[inline(always)]
    pub fn voh_sel67(&mut self) -> VOH_SEL67_W {
        VOH_SEL67_W { w: self }
    }
}
