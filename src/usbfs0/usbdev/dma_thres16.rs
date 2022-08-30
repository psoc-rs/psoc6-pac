#[doc = "Register `DMA_THRES16` reader"]
pub struct R(crate::R<DMA_THRES16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_THRES16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_THRES16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_THRES16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_THRES16` writer"]
pub struct W(crate::W<DMA_THRES16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_THRES16_SPEC>;
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
impl From<crate::W<DMA_THRES16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_THRES16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_THS16` reader - DMA Threshold count"]
pub type DMA_THS16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMA_THS16` writer - DMA Threshold count"]
pub type DMA_THS16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_THRES16_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths16(&self) -> DMA_THS16_R {
        DMA_THS16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths16(&mut self) -> DMA_THS16_W<0> {
        DMA_THS16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Burst / Threshold Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_thres16](index.html) module"]
pub struct DMA_THRES16_SPEC;
impl crate::RegisterSpec for DMA_THRES16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_thres16::R](R) reader structure"]
impl crate::Readable for DMA_THRES16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_thres16::W](W) writer structure"]
impl crate::Writable for DMA_THRES16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_THRES16 to value 0"]
impl crate::Resettable for DMA_THRES16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
