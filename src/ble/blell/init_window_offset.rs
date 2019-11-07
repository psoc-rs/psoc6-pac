#[doc = "Reader of register INIT_WINDOW_OFFSET"]
pub type R = crate::R<u32, super::INIT_WINDOW_OFFSET>;
#[doc = "Reader of field `INIT_WINDOW_NI`"]
pub type INIT_WINDOW_NI_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Initiator Window offset captured at conn request. This value is in 1.25ms slots"]
    #[inline(always)]
    pub fn init_window_ni(&self) -> INIT_WINDOW_NI_R {
        INIT_WINDOW_NI_R::new((self.bits & 0xffff) as u16)
    }
}
