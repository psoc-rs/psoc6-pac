#[doc = "Reader of register PEER_SEC_ADDR_ADV_H"]
pub type R = crate::R<u32, super::PEER_SEC_ADDR_ADV_H>;
#[doc = "Writer for register PEER_SEC_ADDR_ADV_H"]
pub type W = crate::W<u32, super::PEER_SEC_ADDR_ADV_H>;
#[doc = "Register PEER_SEC_ADDR_ADV_H `reset()`'s with value 0"]
impl crate::ResetValue for super::PEER_SEC_ADDR_ADV_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEER_SEC_ADDR_H`"]
pub type PEER_SEC_ADDR_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PEER_SEC_ADDR_H`"]
pub struct PEER_SEC_ADDR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_SEC_ADDR_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit secondary address of the peer device for ADV_DIR. While doing directed Advertising in device privacy mode, if the peer device has shared its IRK, then the peer device RPA is written into the PEER_ADDR registers and the peer device identity address is written into this register. If the peer device has not shared its IRK, then the peer identity address is written into the PEER_ADDR registers and this register must be cleared."]
    #[inline(always)]
    pub fn peer_sec_addr_h(&self) -> PEER_SEC_ADDR_H_R {
        PEER_SEC_ADDR_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit secondary address of the peer device for ADV_DIR. While doing directed Advertising in device privacy mode, if the peer device has shared its IRK, then the peer device RPA is written into the PEER_ADDR registers and the peer device identity address is written into this register. If the peer device has not shared its IRK, then the peer identity address is written into the PEER_ADDR registers and this register must be cleared."]
    #[inline(always)]
    pub fn peer_sec_addr_h(&mut self) -> PEER_SEC_ADDR_H_W {
        PEER_SEC_ADDR_H_W { w: self }
    }
}
