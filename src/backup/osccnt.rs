#[doc = "Reader of register OSCCNT"]
pub type R = crate::R<u32, super::OSCCNT>;
#[doc = "Reader of field `CNT32KHZ`"]
pub type CNT32KHZ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
    #[inline(always)]
    pub fn cnt32khz(&self) -> CNT32KHZ_R {
        CNT32KHZ_R::new((self.bits & 0xff) as u8)
    }
}
