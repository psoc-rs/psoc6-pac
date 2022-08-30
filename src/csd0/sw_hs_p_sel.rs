#[doc = "Register `SW_HS_P_SEL` reader"]
pub struct R(crate::R<SW_HS_P_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_HS_P_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_HS_P_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_HS_P_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_HS_P_SEL` writer"]
pub struct W(crate::W<SW_HS_P_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_HS_P_SEL_SPEC>;
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
impl From<crate::W<SW_HS_P_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_HS_P_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_HMPM` reader - Set HMPM switch 0: static open 1: static closed"]
pub type SW_HMPM_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMPM` writer - Set HMPM switch 0: static open 1: static closed"]
pub type SW_HMPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HMPT` reader - Set corresponding switch"]
pub type SW_HMPT_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMPT` writer - Set corresponding switch"]
pub type SW_HMPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HMPS` reader - Set corresponding switch"]
pub type SW_HMPS_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMPS` writer - Set corresponding switch"]
pub type SW_HMPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HMMA` reader - Set corresponding switch"]
pub type SW_HMMA_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMMA` writer - Set corresponding switch"]
pub type SW_HMMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HMMB` reader - Set corresponding switch"]
pub type SW_HMMB_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMMB` writer - Set corresponding switch"]
pub type SW_HMMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HMCA` reader - Set corresponding switch"]
pub type SW_HMCA_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMCA` writer - Set corresponding switch"]
pub type SW_HMCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HMCB` reader - Set corresponding switch"]
pub type SW_HMCB_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMCB` writer - Set corresponding switch"]
pub type SW_HMCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HMRH` reader - Set corresponding switch"]
pub type SW_HMRH_R = crate::BitReader<bool>;
#[doc = "Field `SW_HMRH` writer - Set corresponding switch"]
pub type SW_HMRH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_P_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn sw_hmpm(&self) -> SW_HMPM_R {
        SW_HMPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmpt(&self) -> SW_HMPT_R {
        SW_HMPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmps(&self) -> SW_HMPS_R {
        SW_HMPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmma(&self) -> SW_HMMA_R {
        SW_HMMA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmmb(&self) -> SW_HMMB_R {
        SW_HMMB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmca(&self) -> SW_HMCA_R {
        SW_HMCA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmcb(&self) -> SW_HMCB_R {
        SW_HMCB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmrh(&self) -> SW_HMRH_R {
        SW_HMRH_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn sw_hmpm(&mut self) -> SW_HMPM_W<0> {
        SW_HMPM_W::new(self)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmpt(&mut self) -> SW_HMPT_W<4> {
        SW_HMPT_W::new(self)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmps(&mut self) -> SW_HMPS_W<8> {
        SW_HMPS_W::new(self)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmma(&mut self) -> SW_HMMA_W<12> {
        SW_HMMA_W::new(self)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmmb(&mut self) -> SW_HMMB_W<16> {
        SW_HMMB_W::new(self)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmca(&mut self) -> SW_HMCA_W<20> {
        SW_HMCA_W::new(self)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmcb(&mut self) -> SW_HMCB_W<24> {
        SW_HMCB_W::new(self)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmrh(&mut self) -> SW_HMRH_W<28> {
        SW_HMRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSCMP Pos input switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_hs_p_sel](index.html) module"]
pub struct SW_HS_P_SEL_SPEC;
impl crate::RegisterSpec for SW_HS_P_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_hs_p_sel::R](R) reader structure"]
impl crate::Readable for SW_HS_P_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_hs_p_sel::W](W) writer structure"]
impl crate::Writable for SW_HS_P_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_HS_P_SEL to value 0"]
impl crate::Resettable for SW_HS_P_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
