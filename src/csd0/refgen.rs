#[doc = "Register `REFGEN` reader"]
pub struct R(crate::R<REFGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFGEN` writer"]
pub struct W(crate::W<REFGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFGEN_SPEC>;
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
impl From<crate::W<REFGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFGEN_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `REFGEN_EN` reader - Reference Generator Enable"]
pub type REFGEN_EN_R = crate::BitReader<REFGEN_EN_A>;
impl REFGEN_EN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `REFGEN_EN` writer - Reference Generator Enable"]
pub type REFGEN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFGEN_SPEC, REFGEN_EN_A, O>;
impl<'a, const O: u8> REFGEN_EN_W<'a, O> {
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
}
#[doc = "Field `BYPASS` reader - Bypass selected input reference unbuffered to Vrefhi"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Bypass selected input reference unbuffered to Vrefhi"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFGEN_SPEC, bool, O>;
#[doc = "Field `VDDA_EN` reader - Close Vdda switch to top of resistor string (or Vrefhi?)"]
pub type VDDA_EN_R = crate::BitReader<bool>;
#[doc = "Field `VDDA_EN` writer - Close Vdda switch to top of resistor string (or Vrefhi?)"]
pub type VDDA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFGEN_SPEC, bool, O>;
#[doc = "Field `RES_EN` reader - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
pub type RES_EN_R = crate::BitReader<bool>;
#[doc = "Field `RES_EN` writer - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
pub type RES_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFGEN_SPEC, bool, O>;
#[doc = "Field `GAIN` reader - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REFGEN_SPEC, u8, u8, 5, O>;
#[doc = "Field `VREFLO_SEL` reader - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
pub type VREFLO_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFLO_SEL` writer - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
pub type VREFLO_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REFGEN_SPEC, u8, u8, 5, O>;
#[doc = "Field `VREFLO_INT` reader - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
pub type VREFLO_INT_R = crate::BitReader<bool>;
#[doc = "Field `VREFLO_INT` writer - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
pub type VREFLO_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFGEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    pub fn refgen_en(&self) -> REFGEN_EN_R {
        REFGEN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn vdda_en(&self) -> VDDA_EN_R {
        VDDA_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn res_en(&self) -> RES_EN_R {
        RES_EN_R::new(((self.bits >> 6) & 1) != 0)
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
        VREFLO_INT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Generator Enable"]
    #[inline(always)]
    pub fn refgen_en(&mut self) -> REFGEN_EN_W<0> {
        REFGEN_EN_W::new(self)
    }
    #[doc = "Bit 4 - Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<4> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 5 - Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn vdda_en(&mut self) -> VDDA_EN_W<5> {
        VDDA_EN_W::new(self)
    }
    #[doc = "Bit 6 - Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn res_en(&mut self) -> RES_EN_W<6> {
        RES_EN_W::new(self)
    }
    #[doc = "Bits 8:12 - Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W<8> {
        GAIN_W::new(self)
    }
    #[doc = "Bits 16:20 - Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn vreflo_sel(&mut self) -> VREFLO_SEL_W<16> {
        VREFLO_SEL_W::new(self)
    }
    #[doc = "Bit 23 - Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub fn vreflo_int(&mut self) -> VREFLO_INT_W<23> {
        VREFLO_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Generator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refgen](index.html) module"]
pub struct REFGEN_SPEC;
impl crate::RegisterSpec for REFGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refgen::R](R) reader structure"]
impl crate::Readable for REFGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refgen::W](W) writer structure"]
impl crate::Writable for REFGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFGEN to value 0"]
impl crate::Resettable for REFGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
