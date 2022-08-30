#[doc = "Register `SW_CMP_P_SEL` reader"]
pub struct R(crate::R<SW_CMP_P_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_CMP_P_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_CMP_P_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_CMP_P_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_CMP_P_SEL` writer"]
pub struct W(crate::W<SW_CMP_P_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_CMP_P_SEL_SPEC>;
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
impl From<crate::W<SW_CMP_P_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_CMP_P_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_SFPM` reader - Select waveform for corresponding switch"]
pub type SW_SFPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_SFPM` writer - Select waveform for corresponding switch"]
pub type SW_SFPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_CMP_P_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_SFPT` reader - Select waveform for corresponding switch"]
pub type SW_SFPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_SFPT` writer - Select waveform for corresponding switch"]
pub type SW_SFPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_CMP_P_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_SFPS` reader - Select waveform for corresponding switch"]
pub type SW_SFPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_SFPS` writer - Select waveform for corresponding switch"]
pub type SW_SFPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_CMP_P_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_SFMA` reader - Set corresponding switch"]
pub type SW_SFMA_R = crate::BitReader<bool>;
#[doc = "Field `SW_SFMA` writer - Set corresponding switch"]
pub type SW_SFMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_CMP_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_SFMB` reader - Set corresponding switch"]
pub type SW_SFMB_R = crate::BitReader<bool>;
#[doc = "Field `SW_SFMB` writer - Set corresponding switch"]
pub type SW_SFMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_CMP_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_SFCA` reader - Set corresponding switch"]
pub type SW_SFCA_R = crate::BitReader<bool>;
#[doc = "Field `SW_SFCA` writer - Set corresponding switch"]
pub type SW_SFCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_CMP_P_SEL_SPEC, bool, O>;
#[doc = "Field `SW_SFCB` reader - Set corresponding switch"]
pub type SW_SFCB_R = crate::BitReader<bool>;
#[doc = "Field `SW_SFCB` writer - Set corresponding switch"]
pub type SW_SFCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_CMP_P_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&self) -> SW_SFPM_R {
        SW_SFPM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&self) -> SW_SFPT_R {
        SW_SFPT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&self) -> SW_SFPS_R {
        SW_SFPS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&self) -> SW_SFMA_R {
        SW_SFMA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&self) -> SW_SFMB_R {
        SW_SFMB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&self) -> SW_SFCA_R {
        SW_SFCA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&self) -> SW_SFCB_R {
        SW_SFCB_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&mut self) -> SW_SFPM_W<0> {
        SW_SFPM_W::new(self)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&mut self) -> SW_SFPT_W<4> {
        SW_SFPT_W::new(self)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&mut self) -> SW_SFPS_W<8> {
        SW_SFPS_W::new(self)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&mut self) -> SW_SFMA_W<12> {
        SW_SFMA_W::new(self)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&mut self) -> SW_SFMB_W<16> {
        SW_SFMB_W::new(self)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&mut self) -> SW_SFCA_W<20> {
        SW_SFCA_W::new(self)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&mut self) -> SW_SFCB_W<24> {
        SW_SFCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSDCMP Pos Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cmp_p_sel](index.html) module"]
pub struct SW_CMP_P_SEL_SPEC;
impl crate::RegisterSpec for SW_CMP_P_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_cmp_p_sel::R](R) reader structure"]
impl crate::Readable for SW_CMP_P_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_cmp_p_sel::W](W) writer structure"]
impl crate::Writable for SW_CMP_P_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_CMP_P_SEL to value 0"]
impl crate::Resettable for SW_CMP_P_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
