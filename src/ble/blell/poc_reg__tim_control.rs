#[doc = "Reader of register POC_REG__TIM_CONTROL"]
pub type R = crate::R<u32, super::POC_REG__TIM_CONTROL>;
#[doc = "Writer for register POC_REG__TIM_CONTROL"]
pub type W = crate::W<u32, super::POC_REG__TIM_CONTROL>;
#[doc = "Register POC_REG__TIM_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::POC_REG__TIM_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BB_CLK_FREQ_MINUS_1`"]
pub type BB_CLK_FREQ_MINUS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BB_CLK_FREQ_MINUS_1`"]
pub struct BB_CLK_FREQ_MINUS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_CLK_FREQ_MINUS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `START_SLOT_OFFSET`"]
pub type START_SLOT_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `START_SLOT_OFFSET`"]
pub struct START_SLOT_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SLOT_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - LLH clock configuration. The clock frequency of the clock input to this design is configured in this register. This is used to derive a 1MHz clock."]
    #[inline(always)]
    pub fn bb_clk_freq_minus_1(&self) -> BB_CLK_FREQ_MINUS_1_R {
        BB_CLK_FREQ_MINUS_1_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - LLH clock configuration. The start of slot signal is offset by this value. If value is 0, the start of slot signal is generated at the 625us. The offset value is in terms of us."]
    #[inline(always)]
    pub fn start_slot_offset(&self) -> START_SLOT_OFFSET_R {
        START_SLOT_OFFSET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - LLH clock configuration. The clock frequency of the clock input to this design is configured in this register. This is used to derive a 1MHz clock."]
    #[inline(always)]
    pub fn bb_clk_freq_minus_1(&mut self) -> BB_CLK_FREQ_MINUS_1_W {
        BB_CLK_FREQ_MINUS_1_W { w: self }
    }
    #[doc = "Bits 8:11 - LLH clock configuration. The start of slot signal is offset by this value. If value is 0, the start of slot signal is generated at the 625us. The offset value is in terms of us."]
    #[inline(always)]
    pub fn start_slot_offset(&mut self) -> START_SLOT_OFFSET_W {
        START_SLOT_OFFSET_W { w: self }
    }
}
