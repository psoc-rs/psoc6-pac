#[doc = "Register `AMBUF` reader"]
pub struct R(crate::R<AMBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMBUF` writer"]
pub struct W(crate::W<AMBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMBUF_SPEC>;
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
impl From<crate::W<AMBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMBUF_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `PWR_MODE` reader - Amux buffer power level"]
pub type PWR_MODE_R = crate::FieldReader<u8, PWR_MODE_A>;
impl PWR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWR_MODE_A> {
        match self.bits {
            0 => Some(PWR_MODE_A::OFF),
            1 => Some(PWR_MODE_A::NORM),
            2 => Some(PWR_MODE_A::HI),
            _ => None,
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
#[doc = "Field `PWR_MODE` writer - Amux buffer power level"]
pub type PWR_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMBUF_SPEC, u8, PWR_MODE_A, 2, O>;
impl<'a, const O: u8> PWR_MODE_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<0> {
        PWR_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Generator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ambuf](index.html) module"]
pub struct AMBUF_SPEC;
impl crate::RegisterSpec for AMBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ambuf::R](R) reader structure"]
impl crate::Readable for AMBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ambuf::W](W) writer structure"]
impl crate::Writable for AMBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMBUF to value 0"]
impl crate::Resettable for AMBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
