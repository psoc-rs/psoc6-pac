#[doc = "Reader of register TX_DATA_FIFO_CTL"]
pub type R = crate::R<u32, super::TX_DATA_FIFO_CTL>;
#[doc = "Writer for register TX_DATA_FIFO_CTL"]
pub type W = crate::W<u32, super::TX_DATA_FIFO_CTL>;
#[doc = "Register TX_DATA_FIFO_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_DATA_FIFO_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIGGER_LEVEL`"]
pub type TRIGGER_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIGGER_LEVEL`"]
pub struct TRIGGER_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED <= TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED <= TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W {
        TRIGGER_LEVEL_W { w: self }
    }
}
