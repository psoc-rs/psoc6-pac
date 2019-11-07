#[doc = "Reader of register HOST_EP1_BLK"]
pub type R = crate::R<u32, super::HOST_EP1_BLK>;
#[doc = "Writer for register HOST_EP1_BLK"]
pub type W = crate::W<u32, super::HOST_EP1_BLK>;
#[doc = "Register HOST_EP1_BLK `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_EP1_BLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK_NUM`"]
pub type BLK_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLK_NUM`"]
pub struct BLK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decrement when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    pub fn blk_num(&self) -> BLK_NUM_R {
        BLK_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decrement when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    pub fn blk_num(&mut self) -> BLK_NUM_W {
        BLK_NUM_W { w: self }
    }
}
