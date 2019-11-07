#[doc = "Reader of register DMA_THRES"]
pub type R = crate::R<u32, super::DMA_THRES>;
#[doc = "Writer for register DMA_THRES"]
pub type W = crate::W<u32, super::DMA_THRES>;
#[doc = "Register DMA_THRES `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_THRES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_THS`"]
pub type DMA_THS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_THS`"]
pub struct DMA_THS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_THS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths(&self) -> DMA_THS_R {
        DMA_THS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths(&mut self) -> DMA_THS_W {
        DMA_THS_W { w: self }
    }
}
