#[doc = "Reader of register OSCLK_DR0"]
pub type R = crate::R<u32, super::OSCLK_DR0>;
#[doc = "Reader of field `ADDER`"]
pub type ADDER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits return the lower 8 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder(&self) -> ADDER_R {
        ADDER_R::new((self.bits & 0xff) as u8)
    }
}
