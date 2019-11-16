#[doc = "Reader of register EP0_CNT"]
pub type R = crate::R<u32, super::EP0_CNT>;
#[doc = "Writer for register EP0_CNT"]
pub type W = crate::W<u32, super::EP0_CNT>;
#[doc = "Register EP0_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::EP0_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BYTE_COUNT`"]
pub type BYTE_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE_COUNT`"]
pub struct BYTE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_VALID_A {
    #[doc = "0: No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "1: Indicates a transaction ended with an ACK."]
    DATA_VALID = 1,
}
impl From<DATA_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_VALID`"]
pub type DATA_VALID_R = crate::R<bool, DATA_VALID_A>;
impl DATA_VALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_VALID_A {
        match self.bits {
            false => DATA_VALID_A::DATA_ERROR,
            true => DATA_VALID_A::DATA_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_ERROR`"]
    #[inline(always)]
    pub fn is_data_error(&self) -> bool {
        *self == DATA_VALID_A::DATA_ERROR
    }
    #[doc = "Checks if the value of the field is `DATA_VALID`"]
    #[inline(always)]
    pub fn is_data_valid(&self) -> bool {
        *self == DATA_VALID_A::DATA_VALID
    }
}
#[doc = "Write proxy for field `DATA_VALID`"]
pub struct DATA_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_VALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn data_error(self) -> &'a mut W {
        self.variant(DATA_VALID_A::DATA_ERROR)
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn data_valid(self) -> &'a mut W {
        self.variant(DATA_VALID_A::DATA_VALID)
    }
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
#[doc = "Reader of field `DATA_TOGGLE`"]
pub type DATA_TOGGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_TOGGLE`"]
pub struct DATA_TOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TOGGLE_W<'a> {
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
    #[doc = "Bits 0:3 - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn data_valid(&self) -> DATA_VALID_R {
        DATA_VALID_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn data_toggle(&self) -> DATA_TOGGLE_R {
        DATA_TOGGLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    pub fn byte_count(&mut self) -> BYTE_COUNT_W {
        BYTE_COUNT_W { w: self }
    }
    #[doc = "Bit 6 - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn data_valid(&mut self) -> DATA_VALID_W {
        DATA_VALID_W { w: self }
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn data_toggle(&mut self) -> DATA_TOGGLE_W {
        DATA_TOGGLE_W { w: self }
    }
}
