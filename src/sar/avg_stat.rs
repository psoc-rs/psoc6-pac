#[doc = "Register `AVG_STAT` reader"]
pub struct R(crate::R<AVG_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVG_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVG_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVG_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUR_AVG_ACCU` reader - the current value of the averaging accumulator"]
pub type CUR_AVG_ACCU_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTRLV_BUSY` reader - If high then the SAR is in the middle of Interleaved averaging spanning several scans. While this bit is high the Firmware should not make any changes to the configuration registers otherwise some results may be incorrect. Note that the CUR_AVG_CNT status register below gives an indication how many more scans need to be done to complete the Interleaved averaging. This bit can be cleared by changing the averaging mode to ACCUNDUMP or by disabling the SAR."]
pub type INTRLV_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CUR_AVG_CNT` reader - the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
pub type CUR_AVG_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:19 - the current value of the averaging accumulator"]
    #[inline(always)]
    pub fn cur_avg_accu(&self) -> CUR_AVG_ACCU_R {
        CUR_AVG_ACCU_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 23 - If high then the SAR is in the middle of Interleaved averaging spanning several scans. While this bit is high the Firmware should not make any changes to the configuration registers otherwise some results may be incorrect. Note that the CUR_AVG_CNT status register below gives an indication how many more scans need to be done to complete the Interleaved averaging. This bit can be cleared by changing the averaging mode to ACCUNDUMP or by disabling the SAR."]
    #[inline(always)]
    pub fn intrlv_busy(&self) -> INTRLV_BUSY_R {
        INTRLV_BUSY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
    #[inline(always)]
    pub fn cur_avg_cnt(&self) -> CUR_AVG_CNT_R {
        CUR_AVG_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Current averaging status (for debug)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avg_stat](index.html) module"]
pub struct AVG_STAT_SPEC;
impl crate::RegisterSpec for AVG_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [avg_stat::R](R) reader structure"]
impl crate::Readable for AVG_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AVG_STAT to value 0"]
impl crate::Resettable for AVG_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
