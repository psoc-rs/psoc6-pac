#[doc = "Reader of register USBIO_CTL"]
pub type R = crate::R<u32, super::USBIO_CTL>;
#[doc = "Writer for register USBIO_CTL"]
pub type W = crate::W<u32, super::USBIO_CTL>;
#[doc = "Register USBIO_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBIO_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DM_P_A {
    #[doc = "0: Mode 0: Output buffer off (high Z). Input buffer off."]
    OFF = 0,
    #[doc = "1: Mode 1: Output buffer off (high Z). Input buffer on.\n\nOther values, not supported."]
    INPUT = 1,
}
impl From<DM_P_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_P_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DM_P`"]
pub type DM_P_R = crate::R<u8, DM_P_A>;
impl DM_P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DM_P_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DM_P_A::OFF),
            1 => Val(DM_P_A::INPUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DM_P_A::OFF
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DM_P_A::INPUT
    }
}
#[doc = "Write proxy for field `DM_P`"]
pub struct DM_P_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM_P_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Mode 0: Output buffer off (high Z). Input buffer off."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DM_P_A::OFF)
    }
    #[doc = "Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DM_P_A::INPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DM_M`"]
pub type DM_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DM_M`"]
pub struct DM_M_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub fn dm_p(&self) -> DM_P_R {
        DM_P_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub fn dm_m(&self) -> DM_M_R {
        DM_M_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub fn dm_p(&mut self) -> DM_P_W {
        DM_P_W { w: self }
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub fn dm_m(&mut self) -> DM_M_W {
        DM_M_W { w: self }
    }
}
