#[doc = "Reader of register TRANSMIT_WINDOW_SIZE"]
pub type R = crate::R<u32, super::TRANSMIT_WINDOW_SIZE>;
#[doc = "Writer for register TRANSMIT_WINDOW_SIZE"]
pub type W = crate::W<u32, super::TRANSMIT_WINDOW_SIZE>;
#[doc = "Register TRANSMIT_WINDOW_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::TRANSMIT_WINDOW_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_WINDOW_SIZE`"]
pub type TX_WINDOW_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_WINDOW_SIZE`"]
pub struct TX_WINDOW_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WINDOW_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size(&self) -> TX_WINDOW_SIZE_R {
        TX_WINDOW_SIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size(&mut self) -> TX_WINDOW_SIZE_W {
        TX_WINDOW_SIZE_W { w: self }
    }
}
