#[doc = "Reader of register STAT_SEQ"]
pub type R = crate::R<u32, super::STAT_SEQ>;
#[doc = "Reader of field `SEQ_STATE`"]
pub type SEQ_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADC_STATE`"]
pub type ADC_STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - CSD sequencer state"]
    #[inline(always)]
    pub fn seq_state(&self) -> SEQ_STATE_R {
        SEQ_STATE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
    #[inline(always)]
    pub fn adc_state(&self) -> ADC_STATE_R {
        ADC_STATE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
