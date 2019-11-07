#[doc = "Reader of register AVG_STAT"]
pub type R = crate::R<u32, super::AVG_STAT>;
#[doc = "Reader of field `CUR_AVG_ACCU`"]
pub type CUR_AVG_ACCU_R = crate::R<u32, u32>;
#[doc = "Reader of field `INTRLV_BUSY`"]
pub type INTRLV_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CUR_AVG_CNT`"]
pub type CUR_AVG_CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:19 - the current value of the averaging accumulator"]
    #[inline(always)]
    pub fn cur_avg_accu(&self) -> CUR_AVG_ACCU_R {
        CUR_AVG_ACCU_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 23 - If high then the SAR is in the middle of Interleaved averaging spanning several scans. While this bit is high the Firmware should not make any changes to the configuration registers otherwise some results may be incorrect. Note that the CUR_AVG_CNT status register below gives an indication how many more scans need to be done to complete the Interleaved averaging. This bit can be cleared by changing the averaging mode to ACCUNDUMP or by disabling the SAR."]
    #[inline(always)]
    pub fn intrlv_busy(&self) -> INTRLV_BUSY_R {
        INTRLV_BUSY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
    #[inline(always)]
    pub fn cur_avg_cnt(&self) -> CUR_AVG_CNT_R {
        CUR_AVG_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
