#[doc = "Reader of register SCAN_INTERVAL"]
pub type R = crate::R<u32, super::SCAN_INTERVAL>;
#[doc = "Writer for register SCAN_INTERVAL"]
pub type W = crate::W<u32, super::SCAN_INTERVAL>;
#[doc = "Register SCAN_INTERVAL `reset()`'s with value 0x10"]
impl crate::ResetValue for super::SCAN_INTERVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `SCAN_INTERVAL`"]
pub type SCAN_INTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCAN_INTERVAL`"]
pub struct SCAN_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Scan interval register. Interval between two consecutive scanning events. Firmware sets the scanning interval value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as SCAN_NI_TIMER when the SCAN_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn scan_interval(&self) -> SCAN_INTERVAL_R {
        SCAN_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan interval register. Interval between two consecutive scanning events. Firmware sets the scanning interval value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as SCAN_NI_TIMER when the SCAN_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn scan_interval(&mut self) -> SCAN_INTERVAL_W {
        SCAN_INTERVAL_W { w: self }
    }
}
