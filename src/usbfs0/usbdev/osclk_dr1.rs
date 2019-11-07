#[doc = "Reader of register OSCLK_DR1"]
pub type R = crate::R<u32, super::OSCLK_DR1>;
#[doc = "Reader of field `ADDER_MSB`"]
pub type ADDER_MSB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - These bits return the upper 7 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder_msb(&self) -> ADDER_MSB_R {
        ADDER_MSB_R::new((self.bits & 0x7f) as u8)
    }
}
