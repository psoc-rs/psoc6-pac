#[doc = "Reader of register SCAN_WINDOW"]
pub type R = crate::R<u32, super::SCAN_WINDOW>;
#[doc = "Writer for register SCAN_WINDOW"]
pub type W = crate::W<u32, super::SCAN_WINDOW>;
#[doc = "Register SCAN_WINDOW `reset()`'s with value 0x10"]
impl crate::ResetValue for super::SCAN_WINDOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `SCAN_WINDOW`"]
pub type SCAN_WINDOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCAN_WINDOW`"]
pub struct SCAN_WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. (To prevent ADV RX - SCAN REQ TX - SCAN RSP RX spilling over across the scan window, when not in continuous scan, the scan window must be 2 slots less that the scan interval."]
    #[inline(always)]
    pub fn scan_window(&self) -> SCAN_WINDOW_R {
        SCAN_WINDOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. (To prevent ADV RX - SCAN REQ TX - SCAN RSP RX spilling over across the scan window, when not in continuous scan, the scan window must be 2 slots less that the scan interval."]
    #[inline(always)]
    pub fn scan_window(&mut self) -> SCAN_WINDOW_W {
        SCAN_WINDOW_W { w: self }
    }
}
