#[doc = "Reader of register CLK_CAL_CNT1"]
pub type R = crate::R<u32, super::CLK_CAL_CNT1>;
#[doc = "Writer for register CLK_CAL_CNT1"]
pub type W = crate::W<u32, super::CLK_CAL_CNT1>;
#[doc = "Register CLK_CAL_CNT1 `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::CLK_CAL_CNT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `CAL_COUNTER1`"]
pub type CAL_COUNTER1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CAL_COUNTER1`"]
pub struct CAL_COUNTER1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_COUNTER1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `CAL_COUNTER_DONE`"]
pub type CAL_COUNTER_DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:23 - Down-counter clocked on fast DDFT output #0 (see TST_DDFT_FAST_CTL). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1."]
    #[inline(always)]
    pub fn cal_counter1(&self) -> CAL_COUNTER1_R {
        CAL_COUNTER1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 31 - Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
    #[inline(always)]
    pub fn cal_counter_done(&self) -> CAL_COUNTER_DONE_R {
        CAL_COUNTER_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Down-counter clocked on fast DDFT output #0 (see TST_DDFT_FAST_CTL). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1."]
    #[inline(always)]
    pub fn cal_counter1(&mut self) -> CAL_COUNTER1_W {
        CAL_COUNTER1_W { w: self }
    }
}
