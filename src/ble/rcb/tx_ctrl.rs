#[doc = "Reader of register TX_CTRL"]
pub type R = crate::R<u32, super::TX_CTRL>;
#[doc = "Writer for register TX_CTRL"]
pub type W = crate::W<u32, super::TX_CTRL>;
#[doc = "Register TX_CTRL `reset()`'s with value 0x21"]
impl crate::ResetValue for super::TX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x21
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
#[doc = "Reader of field `FIFO_RECONFIG`"]
pub type FIFO_RECONFIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_RECONFIG`"]
pub struct FIFO_RECONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RECONFIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TX_ENTRIES`"]
pub type TX_ENTRIES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_ENTRIES`"]
pub struct TX_ENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\] is data and \\[(ADDR_WIDTH+15):16\\] is used for address When MSB_FIRST = 0, then \\[15:0\\] is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting this bit, clears the FIFO and resets the pointer"]
    #[inline(always)]
    pub fn fifo_reconfig(&self) -> FIFO_RECONFIG_R {
        FIFO_RECONFIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
    #[inline(always)]
    pub fn tx_entries(&self) -> TX_ENTRIES_R {
        TX_ENTRIES_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\] is data and \\[(ADDR_WIDTH+15):16\\] is used for address When MSB_FIRST = 0, then \\[15:0\\] is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W { w: self }
    }
    #[doc = "Bit 1 - Setting this bit, clears the FIFO and resets the pointer"]
    #[inline(always)]
    pub fn fifo_reconfig(&mut self) -> FIFO_RECONFIG_W {
        FIFO_RECONFIG_W { w: self }
    }
    #[doc = "Bits 2:6 - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
    #[inline(always)]
    pub fn tx_entries(&mut self) -> TX_ENTRIES_W {
        TX_ENTRIES_W { w: self }
    }
}
