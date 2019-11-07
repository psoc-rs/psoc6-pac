#[doc = "Reader of register HV_CTL"]
pub type R = crate::R<u32, super::HV_CTL>;
#[doc = "Writer for register HV_CTL"]
pub type W = crate::W<u32, super::HV_CTL>;
#[doc = "Register HV_CTL `reset()`'s with value 0x32"]
impl crate::ResetValue for super::HV_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x32
    }
}
#[doc = "Reader of field `TIMER_CLOCK_FREQ`"]
pub type TIMER_CLOCK_FREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER_CLOCK_FREQ`"]
pub struct TIMER_CLOCK_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_CLOCK_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
    #[inline(always)]
    pub fn timer_clock_freq(&self) -> TIMER_CLOCK_FREQ_R {
        TIMER_CLOCK_FREQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
    #[inline(always)]
    pub fn timer_clock_freq(&mut self) -> TIMER_CLOCK_FREQ_W {
        TIMER_CLOCK_FREQ_W { w: self }
    }
}
