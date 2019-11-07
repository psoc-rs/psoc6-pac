#[doc = "Reader of register CONN_REQ_WORD8"]
pub type R = crate::R<u32, super::CONN_REQ_WORD8>;
#[doc = "Writer for register CONN_REQ_WORD8"]
pub type W = crate::W<u32, super::CONN_REQ_WORD8>;
#[doc = "Register CONN_REQ_WORD8 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_CHANNELS_LOWER`"]
pub type DATA_CHANNELS_LOWER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA_CHANNELS_LOWER`"]
pub struct DATA_CHANNELS_LOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CHANNELS_LOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. 1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_lower(&self) -> DATA_CHANNELS_LOWER_R {
        DATA_CHANNELS_LOWER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. 1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_lower(&mut self) -> DATA_CHANNELS_LOWER_W {
        DATA_CHANNELS_LOWER_W { w: self }
    }
}
