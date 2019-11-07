#[doc = "Reader of register PEER_ADDR_INIT_L"]
pub type R = crate::R<u32, super::PEER_ADDR_INIT_L>;
#[doc = "Writer for register PEER_ADDR_INIT_L"]
pub type W = crate::W<u32, super::PEER_ADDR_INIT_L>;
#[doc = "Register PEER_ADDR_INIT_L `reset()`'s with value 0"]
impl crate::ResetValue for super::PEER_ADDR_INIT_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEER_ADDR_L`"]
pub type PEER_ADDR_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PEER_ADDR_L`"]
pub struct PEER_ADDR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit address of the peer device. This is used only in MMMS mode The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created."]
    #[inline(always)]
    pub fn peer_addr_l(&self) -> PEER_ADDR_L_R {
        PEER_ADDR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit address of the peer device. This is used only in MMMS mode The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created."]
    #[inline(always)]
    pub fn peer_addr_l(&mut self) -> PEER_ADDR_L_W {
        PEER_ADDR_L_W { w: self }
    }
}
