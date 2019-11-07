#[doc = "Reader of register EP0_DR[%s]"]
pub type R = crate::R<u32, super::EP0_DR>;
#[doc = "Writer for register EP0_DR[%s]"]
pub type W = crate::W<u32, super::EP0_DR>;
#[doc = "Register EP0_DR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::EP0_DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_BYTE`"]
pub type DATA_BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BYTE`"]
pub struct DATA_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub fn data_byte(&self) -> DATA_BYTE_R {
        DATA_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub fn data_byte(&mut self) -> DATA_BYTE_W {
        DATA_BYTE_W { w: self }
    }
}
