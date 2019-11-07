#[doc = "Reader of register ADV_NEXT_INSTANT"]
pub type R = crate::R<u32, super::ADV_NEXT_INSTANT>;
#[doc = "Reader of field `ADV_NEXT_INSTANT`"]
pub type ADV_NEXT_INSTANT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the next start of advertising event with reference to the internal reference clock."]
    #[inline(always)]
    pub fn adv_next_instant(&self) -> ADV_NEXT_INSTANT_R {
        ADV_NEXT_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
