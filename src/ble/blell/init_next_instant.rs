#[doc = "Reader of register INIT_NEXT_INSTANT"]
pub type R = crate::R<u32, super::INIT_NEXT_INSTANT>;
#[doc = "Reader of field `INIT_NEXT_INSTANT`"]
pub type INIT_NEXT_INSTANT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the instant with respect to internal reference clock of resolution 625 us at which next initiator scanning event begins."]
    #[inline(always)]
    pub fn init_next_instant(&self) -> INIT_NEXT_INSTANT_R {
        INIT_NEXT_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
