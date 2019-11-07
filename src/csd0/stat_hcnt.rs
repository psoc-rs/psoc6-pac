#[doc = "Reader of register STAT_HCNT"]
pub type R = crate::R<u32, super::STAT_HCNT>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of HSCMP counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
