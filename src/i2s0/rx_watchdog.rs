#[doc = "Reader of register RX_WATCHDOG"]
pub type R = crate::R<u32, super::RX_WATCHDOG>;
#[doc = "Writer for register RX_WATCHDOG"]
pub type W = crate::W<u32, super::RX_WATCHDOG>;
#[doc = "Register RX_WATCHDOG `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_WATCHDOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WD_COUNTER`"]
pub type WD_COUNTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WD_COUNTER`"]
pub struct WD_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&self) -> WD_COUNTER_R {
        WD_COUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&mut self) -> WD_COUNTER_W {
        WD_COUNTER_W { w: self }
    }
}
