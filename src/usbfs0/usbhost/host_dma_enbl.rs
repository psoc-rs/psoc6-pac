#[doc = "Register `HOST_DMA_ENBL` reader"]
pub struct R(crate::R<HOST_DMA_ENBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_DMA_ENBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_DMA_ENBL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_DMA_ENBL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_DMA_ENBL` writer"]
pub struct W(crate::W<HOST_DMA_ENBL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_DMA_ENBL_SPEC>;
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
impl From<crate::W<HOST_DMA_ENBL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_DMA_ENBL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DM_EP1DRQE` reader - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP1DRQE_R = crate::BitReader<bool>;
#[doc = "Field `DM_EP1DRQE` writer - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP1DRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_DMA_ENBL_SPEC, bool, O>;
#[doc = "Field `DM_EP2DRQE` reader - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP2DRQE_R = crate::BitReader<bool>;
#[doc = "Field `DM_EP2DRQE` writer - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
pub type DM_EP2DRQE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_DMA_ENBL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep1drqe(&self) -> DM_EP1DRQE_R {
        DM_EP1DRQE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep2drqe(&self) -> DM_EP2DRQE_R {
        DM_EP2DRQE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep1drqe(&mut self) -> DM_EP1DRQE_W<2> {
        DM_EP1DRQE_W::new(self)
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep2drqe(&mut self) -> DM_EP2DRQE_W<3> {
        DM_EP2DRQE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_dma_enbl](index.html) module"]
pub struct HOST_DMA_ENBL_SPEC;
impl crate::RegisterSpec for HOST_DMA_ENBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_dma_enbl::R](R) reader structure"]
impl crate::Readable for HOST_DMA_ENBL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_dma_enbl::W](W) writer structure"]
impl crate::Writable for HOST_DMA_ENBL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_DMA_ENBL to value 0"]
impl crate::Resettable for HOST_DMA_ENBL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
