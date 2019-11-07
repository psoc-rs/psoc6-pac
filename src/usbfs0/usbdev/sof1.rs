#[doc = "Reader of register SOF1"]
pub type R = crate::R<u32, super::SOF1>;
#[doc = "Reader of field `FRAME_NUMBER_MSB`"]
pub type FRAME_NUMBER_MSB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - It has the upper 3 bits \\[10:8\\] of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number_msb(&self) -> FRAME_NUMBER_MSB_R {
        FRAME_NUMBER_MSB_R::new((self.bits & 0x07) as u8)
    }
}
