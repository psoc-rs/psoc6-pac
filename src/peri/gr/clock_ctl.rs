#[doc = "Reader of register CLOCK_CTL"]
pub type R = crate::R<u32, super::CLOCK_CTL>;
#[doc = "Writer for register CLOCK_CTL"]
pub type W = crate::W<u32, super::CLOCK_CTL>;
#[doc = "Register CLOCK_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT8_DIV`"]
pub type INT8_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT8_DIV`"]
pub struct INT8_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INT8_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&self) -> INT8_DIV_R {
        INT8_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&mut self) -> INT8_DIV_W {
        INT8_DIV_W { w: self }
    }
}
