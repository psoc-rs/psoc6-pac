#[doc = "Register `SW_AMUXBUF_SEL` reader"]
pub struct R(crate::R<SW_AMUXBUF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_AMUXBUF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_AMUXBUF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_AMUXBUF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_AMUXBUF_SEL` writer"]
pub struct W(crate::W<SW_AMUXBUF_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_AMUXBUF_SEL_SPEC>;
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
impl From<crate::W<SW_AMUXBUF_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_AMUXBUF_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_IRBY` reader - Set corresponding switch"]
pub type SW_IRBY_R = crate::BitReader<bool>;
#[doc = "Field `SW_IRBY` writer - Set corresponding switch"]
pub type SW_IRBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_AMUXBUF_SEL_SPEC, bool, O>;
#[doc = "Field `SW_IRLB` reader - Set corresponding switch"]
pub type SW_IRLB_R = crate::BitReader<bool>;
#[doc = "Field `SW_IRLB` writer - Set corresponding switch"]
pub type SW_IRLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_AMUXBUF_SEL_SPEC, bool, O>;
#[doc = "Field `SW_ICA` reader - Set corresponding switch"]
pub type SW_ICA_R = crate::BitReader<bool>;
#[doc = "Field `SW_ICA` writer - Set corresponding switch"]
pub type SW_ICA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_AMUXBUF_SEL_SPEC, bool, O>;
#[doc = "Field `SW_ICB` reader - Select waveform for corresponding switch"]
pub type SW_ICB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_ICB` writer - Select waveform for corresponding switch"]
pub type SW_ICB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_AMUXBUF_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_IRLI` reader - Set corresponding switch"]
pub type SW_IRLI_R = crate::BitReader<bool>;
#[doc = "Field `SW_IRLI` writer - Set corresponding switch"]
pub type SW_IRLI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_AMUXBUF_SEL_SPEC, bool, O>;
#[doc = "Field `SW_IRH` reader - Set corresponding switch"]
pub type SW_IRH_R = crate::BitReader<bool>;
#[doc = "Field `SW_IRH` writer - Set corresponding switch"]
pub type SW_IRH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_AMUXBUF_SEL_SPEC, bool, O>;
#[doc = "Field `SW_IRL` reader - Set corresponding switch"]
pub type SW_IRL_R = crate::BitReader<bool>;
#[doc = "Field `SW_IRL` writer - Set corresponding switch"]
pub type SW_IRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_AMUXBUF_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&self) -> SW_IRBY_R {
        SW_IRBY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&self) -> SW_IRLB_R {
        SW_IRLB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&self) -> SW_ICA_R {
        SW_ICA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&self) -> SW_ICB_R {
        SW_ICB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&self) -> SW_IRLI_R {
        SW_IRLI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&self) -> SW_IRH_R {
        SW_IRH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&self) -> SW_IRL_R {
        SW_IRL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&mut self) -> SW_IRBY_W<4> {
        SW_IRBY_W::new(self)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&mut self) -> SW_IRLB_W<8> {
        SW_IRLB_W::new(self)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&mut self) -> SW_ICA_W<12> {
        SW_ICA_W::new(self)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&mut self) -> SW_ICB_W<16> {
        SW_ICB_W::new(self)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&mut self) -> SW_IRLI_W<20> {
        SW_IRLI_W::new(self)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&mut self) -> SW_IRH_W<24> {
        SW_IRH_W::new(self)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&mut self) -> SW_IRL_W<28> {
        SW_IRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amuxbuffer switches Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_amuxbuf_sel](index.html) module"]
pub struct SW_AMUXBUF_SEL_SPEC;
impl crate::RegisterSpec for SW_AMUXBUF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_amuxbuf_sel::R](R) reader structure"]
impl crate::Readable for SW_AMUXBUF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_amuxbuf_sel::W](W) writer structure"]
impl crate::Writable for SW_AMUXBUF_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_AMUXBUF_SEL to value 0"]
impl crate::Resettable for SW_AMUXBUF_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
