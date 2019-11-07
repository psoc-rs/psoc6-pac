#[doc = "Reader of register RX_CTRL"]
pub type R = crate::R<u32, super::RX_CTRL>;
#[doc = "Writer for register RX_CTRL"]
pub type W = crate::W<u32, super::RX_CTRL>;
#[doc = "Register RX_CTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MSB_FIRST`"]
pub type MSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSB_FIRST`"]
pub struct MSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_FIRST_W<'a> {
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
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\] is data and \\[(ADDR_WIDTH+15):16\\] is used for address When MSB_FIRST = 0, then \\[15:0\\] is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\] is data and \\[(ADDR_WIDTH+15):16\\] is used for address When MSB_FIRST = 0, then \\[15:0\\] is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W { w: self }
    }
}
