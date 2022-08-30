#[doc = "Register `INTR_CAUSE3` reader"]
pub struct R(crate::R<INTR_CAUSE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PORT_INT` reader - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
pub type PORT_INT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn port_int(&self) -> PORT_INT_R {
        PORT_INT_R::new(self.bits)
    }
}
#[doc = "Interrupt port cause register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause3](index.html) module"]
pub struct INTR_CAUSE3_SPEC;
impl crate::RegisterSpec for INTR_CAUSE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause3::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE3 to value 0"]
impl crate::Resettable for INTR_CAUSE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
