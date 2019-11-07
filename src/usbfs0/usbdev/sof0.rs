#[doc = "Reader of register SOF0"]
pub type R = crate::R<u32, super::SOF0>;
#[doc = "Reader of field `FRAME_NUMBER`"]
pub type FRAME_NUMBER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - It has the lower 8 bits \\[7:0\\] of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number(&self) -> FRAME_NUMBER_R {
        FRAME_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
