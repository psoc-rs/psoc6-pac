#[doc = "Register `USBIO_CTL` reader"]
pub struct R(crate::R<USBIO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIO_CTL` writer"]
pub struct W(crate::W<USBIO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USBIO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DM_P_A {
    #[doc = "0: Mode 0: Output buffer off (high Z). Input buffer off."]
    OFF = 0,
    #[doc = "1: Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    INPUT = 1,
}
impl From<DM_P_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_P_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DM_P` reader - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
pub type DM_P_R = crate::FieldReader<u8, DM_P_A>;
impl DM_P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DM_P_A> {
        match self.bits {
            0 => Some(DM_P_A::OFF),
            1 => Some(DM_P_A::INPUT),
            _ => None,
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
#[doc = "Field `DM_P` writer - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
pub type DM_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBIO_CTL_SPEC, u8, DM_P_A, 3, O>;
impl<'a, const O: u8> DM_P_W<'a, O> {
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
}
#[doc = "Field `DM_M` reader - The GPIO Drive Mode for DM IO pad."]
pub type DM_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DM_M` writer - The GPIO Drive Mode for DM IO pad."]
pub type DM_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBIO_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub fn dm_p(&self) -> DM_P_R {
        DM_P_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub fn dm_m(&self) -> DM_M_R {
        DM_M_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub fn dm_p(&mut self) -> DM_P_W<0> {
        DM_P_W::new(self)
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub fn dm_m(&mut self) -> DM_M_W<3> {
        DM_M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB IO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_ctl](index.html) module"]
pub struct USBIO_CTL_SPEC;
impl crate::RegisterSpec for USBIO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbio_ctl::R](R) reader structure"]
impl crate::Readable for USBIO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbio_ctl::W](W) writer structure"]
impl crate::Writable for USBIO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIO_CTL to value 0"]
impl crate::Resettable for USBIO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
