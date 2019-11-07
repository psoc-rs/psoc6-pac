#[doc = "Reader of register RSLV_LIST_PEER_IDNTT_BASE_ADDR"]
pub type R = crate::R<u32, super::RSLV_LIST_PEER_IDNTT_BASE_ADDR>;
#[doc = "Writer for register RSLV_LIST_PEER_IDNTT_BASE_ADDR"]
pub type W = crate::W<u32, super::RSLV_LIST_PEER_IDNTT_BASE_ADDR>;
#[doc = "Register RSLV_LIST_PEER_IDNTT_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSLV_LIST_PEER_IDNTT_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSLV_LIST_PEER_IDNTT_BASE_ADDR`"]
pub type RSLV_LIST_PEER_IDNTT_BASE_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSLV_LIST_PEER_IDNTT_BASE_ADDR`"]
pub struct RSLV_LIST_PEER_IDNTT_BASE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSLV_LIST_PEER_IDNTT_BASE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Device address values written to the list are written as 16-bit wide address."]
    #[inline(always)]
    pub fn rslv_list_peer_idntt_base_addr(&self) -> RSLV_LIST_PEER_IDNTT_BASE_ADDR_R {
        RSLV_LIST_PEER_IDNTT_BASE_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address values written to the list are written as 16-bit wide address."]
    #[inline(always)]
    pub fn rslv_list_peer_idntt_base_addr(&mut self) -> RSLV_LIST_PEER_IDNTT_BASE_ADDR_W {
        RSLV_LIST_PEER_IDNTT_BASE_ADDR_W { w: self }
    }
}
