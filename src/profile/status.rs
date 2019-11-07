#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `WIN_ACTIVE`"]
pub type WIN_ACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates if the profiling time window is active. '0': Not active. '1': Active."]
    #[inline(always)]
    pub fn win_active(&self) -> WIN_ACTIVE_R {
        WIN_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
