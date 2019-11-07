#[doc = "Reader of register OSCLK_DR16"]
pub type R = crate::R<u32, super::OSCLK_DR16>;
#[doc = "Reader of field `ADDER16`"]
pub type ADDER16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - These bits return the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder16(&self) -> ADDER16_R {
        ADDER16_R::new((self.bits & 0x7fff) as u16)
    }
}
