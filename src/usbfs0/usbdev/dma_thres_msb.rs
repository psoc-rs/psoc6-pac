#[doc = "Reader of register DMA_THRES_MSB"]
pub type R = crate::R<u32, super::DMA_THRES_MSB>;
#[doc = "Writer for register DMA_THRES_MSB"]
pub type W = crate::W<u32, super::DMA_THRES_MSB>;
#[doc = "Register DMA_THRES_MSB `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_THRES_MSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_THS_MSB`"]
pub type DMA_THS_MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_THS_MSB`"]
pub struct DMA_THS_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_THS_MSB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths_msb(&self) -> DMA_THS_MSB_R {
        DMA_THS_MSB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths_msb(&mut self) -> DMA_THS_MSB_W {
        DMA_THS_MSB_W { w: self }
    }
}
