#[doc = "Reader of register PDU_ACCESS_ADDR_L_REGISTER"]
pub type R = crate::R<u32, super::PDU_ACCESS_ADDR_L_REGISTER>;
#[doc = "Writer for register PDU_ACCESS_ADDR_L_REGISTER"]
pub type W = crate::W<u32, super::PDU_ACCESS_ADDR_L_REGISTER>;
#[doc = "Register PDU_ACCESS_ADDR_L_REGISTER `reset()`'s with value 0"]
impl crate::ResetValue for super::PDU_ACCESS_ADDR_L_REGISTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDU_ACCESS_ADDRESS_LOWER_BITS`"]
pub type PDU_ACCESS_ADDRESS_LOWER_BITS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PDU_ACCESS_ADDRESS_LOWER_BITS`"]
pub struct PDU_ACCESS_ADDRESS_LOWER_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDU_ACCESS_ADDRESS_LOWER_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the lower 16 bits of the access address for each Link layer connection between any two devices."]
    #[inline(always)]
    pub fn pdu_access_address_lower_bits(&self) -> PDU_ACCESS_ADDRESS_LOWER_BITS_R {
        PDU_ACCESS_ADDRESS_LOWER_BITS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower 16 bits of the access address for each Link layer connection between any two devices."]
    #[inline(always)]
    pub fn pdu_access_address_lower_bits(&mut self) -> PDU_ACCESS_ADDRESS_LOWER_BITS_W {
        PDU_ACCESS_ADDRESS_LOWER_BITS_W { w: self }
    }
}
