#[doc = "Reader of register CM0_CA_STATUS1"]
pub type R = crate::R<u32, super::CM0_CA_STATUS1>;
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cache line address of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
