#[doc = "Reader of register PEER_ADDR_M"]
pub type R = crate::R<u32, super::PEER_ADDR_M>;
#[doc = "Writer for register PEER_ADDR_M"]
pub type W = crate::W<u32, super::PEER_ADDR_M>;
#[doc = "Register PEER_ADDR_M `reset()`'s with value 0"]
impl crate::ResetValue for super::PEER_ADDR_M {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEER_ADDR_M`"]
pub type PEER_ADDR_M_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PEER_ADDR_M`"]
pub struct PEER_ADDR_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Middle 16 bit of 48-bit address of the peer device."]
    #[inline(always)]
    pub fn peer_addr_m(&self) -> PEER_ADDR_M_R {
        PEER_ADDR_M_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Middle 16 bit of 48-bit address of the peer device."]
    #[inline(always)]
    pub fn peer_addr_m(&mut self) -> PEER_ADDR_M_W {
        PEER_ADDR_M_W { w: self }
    }
}
