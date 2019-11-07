#[doc = "Reader of register HOST_DMA_ENBL"]
pub type R = crate::R<u32, super::HOST_DMA_ENBL>;
#[doc = "Writer for register HOST_DMA_ENBL"]
pub type W = crate::W<u32, super::HOST_DMA_ENBL>;
#[doc = "Register HOST_DMA_ENBL `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_DMA_ENBL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DM_EP1DRQE`"]
pub type DM_EP1DRQE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_EP1DRQE`"]
pub struct DM_EP1DRQE_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_EP1DRQE_W<'a> {
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
#[doc = "Reader of field `DM_EP2DRQE`"]
pub type DM_EP2DRQE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_EP2DRQE`"]
pub struct DM_EP2DRQE_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_EP2DRQE_W<'a> {
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
impl R {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep1drqe(&self) -> DM_EP1DRQE_R {
        DM_EP1DRQE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep2drqe(&self) -> DM_EP2DRQE_R {
        DM_EP2DRQE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep1drqe(&mut self) -> DM_EP1DRQE_W {
        DM_EP1DRQE_W { w: self }
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep2drqe(&mut self) -> DM_EP2DRQE_W {
        DM_EP2DRQE_W { w: self }
    }
}
