#[doc = "Reader of register CONN_REQ_WORD3"]
pub type R = crate::R<u32, super::CONN_REQ_WORD3>;
#[doc = "Writer for register CONN_REQ_WORD3"]
pub type W = crate::W<u32, super::CONN_REQ_WORD3>;
#[doc = "Register CONN_REQ_WORD3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC_INIT_UPPER`"]
pub type CRC_INIT_UPPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC_INIT_UPPER`"]
pub struct CRC_INIT_UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the upper byte \\[23:8\\] of the CRC initialization value that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn crc_init_upper(&self) -> CRC_INIT_UPPER_R {
        CRC_INIT_UPPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the upper byte \\[23:8\\] of the CRC initialization value that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn crc_init_upper(&mut self) -> CRC_INIT_UPPER_W {
        CRC_INIT_UPPER_W { w: self }
    }
}
