#[doc = "Reader of register TX_FIFO_CTRL"]
pub type R = crate::R<u32, super::TX_FIFO_CTRL>;
#[doc = "Writer for register TX_FIFO_CTRL"]
pub type W = crate::W<u32, super::TX_FIFO_CTRL>;
#[doc = "Register TX_FIFO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_FIFO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_TRIGGER_LEVEL`"]
pub type TX_TRIGGER_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_TRIGGER_LEVEL`"]
pub struct TX_TRIGGER_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TRIGGER_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CLEAR`"]
pub type CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    pub fn tx_trigger_level(&self) -> TX_TRIGGER_LEVEL_R {
        TX_TRIGGER_LEVEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    pub fn tx_trigger_level(&mut self) -> TX_TRIGGER_LEVEL_W {
        TX_TRIGGER_LEVEL_W { w: self }
    }
    #[doc = "Bit 16 - When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
}
