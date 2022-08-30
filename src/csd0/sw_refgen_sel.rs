#[doc = "Register `SW_REFGEN_SEL` reader"]
pub struct R(crate::R<SW_REFGEN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_REFGEN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_REFGEN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_REFGEN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_REFGEN_SEL` writer"]
pub struct W(crate::W<SW_REFGEN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_REFGEN_SEL_SPEC>;
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
impl From<crate::W<SW_REFGEN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_REFGEN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_IAIB` reader - Set corresponding switch"]
pub type SW_IAIB_R = crate::BitReader<bool>;
#[doc = "Field `SW_IAIB` writer - Set corresponding switch"]
pub type SW_IAIB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_REFGEN_SEL_SPEC, bool, O>;
#[doc = "Field `SW_IBCB` reader - Set corresponding switch"]
pub type SW_IBCB_R = crate::BitReader<bool>;
#[doc = "Field `SW_IBCB` writer - Set corresponding switch"]
pub type SW_IBCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_REFGEN_SEL_SPEC, bool, O>;
#[doc = "Field `SW_SGMB` reader - Set corresponding switch"]
pub type SW_SGMB_R = crate::BitReader<bool>;
#[doc = "Field `SW_SGMB` writer - Set corresponding switch"]
pub type SW_SGMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_REFGEN_SEL_SPEC, bool, O>;
#[doc = "Field `SW_SGRP` reader - Set corresponding switch"]
pub type SW_SGRP_R = crate::BitReader<bool>;
#[doc = "Field `SW_SGRP` writer - Set corresponding switch"]
pub type SW_SGRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_REFGEN_SEL_SPEC, bool, O>;
#[doc = "Field `SW_SGRE` reader - Set corresponding switch"]
pub type SW_SGRE_R = crate::BitReader<bool>;
#[doc = "Field `SW_SGRE` writer - Set corresponding switch"]
pub type SW_SGRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_REFGEN_SEL_SPEC, bool, O>;
#[doc = "Field `SW_SGR` reader - Set corresponding switch"]
pub type SW_SGR_R = crate::BitReader<bool>;
#[doc = "Field `SW_SGR` writer - Set corresponding switch"]
pub type SW_SGR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_REFGEN_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&self) -> SW_IAIB_R {
        SW_IAIB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&self) -> SW_IBCB_R {
        SW_IBCB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&self) -> SW_SGMB_R {
        SW_SGMB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&self) -> SW_SGRP_R {
        SW_SGRP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&self) -> SW_SGRE_R {
        SW_SGRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&self) -> SW_SGR_R {
        SW_SGR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&mut self) -> SW_IAIB_W<0> {
        SW_IAIB_W::new(self)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&mut self) -> SW_IBCB_W<4> {
        SW_IBCB_W::new(self)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&mut self) -> SW_SGMB_W<16> {
        SW_SGMB_W::new(self)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&mut self) -> SW_SGRP_W<20> {
        SW_SGRP_W::new(self)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&mut self) -> SW_SGRE_W<24> {
        SW_SGRE_W::new(self)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&mut self) -> SW_SGR_W<28> {
        SW_SGR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Generator Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_refgen_sel](index.html) module"]
pub struct SW_REFGEN_SEL_SPEC;
impl crate::RegisterSpec for SW_REFGEN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_refgen_sel::R](R) reader structure"]
impl crate::Readable for SW_REFGEN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_refgen_sel::W](W) writer structure"]
impl crate::Writable for SW_REFGEN_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_REFGEN_SEL to value 0"]
impl crate::Resettable for SW_REFGEN_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
