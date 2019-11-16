#[doc = "Reader of register CLK_TIMER_CTL"]
pub type R = crate::R<u32, super::CLK_TIMER_CTL>;
#[doc = "Writer for register CLK_TIMER_CTL"]
pub type W = crate::W<u32, super::CLK_TIMER_CTL>;
#[doc = "Register CLK_TIMER_CTL `reset()`'s with value 0x0007_0000"]
impl crate::ResetValue for super::CLK_TIMER_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_0000
    }
}
#[doc = "Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SEL_A {
    #[doc = "0: IMO - Internal Main Oscillator"]
    IMO = 0,
    #[doc = "1: Select the output of the predivider configured by TIMER_HF0_DIV."]
    HF0_DIV = 1,
}
impl From<TIMER_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER_SEL`"]
pub type TIMER_SEL_R = crate::R<bool, TIMER_SEL_A>;
impl TIMER_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SEL_A {
        match self.bits {
            false => TIMER_SEL_A::IMO,
            true => TIMER_SEL_A::HF0_DIV,
        }
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == TIMER_SEL_A::IMO
    }
    #[doc = "Checks if the value of the field is `HF0_DIV`"]
    #[inline(always)]
    pub fn is_hf0_div(&self) -> bool {
        *self == TIMER_SEL_A::HF0_DIV
    }
}
#[doc = "Write proxy for field `TIMER_SEL`"]
pub struct TIMER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(TIMER_SEL_A::IMO)
    }
    #[doc = "Select the output of the predivider configured by TIMER_HF0_DIV."]
    #[inline(always)]
    pub fn hf0_div(self) -> &'a mut W {
        self.variant(TIMER_SEL_A::HF0_DIV)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_HF0_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    NO_DIV = 0,
    #[doc = "1: Divide HFCLK0 by 2."]
    DIV_BY_2 = 1,
    #[doc = "2: Divide HFCLK0 by 4."]
    DIV_BY_4 = 2,
    #[doc = "3: Divide HFCLK0 by 8."]
    DIV_BY_8 = 3,
}
impl From<TIMER_HF0_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_HF0_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMER_HF0_DIV`"]
pub type TIMER_HF0_DIV_R = crate::R<u8, TIMER_HF0_DIV_A>;
impl TIMER_HF0_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_HF0_DIV_A {
        match self.bits {
            0 => TIMER_HF0_DIV_A::NO_DIV,
            1 => TIMER_HF0_DIV_A::DIV_BY_2,
            2 => TIMER_HF0_DIV_A::DIV_BY_4,
            3 => TIMER_HF0_DIV_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == TIMER_HF0_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == TIMER_HF0_DIV_A::DIV_BY_8
    }
}
#[doc = "Write proxy for field `TIMER_HF0_DIV`"]
pub struct TIMER_HF0_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_HF0_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_HF0_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::NO_DIV)
    }
    #[doc = "Divide HFCLK0 by 2."]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide HFCLK0 by 4."]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide HFCLK0 by 8."]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(TIMER_HF0_DIV_A::DIV_BY_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMER_DIV`"]
pub type TIMER_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER_DIV`"]
pub struct TIMER_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    pub fn timer_hf0_div(&self) -> TIMER_HF0_DIV_R {
        TIMER_HF0_DIV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    pub fn timer_div(&self) -> TIMER_DIV_R {
        TIMER_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W {
        TIMER_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    pub fn timer_hf0_div(&mut self) -> TIMER_HF0_DIV_W {
        TIMER_HF0_DIV_W { w: self }
    }
    #[doc = "Bits 16:23 - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    pub fn timer_div(&mut self) -> TIMER_DIV_W {
        TIMER_DIV_W { w: self }
    }
    #[doc = "Bit 31 - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
