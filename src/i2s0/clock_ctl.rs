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
#[doc = "Reader of field `CLOCK_DIV`"]
pub type CLOCK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLOCK_DIV`"]
pub struct CLOCK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CLOCK_SEL`"]
pub type CLOCK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLOCK_SEL`"]
pub struct CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W {
        CLOCK_DIV_W { w: self }
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W {
        CLOCK_SEL_W { w: self }
    }
}
