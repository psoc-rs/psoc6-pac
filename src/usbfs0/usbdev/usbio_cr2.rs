#[doc = "Reader of register USBIO_CR2"]
pub type R = crate::R<u32, super::USBIO_CR2>;
#[doc = "Writer for register USBIO_CR2"]
pub type W = crate::W<u32, super::USBIO_CR2>;
#[doc = "Register USBIO_CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBIO_CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSVD_5_0`"]
pub type RSVD_5_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEST_PKT`"]
pub type TEST_PKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_PKT`"]
pub struct TEST_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_PKT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RSVD_7`"]
pub type RSVD_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVD_7`"]
pub struct RSVD_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5_0(&self) -> RSVD_5_0_R {
        RSVD_5_0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&self) -> TEST_PKT_R {
        TEST_PKT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&mut self) -> TEST_PKT_W {
        TEST_PKT_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&mut self) -> RSVD_7_W {
        RSVD_7_W { w: self }
    }
}
