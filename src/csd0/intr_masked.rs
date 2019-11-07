#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_RES`"]
pub type ADC_RES_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn adc_res(&self) -> ADC_RES_R {
        ADC_RES_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
