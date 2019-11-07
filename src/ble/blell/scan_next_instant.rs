#[doc = "Reader of register SCAN_NEXT_INSTANT"]
pub type R = crate::R<u32, super::SCAN_NEXT_INSTANT>;
#[doc = "Reader of field `NEXT_SCAN_INSTANT`"]
pub type NEXT_SCAN_INSTANT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the instant with respect to internal reference clock of resolution 625 us at which next scanning event begins."]
    #[inline(always)]
    pub fn next_scan_instant(&self) -> NEXT_SCAN_INSTANT_R {
        NEXT_SCAN_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
