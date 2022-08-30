#[doc = "Register `INTR_HOST_EP_MASK` reader"]
pub struct R(crate::R<INTR_HOST_EP_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_HOST_EP_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_HOST_EP_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_HOST_EP_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_HOST_EP_MASK` writer"]
pub struct W(crate::W<INTR_HOST_EP_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_HOST_EP_MASK_SPEC>;
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
impl From<crate::W<INTR_HOST_EP_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_HOST_EP_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1DRQM` reader - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
pub type EP1DRQM_R = crate::BitReader<bool>;
#[doc = "Field `EP1DRQM` writer - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
pub type EP1DRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_MASK_SPEC, bool, O>;
#[doc = "Field `EP1SPKM` reader - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
pub type EP1SPKM_R = crate::BitReader<bool>;
#[doc = "Field `EP1SPKM` writer - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
pub type EP1SPKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_MASK_SPEC, bool, O>;
#[doc = "Field `EP2DRQM` reader - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
pub type EP2DRQM_R = crate::BitReader<bool>;
#[doc = "Field `EP2DRQM` writer - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
pub type EP2DRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_MASK_SPEC, bool, O>;
#[doc = "Field `EP2SPKM` reader - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
pub type EP2SPKM_R = crate::BitReader<bool>;
#[doc = "Field `EP2SPKM` writer - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
pub type EP2SPKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1drqm(&self) -> EP1DRQM_R {
        EP1DRQM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1spkm(&self) -> EP1SPKM_R {
        EP1SPKM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2drqm(&self) -> EP2DRQM_R {
        EP2DRQM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2spkm(&self) -> EP2SPKM_R {
        EP2SPKM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1drqm(&mut self) -> EP1DRQM_W<2> {
        EP1DRQM_W::new(self)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1spkm(&mut self) -> EP1SPKM_W<3> {
        EP1SPKM_W::new(self)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2drqm(&mut self) -> EP2DRQM_W<4> {
        EP2DRQM_W::new(self)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2spkm(&mut self) -> EP2SPKM_W<5> {
        EP2SPKM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt USB Host Endpoint Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_mask](index.html) module"]
pub struct INTR_HOST_EP_MASK_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_host_ep_mask::R](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_host_ep_mask::W](W) writer structure"]
impl crate::Writable for INTR_HOST_EP_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_HOST_EP_MASK to value 0"]
impl crate::Resettable for INTR_HOST_EP_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
