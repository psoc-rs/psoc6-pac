#[doc = "Reader of register CLK_CAL_CNT2"]
pub type R = crate::R<u32, super::CLK_CAL_CNT2>;
#[doc = "Reader of field `CAL_COUNTER2`"]
pub type CAL_COUNTER2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Up-counter clocked on fast DDFT output #1 (see TST_DDFT_FAST_CTL). When CLK_CAL_CNT1.CAL_COUNTER_DONE==1, the counter is stopped and can be read by SW. Do not read this value unless CAL_COUNTER_DONE==1. The expected final value is related to the ratio of clock frequencies used for the two counters and the value loaded into counter 1: CLK_CAL_CNT2.COUNTER=(F_cnt2/F_cnt1)*(CLK_CAL_CNT1.COUNTER)"]
    #[inline(always)]
    pub fn cal_counter2(&self) -> CAL_COUNTER2_R {
        CAL_COUNTER2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
