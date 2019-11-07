#[doc = "Reader of register CM4_CLOCK_CTL"]
pub type R = crate::R<u32, super::CM4_CLOCK_CTL>;
#[doc = "Writer for register CM4_CLOCK_CTL"]
pub type W = crate::W<u32, super::CM4_CLOCK_CTL>;
#[doc = "Register CM4_CLOCK_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CM4_CLOCK_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAST_INT_DIV`"]
pub type FAST_INT_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FAST_INT_DIV`"]
pub struct FAST_INT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_INT_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn fast_int_div(&self) -> FAST_INT_DIV_R {
        FAST_INT_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\] (FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn fast_int_div(&mut self) -> FAST_INT_DIV_W {
        FAST_INT_DIV_W { w: self }
    }
}
