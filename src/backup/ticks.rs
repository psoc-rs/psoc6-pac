#[doc = "Reader of register TICKS"]
pub type R = crate::R<u32, super::TICKS>;
#[doc = "Reader of field `CNT128HZ`"]
pub type CNT128HZ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - 128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
    #[inline(always)]
    pub fn cnt128hz(&self) -> CNT128HZ_R {
        CNT128HZ_R::new((self.bits & 0x3f) as u8)
    }
}
