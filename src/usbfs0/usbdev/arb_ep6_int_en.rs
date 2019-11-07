#[doc = "Reader of register ARB_EP6_INT_EN"]
pub type R = crate::R<u32, super::ARB_EP6_INT_EN>;
#[doc = "Writer for register ARB_EP6_INT_EN"]
pub type W = crate::W<u32, super::ARB_EP6_INT_EN>;
#[doc = "Register ARB_EP6_INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_EP6_INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN_BUF_FULL_EN`"]
pub type IN_BUF_FULL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_BUF_FULL_EN`"]
pub struct IN_BUF_FULL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_BUF_FULL_EN_W<'a> {
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
#[doc = "Reader of field `DMA_GNT_EN`"]
pub type DMA_GNT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_GNT_EN`"]
pub struct DMA_GNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_GNT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BUF_OVER_EN`"]
pub type BUF_OVER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUF_OVER_EN`"]
pub struct BUF_OVER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BUF_UNDER_EN`"]
pub type BUF_UNDER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUF_UNDER_EN`"]
pub struct BUF_UNDER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_UNDER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ERR_INT_EN`"]
pub type ERR_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_INT_EN`"]
pub struct ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMA_TERMIN_EN`"]
pub type DMA_TERMIN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_TERMIN_EN`"]
pub struct DMA_TERMIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TERMIN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn in_buf_full_en(&self) -> IN_BUF_FULL_EN_R {
        IN_BUF_FULL_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn dma_gnt_en(&self) -> DMA_GNT_EN_R {
        DMA_GNT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn buf_over_en(&self) -> BUF_OVER_EN_R {
        BUF_OVER_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn buf_under_en(&self) -> BUF_UNDER_EN_R {
        BUF_UNDER_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn err_int_en(&self) -> ERR_INT_EN_R {
        ERR_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn dma_termin_en(&self) -> DMA_TERMIN_EN_R {
        DMA_TERMIN_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn in_buf_full_en(&mut self) -> IN_BUF_FULL_EN_W {
        IN_BUF_FULL_EN_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn dma_gnt_en(&mut self) -> DMA_GNT_EN_W {
        DMA_GNT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn buf_over_en(&mut self) -> BUF_OVER_EN_W {
        BUF_OVER_EN_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn buf_under_en(&mut self) -> BUF_UNDER_EN_W {
        BUF_UNDER_EN_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn err_int_en(&mut self) -> ERR_INT_EN_W {
        ERR_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn dma_termin_en(&mut self) -> DMA_TERMIN_EN_W {
        DMA_TERMIN_EN_W { w: self }
    }
}
