#[doc = "Register `HVLDO_CTRL` reader"]
pub struct R(crate::R<HVLDO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HVLDO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HVLDO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HVLDO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HVLDO_CTRL` writer"]
pub struct W(crate::W<HVLDO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HVLDO_CTRL_SPEC>;
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
impl From<crate::W<HVLDO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HVLDO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADFT_EN` reader - ADFT enable"]
pub type ADFT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADFT_EN` writer - ADFT enable"]
pub type ADFT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HVLDO_CTRL_SPEC, bool, O>;
#[doc = "Field `ADFT_CTRL` reader - ADFT select"]
pub type ADFT_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADFT_CTRL` writer - ADFT select"]
pub type ADFT_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HVLDO_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `VREF_EXT_EN` reader - Vref ext input enable."]
pub type VREF_EXT_EN_R = crate::BitReader<bool>;
#[doc = "Field `VREF_EXT_EN` writer - Vref ext input enable."]
pub type VREF_EXT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HVLDO_CTRL_SPEC, bool, O>;
#[doc = "Field `STATUS` reader - hvldo LV detect status"]
pub type STATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ADFT enable"]
    #[inline(always)]
    pub fn adft_en(&self) -> ADFT_EN_R {
        ADFT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - ADFT select"]
    #[inline(always)]
    pub fn adft_ctrl(&self) -> ADFT_CTRL_R {
        ADFT_CTRL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Vref ext input enable."]
    #[inline(always)]
    pub fn vref_ext_en(&self) -> VREF_EXT_EN_R {
        VREF_EXT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - hvldo LV detect status"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADFT enable"]
    #[inline(always)]
    pub fn adft_en(&mut self) -> ADFT_EN_W<0> {
        ADFT_EN_W::new(self)
    }
    #[doc = "Bits 1:4 - ADFT select"]
    #[inline(always)]
    pub fn adft_ctrl(&mut self) -> ADFT_CTRL_W<1> {
        ADFT_CTRL_W::new(self)
    }
    #[doc = "Bit 6 - Vref ext input enable."]
    #[inline(always)]
    pub fn vref_ext_en(&mut self) -> VREF_EXT_EN_W<6> {
        VREF_EXT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HVLDO Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hvldo_ctrl](index.html) module"]
pub struct HVLDO_CTRL_SPEC;
impl crate::RegisterSpec for HVLDO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hvldo_ctrl::R](R) reader structure"]
impl crate::Readable for HVLDO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hvldo_ctrl::W](W) writer structure"]
impl crate::Writable for HVLDO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HVLDO_CTRL to value 0"]
impl crate::Resettable for HVLDO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
