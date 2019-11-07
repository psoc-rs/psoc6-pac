#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x00f8_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00f8_0000
    }
}
#[doc = "Reader of field `TX_CLK_EDGE`"]
pub type TX_CLK_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CLK_EDGE`"]
pub struct TX_CLK_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_EDGE_W<'a> {
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
#[doc = "Reader of field `RX_CLK_EDGE`"]
pub type RX_CLK_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CLK_EDGE`"]
pub struct RX_CLK_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RX_CLK_SRC`"]
pub type RX_CLK_SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CLK_SRC`"]
pub struct RX_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLK_SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SCLK_CONTINUOUS`"]
pub type SCLK_CONTINUOUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_CONTINUOUS`"]
pub struct SCLK_CONTINUOUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_CONTINUOUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SSEL_POLARITY`"]
pub type SSEL_POLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSEL_POLARITY`"]
pub struct SSEL_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `LEAD`"]
pub type LEAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEAD`"]
pub struct LEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `LAG`"]
pub type LAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LAG`"]
pub struct LAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `DIV_ENABLED`"]
pub type DIV_ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV_ENABLED`"]
pub struct DIV_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 13)) | (((value as u32) & 0x3f) << 13);
        self.w
    }
}
#[doc = "Reader of field `ADDR_WIDTH`"]
pub type ADDR_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR_WIDTH`"]
pub struct ADDR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "Reader of field `DATA_WIDTH`"]
pub type DATA_WIDTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_WIDTH`"]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn tx_clk_edge(&self) -> TX_CLK_EDGE_R {
        TX_CLK_EDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rx_clk_edge(&self) -> RX_CLK_EDGE_R {
        RX_CLK_EDGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rx_clk_src(&self) -> RX_CLK_SRC_R {
        RX_CLK_SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SCLK_CONTINUOUS_R {
        SCLK_CONTINUOUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity(&self) -> SSEL_POLARITY_R {
        SSEL_POLARITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn lead(&self) -> LEAD_R {
        LEAD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn lag(&self) -> LAG_R {
        LAG_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn div_enabled(&self) -> DIV_ENABLED_R {
        DIV_ENABLED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:18 - N/A"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:22 - N/A"]
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn tx_clk_edge(&mut self) -> TX_CLK_EDGE_W {
        TX_CLK_EDGE_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rx_clk_edge(&mut self) -> RX_CLK_EDGE_W {
        RX_CLK_EDGE_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rx_clk_src(&mut self) -> RX_CLK_SRC_W {
        RX_CLK_SRC_W { w: self }
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn sclk_continuous(&mut self) -> SCLK_CONTINUOUS_W {
        SCLK_CONTINUOUS_W { w: self }
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity(&mut self) -> SSEL_POLARITY_W {
        SSEL_POLARITY_W { w: self }
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn lead(&mut self) -> LEAD_W {
        LEAD_W { w: self }
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn lag(&mut self) -> LAG_W {
        LAG_W { w: self }
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn div_enabled(&mut self) -> DIV_ENABLED_W {
        DIV_ENABLED_W { w: self }
    }
    #[doc = "Bits 13:18 - N/A"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bits 19:22 - N/A"]
    #[inline(always)]
    pub fn addr_width(&mut self) -> ADDR_WIDTH_W {
        ADDR_WIDTH_W { w: self }
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
