#[doc = "Reader of register CRYPTO_CMD"]
pub type R = crate::R<u32, super::CRYPTO_CMD>;
#[doc = "Writer for register CRYPTO_CMD"]
pub type W = crate::W<u32, super::CRYPTO_CMD>;
#[doc = "Register CRYPTO_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYPTO_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
}
