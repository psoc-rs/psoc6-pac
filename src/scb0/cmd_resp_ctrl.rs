#[doc = "Reader of register CMD_RESP_CTRL"]
pub type R = crate::R<u32, super::CMD_RESP_CTRL>;
#[doc = "Writer for register CMD_RESP_CTRL"]
pub type W = crate::W<u32, super::CMD_RESP_CTRL>;
#[doc = "Register CMD_RESP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_RESP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASE_RD_ADDR`"]
pub type BASE_RD_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE_RD_ADDR`"]
pub struct BASE_RD_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_RD_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `BASE_WR_ADDR`"]
pub type BASE_WR_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASE_WR_ADDR`"]
pub struct BASE_WR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_WR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&self) -> BASE_RD_ADDR_R {
        BASE_RD_ADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&self) -> BASE_WR_ADDR_R {
        BASE_WR_ADDR_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&mut self) -> BASE_RD_ADDR_W {
        BASE_RD_ADDR_W { w: self }
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&mut self) -> BASE_WR_ADDR_W {
        BASE_WR_ADDR_W { w: self }
    }
}
