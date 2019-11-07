#[doc = "Reader of register INIT_WINDOW"]
pub type R = crate::R<u32, super::INIT_WINDOW>;
#[doc = "Writer for register INIT_WINDOW"]
pub type W = crate::W<u32, super::INIT_WINDOW>;
#[doc = "Register INIT_WINDOW `reset()`'s with value 0"]
impl crate::ResetValue for super::INIT_WINDOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT_SCAN_WINDOW`"]
pub type INIT_SCAN_WINDOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INIT_SCAN_WINDOW`"]
pub struct INIT_SCAN_WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_SCAN_WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing create connection command. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn init_scan_window(&self) -> INIT_SCAN_WINDOW_R {
        INIT_SCAN_WINDOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing create connection command. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn init_scan_window(&mut self) -> INIT_SCAN_WINDOW_W {
        INIT_SCAN_WINDOW_W { w: self }
    }
}
