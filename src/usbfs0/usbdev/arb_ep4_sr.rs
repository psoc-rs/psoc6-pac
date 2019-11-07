#[doc = "Reader of register ARB_EP4_SR"]
pub type R = crate::R<u32, super::ARB_EP4_SR>;
#[doc = "Writer for register ARB_EP4_SR"]
pub type W = crate::W<u32, super::ARB_EP4_SR>;
#[doc = "Register ARB_EP4_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_EP4_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN_BUF_FULL`"]
pub type IN_BUF_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_BUF_FULL`"]
pub struct IN_BUF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_BUF_FULL_W<'a> {
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
#[doc = "Reader of field `DMA_GNT`"]
pub type DMA_GNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_GNT`"]
pub struct DMA_GNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_GNT_W<'a> {
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
#[doc = "Reader of field `BUF_OVER`"]
pub type BUF_OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUF_OVER`"]
pub struct BUF_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVER_W<'a> {
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
#[doc = "Reader of field `BUF_UNDER`"]
pub type BUF_UNDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUF_UNDER`"]
pub struct BUF_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_UNDER_W<'a> {
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
#[doc = "Reader of field `DMA_TERMIN`"]
pub type DMA_TERMIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_TERMIN`"]
pub struct DMA_TERMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TERMIN_W<'a> {
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
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn in_buf_full(&self) -> IN_BUF_FULL_R {
        IN_BUF_FULL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn dma_gnt(&self) -> DMA_GNT_R {
        DMA_GNT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn buf_over(&self) -> BUF_OVER_R {
        BUF_OVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn buf_under(&self) -> BUF_UNDER_R {
        BUF_UNDER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn dma_termin(&self) -> DMA_TERMIN_R {
        DMA_TERMIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn in_buf_full(&mut self) -> IN_BUF_FULL_W {
        IN_BUF_FULL_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn dma_gnt(&mut self) -> DMA_GNT_W {
        DMA_GNT_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn buf_over(&mut self) -> BUF_OVER_W {
        BUF_OVER_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn buf_under(&mut self) -> BUF_UNDER_W {
        BUF_UNDER_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn dma_termin(&mut self) -> DMA_TERMIN_W {
        DMA_TERMIN_W { w: self }
    }
}
