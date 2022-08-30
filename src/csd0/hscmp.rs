#[doc = "Register `HSCMP` reader"]
pub struct R(crate::R<HSCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCMP` writer"]
pub struct W(crate::W<HSCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCMP_SPEC>;
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
impl From<crate::W<HSCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCMP_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `HSCMP_EN` reader - High Speed Comparator enable"]
pub type HSCMP_EN_R = crate::BitReader<HSCMP_EN_A>;
impl HSCMP_EN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `HSCMP_EN` writer - High Speed Comparator enable"]
pub type HSCMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCMP_SPEC, HSCMP_EN_A, O>;
impl<'a, const O: u8> HSCMP_EN_W<'a, O> {
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
}
#[doc = "Field `HSCMP_INVERT` reader - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
pub type HSCMP_INVERT_R = crate::BitReader<bool>;
#[doc = "Field `HSCMP_INVERT` writer - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
pub type HSCMP_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCMP_SPEC, bool, O>;
#[doc = "Field `AZ_EN` reader - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AZ_EN_R = crate::BitReader<bool>;
#[doc = "Field `AZ_EN` writer - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AZ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCMP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    pub fn hscmp_en(&self) -> HSCMP_EN_R {
        HSCMP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn hscmp_invert(&self) -> HSCMP_INVERT_R {
        HSCMP_INVERT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&self) -> AZ_EN_R {
        AZ_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High Speed Comparator enable"]
    #[inline(always)]
    pub fn hscmp_en(&mut self) -> HSCMP_EN_W<0> {
        HSCMP_EN_W::new(self)
    }
    #[doc = "Bit 4 - Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn hscmp_invert(&mut self) -> HSCMP_INVERT_W<4> {
        HSCMP_INVERT_W::new(self)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&mut self) -> AZ_EN_W<31> {
        AZ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Speed Comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hscmp](index.html) module"]
pub struct HSCMP_SPEC;
impl crate::RegisterSpec for HSCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hscmp::R](R) reader structure"]
impl crate::Readable for HSCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hscmp::W](W) writer structure"]
impl crate::Writable for HSCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCMP to value 0"]
impl crate::Resettable for HSCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
