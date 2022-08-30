#[doc = "Register `SW_CMP_N_SEL` reader"]
pub struct R(crate::R<SW_CMP_N_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_CMP_N_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_CMP_N_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_CMP_N_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_CMP_N_SEL` writer"]
pub struct W(crate::W<SW_CMP_N_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_CMP_N_SEL_SPEC>;
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
impl From<crate::W<SW_CMP_N_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_CMP_N_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_SCRH` reader - Select waveform for corresponding switch"]
pub type SW_SCRH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_SCRH` writer - Select waveform for corresponding switch"]
pub type SW_SCRH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_CMP_N_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_SCRL` reader - Select waveform for corresponding switch"]
pub type SW_SCRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_SCRL` writer - Select waveform for corresponding switch"]
pub type SW_SCRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_CMP_N_SEL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrh(&self) -> SW_SCRH_R {
        SW_SCRH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrl(&self) -> SW_SCRL_R {
        SW_SCRL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrh(&mut self) -> SW_SCRH_W<24> {
        SW_SCRH_W::new(self)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrl(&mut self) -> SW_SCRL_W<28> {
        SW_SCRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSDCMP Neg Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cmp_n_sel](index.html) module"]
pub struct SW_CMP_N_SEL_SPEC;
impl crate::RegisterSpec for SW_CMP_N_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_cmp_n_sel::R](R) reader structure"]
impl crate::Readable for SW_CMP_N_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_cmp_n_sel::W](W) writer structure"]
impl crate::Writable for SW_CMP_N_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_CMP_N_SEL to value 0"]
impl crate::Resettable for SW_CMP_N_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
