#[doc = "Reader of register CM4_CA_STATUS1"]
pub type R = crate::R<u32, super::CM4_CA_STATUS1>;
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See CM0_CA_STATUS1."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
