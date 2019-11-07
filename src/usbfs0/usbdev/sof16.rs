#[doc = "Reader of register SOF16"]
pub type R = crate::R<u32, super::SOF16>;
#[doc = "Reader of field `FRAME_NUMBER16`"]
pub type FRAME_NUMBER16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - The frame number (11b)"]
    #[inline(always)]
    pub fn frame_number16(&self) -> FRAME_NUMBER16_R {
        FRAME_NUMBER16_R::new((self.bits & 0x07ff) as u16)
    }
}
