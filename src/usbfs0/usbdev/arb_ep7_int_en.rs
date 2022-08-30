#[doc = "Register `ARB_EP7_INT_EN` reader"]
pub struct R(crate::R<ARB_EP7_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_EP7_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_EP7_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_EP7_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_EP7_INT_EN` writer"]
pub struct W(crate::W<ARB_EP7_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_EP7_INT_EN_SPEC>;
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
impl From<crate::W<ARB_EP7_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_EP7_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_BUF_FULL_EN` reader - IN Endpoint Local Buffer Full Enable"]
pub type IN_BUF_FULL_EN_R = crate::BitReader<bool>;
#[doc = "Field `IN_BUF_FULL_EN` writer - IN Endpoint Local Buffer Full Enable"]
pub type IN_BUF_FULL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ARB_EP7_INT_EN_SPEC, bool, O>;
#[doc = "Field `DMA_GNT_EN` reader - Endpoint DMA Grant Enable"]
pub type DMA_GNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_GNT_EN` writer - Endpoint DMA Grant Enable"]
pub type DMA_GNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ARB_EP7_INT_EN_SPEC, bool, O>;
#[doc = "Field `BUF_OVER_EN` reader - Endpoint Buffer Overflow Enable"]
pub type BUF_OVER_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUF_OVER_EN` writer - Endpoint Buffer Overflow Enable"]
pub type BUF_OVER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ARB_EP7_INT_EN_SPEC, bool, O>;
#[doc = "Field `BUF_UNDER_EN` reader - Endpoint Buffer Underflow Enable"]
pub type BUF_UNDER_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUF_UNDER_EN` writer - Endpoint Buffer Underflow Enable"]
pub type BUF_UNDER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ARB_EP7_INT_EN_SPEC, bool, O>;
#[doc = "Field `ERR_INT_EN` reader - Endpoint Error in Transaction Interrupt Enable"]
pub type ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ERR_INT_EN` writer - Endpoint Error in Transaction Interrupt Enable"]
pub type ERR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ARB_EP7_INT_EN_SPEC, bool, O>;
#[doc = "Field `DMA_TERMIN_EN` reader - Endpoint DMA Terminated Enable"]
pub type DMA_TERMIN_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TERMIN_EN` writer - Endpoint DMA Terminated Enable"]
pub type DMA_TERMIN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ARB_EP7_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn in_buf_full_en(&self) -> IN_BUF_FULL_EN_R {
        IN_BUF_FULL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn dma_gnt_en(&self) -> DMA_GNT_EN_R {
        DMA_GNT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn buf_over_en(&self) -> BUF_OVER_EN_R {
        BUF_OVER_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn buf_under_en(&self) -> BUF_UNDER_EN_R {
        BUF_UNDER_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn err_int_en(&self) -> ERR_INT_EN_R {
        ERR_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn dma_termin_en(&self) -> DMA_TERMIN_EN_R {
        DMA_TERMIN_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn in_buf_full_en(&mut self) -> IN_BUF_FULL_EN_W<0> {
        IN_BUF_FULL_EN_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn dma_gnt_en(&mut self) -> DMA_GNT_EN_W<1> {
        DMA_GNT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn buf_over_en(&mut self) -> BUF_OVER_EN_W<2> {
        BUF_OVER_EN_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn buf_under_en(&mut self) -> BUF_UNDER_EN_W<3> {
        BUF_UNDER_EN_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn err_int_en(&mut self) -> ERR_INT_EN_W<4> {
        ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn dma_termin_en(&mut self) -> DMA_TERMIN_EN_W<5> {
        DMA_TERMIN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Interrupt Enable Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_ep7_int_en](index.html) module"]
pub struct ARB_EP7_INT_EN_SPEC;
impl crate::RegisterSpec for ARB_EP7_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_ep7_int_en::R](R) reader structure"]
impl crate::Readable for ARB_EP7_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_ep7_int_en::W](W) writer structure"]
impl crate::Writable for ARB_EP7_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_EP7_INT_EN to value 0"]
impl crate::Resettable for ARB_EP7_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
