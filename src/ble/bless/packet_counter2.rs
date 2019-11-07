#[doc = "Reader of register PACKET_COUNTER2"]
pub type R = crate::R<u32, super::PACKET_COUNTER2>;
#[doc = "Writer for register PACKET_COUNTER2"]
pub type W = crate::W<u32, super::PACKET_COUNTER2>;
#[doc = "Register PACKET_COUNTER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PACKET_COUNTER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PACKET_COUNTER_UPPER`"]
pub type PACKET_COUNTER_UPPER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACKET_COUNTER_UPPER`"]
pub struct PACKET_COUNTER_UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_COUNTER_UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Upper 8 bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
    #[inline(always)]
    pub fn packet_counter_upper(&self) -> PACKET_COUNTER_UPPER_R {
        PACKET_COUNTER_UPPER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Upper 8 bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
    #[inline(always)]
    pub fn packet_counter_upper(&mut self) -> PACKET_COUNTER_UPPER_W {
        PACKET_COUNTER_UPPER_W { w: self }
    }
}
