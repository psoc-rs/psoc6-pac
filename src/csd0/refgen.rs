#[doc = "Reader of register REFGEN"]
pub type R = crate::R<u32, super::REFGEN>;
#[doc = "Writer for register REFGEN"]
pub type W = crate::W<u32, super::REFGEN>;
#[doc = "Register REFGEN `reset()`'s with value 0"]
impl crate::ResetValue for super::REFGEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reference Generator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGEN_EN_A {
    #[doc = "0: Disable Reference Generator"]
    OFF = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 1,
}
impl From<REFGEN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: REFGEN_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFGEN_EN`"]
pub type REFGEN_EN_R = crate::R<bool, REFGEN_EN_A>;
impl REFGEN_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGEN_EN_A {
        match self.bits {
            false => REFGEN_EN_A::OFF,
            true => REFGEN_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == REFGEN_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == REFGEN_EN_A::ON
    }
}
#[doc = "Write proxy for field `REFGEN_EN`"]
pub struct REFGEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGEN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFGEN_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Reference Generator"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(REFGEN_EN_A::OFF)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(REFGEN_EN_A::ON)
    }
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
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `VDDA_EN`"]
pub type VDDA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDA_EN`"]
pub struct VDDA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RES_EN`"]
pub type RES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RES_EN`"]
pub struct RES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `VREFLO_SEL`"]
pub type VREFLO_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREFLO_SEL`"]
pub struct VREFLO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFLO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `VREFLO_INT`"]
pub type VREFLO_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFLO_INT`"]
pub struct VREFLO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFLO_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    pub fn refgen_en(&self) -> REFGEN_EN_R {
        REFGEN_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn vdda_en(&self) -> VDDA_EN_R {
        VDDA_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn res_en(&self) -> RES_EN_R {
        RES_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn vreflo_sel(&self) -> VREFLO_SEL_R {
        VREFLO_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub fn vreflo_int(&self) -> VREFLO_INT_R {
        VREFLO_INT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    pub fn refgen_en(&mut self) -> REFGEN_EN_W {
        REFGEN_EN_W { w: self }
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn vdda_en(&mut self) -> VDDA_EN_W {
        VDDA_EN_W { w: self }
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn res_en(&mut self) -> RES_EN_W {
        RES_EN_W { w: self }
    }
    #[doc = "Bits 8:12 - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bits 16:20 - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn vreflo_sel(&mut self) -> VREFLO_SEL_W {
        VREFLO_SEL_W { w: self }
    }
    #[doc = "Bit 23 - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub fn vreflo_int(&mut self) -> VREFLO_INT_W {
        VREFLO_INT_W { w: self }
    }
}
