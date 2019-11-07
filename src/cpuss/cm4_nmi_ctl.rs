#[doc = "Reader of register CM4_NMI_CTL"]
pub type R = crate::R<u32, super::CM4_NMI_CTL>;
#[doc = "Writer for register CM4_NMI_CTL"]
pub type W = crate::W<u32, super::CM4_NMI_CTL>;
#[doc = "Register CM4_NMI_CTL `reset()`'s with value 0xf0"]
impl crate::ResetValue for super::CM4_NMI_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf0
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
impl R {
    #[doc = "Bits 0:7 - System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn mux0_sel(&self) -> MUX0_SEL_R {
        MUX0_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn mux0_sel(&mut self) -> MUX0_SEL_W {
        MUX0_SEL_W { w: self }
    }
}
