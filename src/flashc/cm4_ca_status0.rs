#[doc = "Reader of register CM4_CA_STATUS0"]
pub type R = crate::R<u32, super::CM4_CA_STATUS0>;
#[doc = "Reader of field `VALID16`"]
pub type VALID16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - See CM0_CA_STATUS0."]
    #[inline(always)]
    pub fn valid16(&self) -> VALID16_R {
        VALID16_R::new((self.bits & 0xffff) as u16)
    }
}
