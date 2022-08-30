#[doc = "Register `CLK_CAL_CNT1` reader"]
pub struct R(crate::R<CLK_CAL_CNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CAL_CNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CAL_CNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CAL_CNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CAL_CNT1` writer"]
pub struct W(crate::W<CLK_CAL_CNT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CAL_CNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_CAL_CNT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CAL_CNT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_COUNTER1` reader - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
pub type CAL_COUNTER1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAL_COUNTER1` writer - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
pub type CAL_COUNTER1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_CAL_CNT1_SPEC, u32, u32, 24, O>;
#[doc = "Field `CAL_COUNTER_DONE` reader - Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
pub type CAL_COUNTER_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:23 - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    pub fn cal_counter1(&self) -> CAL_COUNTER1_R {
        CAL_COUNTER1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 31 - Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
    #[inline(always)]
    pub fn cal_counter_done(&self) -> CAL_COUNTER_DONE_R {
        CAL_COUNTER_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    pub fn cal_counter1(&mut self) -> CAL_COUNTER1_W<0> {
        CAL_COUNTER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Calibration Counter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cal_cnt1](index.html) module"]
pub struct CLK_CAL_CNT1_SPEC;
impl crate::RegisterSpec for CLK_CAL_CNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cal_cnt1::R](R) reader structure"]
impl crate::Readable for CLK_CAL_CNT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cal_cnt1::W](W) writer structure"]
impl crate::Writable for CLK_CAL_CNT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CAL_CNT1 to value 0x8000_0000"]
impl crate::Resettable for CLK_CAL_CNT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
