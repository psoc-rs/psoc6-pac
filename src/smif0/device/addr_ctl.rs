#[doc = "Reader of register ADDR_CTL"]
pub type R = crate::R<u32, super::ADDR_CTL>;
#[doc = "Writer for register ADDR_CTL"]
pub type W = crate::W<u32, super::ADDR_CTL>;
#[doc = "Register ADDR_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIZE2`"]
pub type SIZE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIZE2`"]
pub struct SIZE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DIV2`"]
pub type DIV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV2`"]
pub struct DIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn size2(&self) -> SIZE2_R {
        SIZE2_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn div2(&self) -> DIV2_R {
        DIV2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn size2(&mut self) -> SIZE2_W {
        SIZE2_W { w: self }
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn div2(&mut self) -> DIV2_W {
        DIV2_W { w: self }
    }
}
