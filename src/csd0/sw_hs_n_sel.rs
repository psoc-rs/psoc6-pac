#[doc = "Register `SW_HS_N_SEL` reader"]
pub struct R(crate::R<SW_HS_N_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_HS_N_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_HS_N_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_HS_N_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_HS_N_SEL` writer"]
pub struct W(crate::W<SW_HS_N_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_HS_N_SEL_SPEC>;
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
impl From<crate::W<SW_HS_N_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_HS_N_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_HCCC` reader - Set corresponding switch"]
pub type SW_HCCC_R = crate::BitReader<bool>;
#[doc = "Field `SW_HCCC` writer - Set corresponding switch"]
pub type SW_HCCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_N_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HCCD` reader - Set corresponding switch"]
pub type SW_HCCD_R = crate::BitReader<bool>;
#[doc = "Field `SW_HCCD` writer - Set corresponding switch"]
pub type SW_HCCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_HS_N_SEL_SPEC, bool, O>;
#[doc = "Field `SW_HCRH` reader - Select waveform for corresponding switch"]
pub type SW_HCRH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_HCRH` writer - Select waveform for corresponding switch"]
pub type SW_HCRH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_HS_N_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_HCRL` reader - Select waveform for corresponding switch"]
pub type SW_HCRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_HCRL` writer - Select waveform for corresponding switch"]
pub type SW_HCRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_HS_N_SEL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&self) -> SW_HCCC_R {
        SW_HCCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&self) -> SW_HCCD_R {
        SW_HCCD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&self) -> SW_HCRH_R {
        SW_HCRH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&self) -> SW_HCRL_R {
        SW_HCRL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&mut self) -> SW_HCCC_W<16> {
        SW_HCCC_W::new(self)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&mut self) -> SW_HCCD_W<20> {
        SW_HCCD_W::new(self)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&mut self) -> SW_HCRH_W<24> {
        SW_HCRH_W::new(self)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&mut self) -> SW_HCRL_W<28> {
        SW_HCRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSCMP Neg input switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_hs_n_sel](index.html) module"]
pub struct SW_HS_N_SEL_SPEC;
impl crate::RegisterSpec for SW_HS_N_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_hs_n_sel::R](R) reader structure"]
impl crate::Readable for SW_HS_N_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_hs_n_sel::W](W) writer structure"]
impl crate::Writable for SW_HS_N_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_HS_N_SEL to value 0"]
impl crate::Resettable for SW_HS_N_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
