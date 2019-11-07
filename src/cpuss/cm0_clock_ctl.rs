#[doc = "Reader of register CM0_CLOCK_CTL"]
pub type R = crate::R<u32, super::CM0_CLOCK_CTL>;
#[doc = "Writer for register CM0_CLOCK_CTL"]
pub type W = crate::W<u32, super::CM0_CLOCK_CTL>;
#[doc = "Register CM0_CLOCK_CTL `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::CM0_CLOCK_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `SLOW_INT_DIV`"]
pub type SLOW_INT_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLOW_INT_DIV`"]
pub struct SLOW_INT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW_INT_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PERI_INT_DIV`"]
pub type PERI_INT_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERI_INT_DIV`"]
pub struct PERI_INT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_INT_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn slow_int_div(&self) -> SLOW_INT_DIV_R {
        SLOW_INT_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    pub fn peri_int_div(&self) -> PERI_INT_DIV_R {
        PERI_INT_DIV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn slow_int_div(&mut self) -> SLOW_INT_DIV_W {
        SLOW_INT_DIV_W { w: self }
    }
    #[doc = "Bits 24:31 - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    pub fn peri_int_div(&mut self) -> PERI_INT_DIV_W {
        PERI_INT_DIV_W { w: self }
    }
}
