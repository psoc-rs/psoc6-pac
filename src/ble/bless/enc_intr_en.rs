#[doc = "Register `ENC_INTR_EN` reader"]
pub struct R(crate::R<ENC_INTR_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_INTR_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_INTR_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_INTR_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_INTR_EN` writer"]
pub struct W(crate::W<ENC_INTR_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_INTR_EN_SPEC>;
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
impl From<crate::W<ENC_INTR_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_INTR_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTH_PASS_INTR_EN` reader - Authentication interrupt enable 0 - Disable 1 - Enable"]
pub type AUTH_PASS_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTH_PASS_INTR_EN` writer - Authentication interrupt enable 0 - Disable 1 - Enable"]
pub type AUTH_PASS_INTR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENC_INTR_EN_SPEC, bool, O>;
#[doc = "Field `ECB_PROC_INTR_EN` reader - ECB processed interrupt enable 0 - Disable 1 - Enable"]
pub type ECB_PROC_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECB_PROC_INTR_EN` writer - ECB processed interrupt enable 0 - Disable 1 - Enable"]
pub type ECB_PROC_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_INTR_EN_SPEC, bool, O>;
#[doc = "Field `CCM_PROC_INTR_EN` reader - CCM processed interupt enable 0 - Disable 1 - Enable"]
pub type CCM_PROC_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCM_PROC_INTR_EN` writer - CCM processed interupt enable 0 - Disable 1 - Enable"]
pub type CCM_PROC_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_INTR_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Authentication interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn auth_pass_intr_en(&self) -> AUTH_PASS_INTR_EN_R {
        AUTH_PASS_INTR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECB processed interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ecb_proc_intr_en(&self) -> ECB_PROC_INTR_EN_R {
        ECB_PROC_INTR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCM processed interupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ccm_proc_intr_en(&self) -> CCM_PROC_INTR_EN_R {
        CCM_PROC_INTR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Authentication interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn auth_pass_intr_en(&mut self) -> AUTH_PASS_INTR_EN_W<0> {
        AUTH_PASS_INTR_EN_W::new(self)
    }
    #[doc = "Bit 1 - ECB processed interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ecb_proc_intr_en(&mut self) -> ECB_PROC_INTR_EN_W<1> {
        ECB_PROC_INTR_EN_W::new(self)
    }
    #[doc = "Bit 2 - CCM processed interupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ccm_proc_intr_en(&mut self) -> CCM_PROC_INTR_EN_W<2> {
        CCM_PROC_INTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_intr_en](index.html) module"]
pub struct ENC_INTR_EN_SPEC;
impl crate::RegisterSpec for ENC_INTR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_intr_en::R](R) reader structure"]
impl crate::Readable for ENC_INTR_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_intr_en::W](W) writer structure"]
impl crate::Writable for ENC_INTR_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_INTR_EN to value 0"]
impl crate::Resettable for ENC_INTR_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
