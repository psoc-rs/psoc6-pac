#[doc = "Reader of register ADC_RES"]
pub type R = crate::R<u32, super::ADC_RES>;
#[doc = "Reader of field `VIN_CNT`"]
pub type VIN_CNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `HSCMP_POL`"]
pub type HSCMP_POL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_OVERFLOW`"]
pub type ADC_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_ABORT`"]
pub type ADC_ABORT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Count to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn vin_cnt(&self) -> VIN_CNT_R {
        VIN_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
    #[inline(always)]
    pub fn hscmp_pol(&self) -> HSCMP_POL_R {
        HSCMP_POL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
    #[inline(always)]
    pub fn adc_overflow(&self) -> ADC_OVERFLOW_R {
        ADC_OVERFLOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
    #[inline(always)]
    pub fn adc_abort(&self) -> ADC_ABORT_R {
        ADC_ABORT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
