#[doc = "Reader of register CLOCK_CTL[%s]"]
pub type R = crate::R<u32, super::CLOCK_CTL>;
#[doc = "Writer for register CLOCK_CTL[%s]"]
pub type W = crate::W<u32, super::CLOCK_CTL>;
#[doc = "Register CLOCK_CTL[%s] `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CLOCK_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `DIV_SEL`"]
pub type DIV_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_SEL`"]
pub struct DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `TYPE_SEL`"]
pub type TYPE_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TYPE_SEL`"]
pub struct TYPE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '63' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub fn div_sel(&self) -> DIV_SEL_R {
        DIV_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn type_sel(&self) -> TYPE_SEL_R {
        TYPE_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Specifies one of the dividers of the divider type specified by TYPE_SEL. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out-of-phase dividers, spurious clock control signals may be generated for one 'clk_peri' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (DIV_SEL is '63' and TYPE_SEL is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub fn div_sel(&mut self) -> DIV_SEL_W {
        DIV_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn type_sel(&mut self) -> TYPE_SEL_W {
        TYPE_SEL_W { w: self }
    }
}
