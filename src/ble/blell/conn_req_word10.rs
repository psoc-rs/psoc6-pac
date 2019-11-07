#[doc = "Reader of register CONN_REQ_WORD10"]
pub type R = crate::R<u32, super::CONN_REQ_WORD10>;
#[doc = "Writer for register CONN_REQ_WORD10"]
pub type W = crate::W<u32, super::CONN_REQ_WORD10>;
#[doc = "Register CONN_REQ_WORD10 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_CHANNELS_UPPER`"]
pub type DATA_CHANNELS_UPPER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_CHANNELS_UPPER`"]
pub struct DATA_CHANNELS_UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CHANNELS_UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_upper(&self) -> DATA_CHANNELS_UPPER_R {
        DATA_CHANNELS_UPPER_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_upper(&mut self) -> DATA_CHANNELS_UPPER_W {
        DATA_CHANNELS_UPPER_W { w: self }
    }
}
