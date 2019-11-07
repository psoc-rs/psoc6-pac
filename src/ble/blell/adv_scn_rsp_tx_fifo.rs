#[doc = "Reader of register ADV_SCN_RSP_TX_FIFO"]
pub type R = crate::R<u32, super::ADV_SCN_RSP_TX_FIFO>;
#[doc = "Writer for register ADV_SCN_RSP_TX_FIFO"]
pub type W = crate::W<u32, super::ADV_SCN_RSP_TX_FIFO>;
#[doc = "Register ADV_SCN_RSP_TX_FIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::ADV_SCN_RSP_TX_FIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCAN_RSP_DATA`"]
pub type SCAN_RSP_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCAN_RSP_DATA`"]
pub struct SCAN_RSP_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RSP_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IO mapped FIFO of depth 16 (2 byte wide), to store scan response data of maximum length 31 bytes for transmitting. Firmware writes consecutive words by writing to the same location. Note: ADV_TX_DATA_FIFO and ADV_SCN_RSP_TX_FIFO shares same physical FIFO of depth 32. 16 locations for each FIFO are allocated. Reading this location resets the FIFO pointer."]
    #[inline(always)]
    pub fn scan_rsp_data(&self) -> SCAN_RSP_DATA_R {
        SCAN_RSP_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IO mapped FIFO of depth 16 (2 byte wide), to store scan response data of maximum length 31 bytes for transmitting. Firmware writes consecutive words by writing to the same location. Note: ADV_TX_DATA_FIFO and ADV_SCN_RSP_TX_FIFO shares same physical FIFO of depth 32. 16 locations for each FIFO are allocated. Reading this location resets the FIFO pointer."]
    #[inline(always)]
    pub fn scan_rsp_data(&mut self) -> SCAN_RSP_DATA_W {
        SCAN_RSP_DATA_W { w: self }
    }
}
