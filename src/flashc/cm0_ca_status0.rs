#[doc = "Reader of register CM0_CA_STATUS0"]
pub type R = crate::R<u32, super::CM0_CA_STATUS0>;
#[doc = "Reader of field `VALID16`"]
pub type VALID16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sixteen valid bits of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn valid16(&self) -> VALID16_R {
        VALID16_R::new((self.bits & 0xffff) as u16)
    }
}
