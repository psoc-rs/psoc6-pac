#[doc = "Reader of register CONN_REQ_WORD1"]
pub type R = crate::R<u32, super::CONN_REQ_WORD1>;
#[doc = "Writer for register CONN_REQ_WORD1"]
pub type W = crate::W<u32, super::CONN_REQ_WORD1>;
#[doc = "Register CONN_REQ_WORD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACCESS_ADDR_UPPER`"]
pub type ACCESS_ADDR_UPPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ACCESS_ADDR_UPPER`"]
pub struct ACCESS_ADDR_UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCESS_ADDR_UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the upper16 bits of the access address that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn access_addr_upper(&self) -> ACCESS_ADDR_UPPER_R {
        ACCESS_ADDR_UPPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the upper16 bits of the access address that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn access_addr_upper(&mut self) -> ACCESS_ADDR_UPPER_W {
        ACCESS_ADDR_UPPER_W { w: self }
    }
}
