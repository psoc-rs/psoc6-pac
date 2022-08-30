#[doc = "Register `RESULT_VAL1` reader"]
pub struct R(crate::R<RESULT_VAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_VAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_VAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_VAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt) this counter counts when csd_sense is high."]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BAD_CONVS` reader - Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
pub type BAD_CONVS_R = crate::FieldReader<u8, u8>;
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
#[doc = "Result CSD/CSX accumulation counter value 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result_val1](index.html) module"]
pub struct RESULT_VAL1_SPEC;
impl crate::RegisterSpec for RESULT_VAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result_val1::R](R) reader structure"]
impl crate::Readable for RESULT_VAL1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT_VAL1 to value 0"]
impl crate::Resettable for RESULT_VAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
