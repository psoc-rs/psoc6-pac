#[doc = "Reader of register CONN_REQ_WORD2"]
pub type R = crate::R<u32, super::CONN_REQ_WORD2>;
#[doc = "Writer for register CONN_REQ_WORD2"]
pub type W = crate::W<u32, super::CONN_REQ_WORD2>;
#[doc = "Register CONN_REQ_WORD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_WINDOW_SIZE_VAL`"]
pub type TX_WINDOW_SIZE_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_WINDOW_SIZE_VAL`"]
pub struct TX_WINDOW_SIZE_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WINDOW_SIZE_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CRC_INIT_LOWER`"]
pub type CRC_INIT_LOWER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRC_INIT_LOWER`"]
pub struct CRC_INIT_LOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_LOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size_val(&self) -> TX_WINDOW_SIZE_VAL_R {
        TX_WINDOW_SIZE_VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field defines the lower byte \\[7:0\\] of the CRC initialization value."]
    #[inline(always)]
    pub fn crc_init_lower(&self) -> CRC_INIT_LOWER_R {
        CRC_INIT_LOWER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size_val(&mut self) -> TX_WINDOW_SIZE_VAL_W {
        TX_WINDOW_SIZE_VAL_W { w: self }
    }
    #[doc = "Bits 8:15 - This field defines the lower byte \\[7:0\\] of the CRC initialization value."]
    #[inline(always)]
    pub fn crc_init_lower(&mut self) -> CRC_INIT_LOWER_W {
        CRC_INIT_LOWER_W { w: self }
    }
}
