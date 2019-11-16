#[doc = "Reader of register AMBUF"]
pub type R = crate::R<u32, super::AMBUF>;
#[doc = "Writer for register AMBUF"]
pub type W = crate::W<u32, super::AMBUF>;
#[doc = "Register AMBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::AMBUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Amux buffer power level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Disable buffer"]
    OFF = 0,
    #[doc = "1: On, normal or low power level depending on CONFIG.LP_MODE."]
    NORM = 1,
    #[doc = "2: On, high or low power level depending on CONFIG.LP_MODE."]
    HI = 2,
}
impl From<PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWR_MODE`"]
pub type PWR_MODE_R = crate::R<u8, PWR_MODE_A>;
impl PWR_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWR_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWR_MODE_A::OFF),
            1 => Val(PWR_MODE_A::NORM),
            2 => Val(PWR_MODE_A::HI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == PWR_MODE_A::NORM
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == PWR_MODE_A::HI
    }
}
#[doc = "Write proxy for field `PWR_MODE`"]
pub struct PWR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWR_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable buffer"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "On, normal or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(PWR_MODE_A::NORM)
    }
    #[doc = "On, high or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(PWR_MODE_A::HI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W {
        PWR_MODE_W { w: self }
    }
}
