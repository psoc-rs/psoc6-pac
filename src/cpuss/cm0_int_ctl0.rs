#[doc = "Reader of register CM0_INT_CTL0"]
pub type R = crate::R<u32, super::CM0_INT_CTL0>;
#[doc = "Writer for register CM0_INT_CTL0"]
pub type W = crate::W<u32, super::CM0_INT_CTL0>;
#[doc = "Register CM0_INT_CTL0 `reset()`'s with value 0xf0f0_f0f0"]
impl crate::ResetValue for super::CM0_INT_CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf0f0_f0f0
    }
}
#[doc = "Reader of field `MUX0_SEL`"]
pub type MUX0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX0_SEL`"]
pub struct MUX0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MUX1_SEL`"]
pub type MUX1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX1_SEL`"]
pub struct MUX1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MUX2_SEL`"]
pub type MUX2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX2_SEL`"]
pub struct MUX2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MUX3_SEL`"]
pub type MUX3_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUX3_SEL`"]
pub struct MUX3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - System interrupt select for CPU interrupt source 0. If the field value is 240, no system interrupt is connected and the CPU interrupt source is always '0'/de-activated."]
    #[inline(always)]
    pub fn mux0_sel(&self) -> MUX0_SEL_R {
        MUX0_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - System interrupt select for CPU interrupt source 1."]
    #[inline(always)]
    pub fn mux1_sel(&self) -> MUX1_SEL_R {
        MUX1_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - System interrupt select for CPU interrupt source 2."]
    #[inline(always)]
    pub fn mux2_sel(&self) -> MUX2_SEL_R {
        MUX2_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - System interrupt select for CPU interrupt source 3."]
    #[inline(always)]
    pub fn mux3_sel(&self) -> MUX3_SEL_R {
        MUX3_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System interrupt select for CPU interrupt source 0. If the field value is 240, no system interrupt is connected and the CPU interrupt source is always '0'/de-activated."]
    #[inline(always)]
    pub fn mux0_sel(&mut self) -> MUX0_SEL_W {
        MUX0_SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - System interrupt select for CPU interrupt source 1."]
    #[inline(always)]
    pub fn mux1_sel(&mut self) -> MUX1_SEL_W {
        MUX1_SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - System interrupt select for CPU interrupt source 2."]
    #[inline(always)]
    pub fn mux2_sel(&mut self) -> MUX2_SEL_W {
        MUX2_SEL_W { w: self }
    }
    #[doc = "Bits 24:31 - System interrupt select for CPU interrupt source 3."]
    #[inline(always)]
    pub fn mux3_sel(&mut self) -> MUX3_SEL_W {
        MUX3_SEL_W { w: self }
    }
}
