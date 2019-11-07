#[doc = "Reader of register INTR_CAUSE0"]
pub type R = crate::R<u32, super::INTR_CAUSE0>;
#[doc = "Reader of field `PORT_INT`"]
pub type PORT_INT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PRT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn port_int(&self) -> PORT_INT_R {
        PORT_INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
