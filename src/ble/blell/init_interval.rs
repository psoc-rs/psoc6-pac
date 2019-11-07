#[doc = "Reader of register INIT_INTERVAL"]
pub type R = crate::R<u32, super::INIT_INTERVAL>;
#[doc = "Writer for register INIT_INTERVAL"]
pub type W = crate::W<u32, super::INIT_INTERVAL>;
#[doc = "Register INIT_INTERVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::INIT_INTERVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT_SCAN_INTERVAL`"]
pub type INIT_SCAN_INTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INIT_SCAN_INTERVAL`"]
pub struct INIT_SCAN_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_SCAN_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Initiator interval register. Firmware sets the initiator's scanning interval value to this regis-ter before issuing create connection command. Interval between two consecutive scanning events. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn init_scan_interval(&self) -> INIT_SCAN_INTERVAL_R {
        INIT_SCAN_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initiator interval register. Firmware sets the initiator's scanning interval value to this regis-ter before issuing create connection command. Interval between two consecutive scanning events. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn init_scan_interval(&mut self) -> INIT_SCAN_INTERVAL_W {
        INIT_SCAN_INTERVAL_W { w: self }
    }
}
