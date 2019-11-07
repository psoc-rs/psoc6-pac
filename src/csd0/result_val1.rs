#[doc = "Reader of register RESULT_VAL1"]
pub type R = crate::R<u32, super::RESULT_VAL1>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Reader of field `BAD_CONVS`"]
pub type BAD_CONVS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt) this counter counts when csd_sense is high."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
    #[inline(always)]
    pub fn bad_convs(&self) -> BAD_CONVS_R {
        BAD_CONVS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
