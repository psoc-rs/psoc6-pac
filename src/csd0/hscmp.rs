#[doc = "Reader of register HSCMP"]
pub type R = crate::R<u32, super::HSCMP>;
#[doc = "Writer for register HSCMP"]
pub type W = crate::W<u32, super::HSCMP>;
#[doc = "Register HSCMP `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "High Speed Comparator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSCMP_EN_A {
    #[doc = "0: Disable comparator, output is zero"]
    OFF = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 1,
}
impl From<HSCMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HSCMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSCMP_EN`"]
pub type HSCMP_EN_R = crate::R<bool, HSCMP_EN_A>;
impl HSCMP_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSCMP_EN_A {
        match self.bits {
            false => HSCMP_EN_A::OFF,
            true => HSCMP_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSCMP_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSCMP_EN_A::ON
    }
}
#[doc = "Write proxy for field `HSCMP_EN`"]
pub struct HSCMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSCMP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSCMP_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSCMP_EN_A::OFF)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSCMP_EN_A::ON)
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
#[doc = "Reader of field `HSCMP_INVERT`"]
pub type HSCMP_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSCMP_INVERT`"]
pub struct HSCMP_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSCMP_INVERT_W<'a> {
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
#[doc = "Reader of field `AZ_EN`"]
pub type AZ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AZ_EN`"]
pub struct AZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_EN_W<'a> {
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
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    pub fn hscmp_en(&self) -> HSCMP_EN_R {
        HSCMP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn hscmp_invert(&self) -> HSCMP_INVERT_R {
        HSCMP_INVERT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&self) -> AZ_EN_R {
        AZ_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    pub fn hscmp_en(&mut self) -> HSCMP_EN_W {
        HSCMP_EN_W { w: self }
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn hscmp_invert(&mut self) -> HSCMP_INVERT_W {
        HSCMP_INVERT_W { w: self }
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&mut self) -> AZ_EN_W {
        AZ_EN_W { w: self }
    }
}
