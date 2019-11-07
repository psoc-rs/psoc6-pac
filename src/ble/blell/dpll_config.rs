#[doc = "Reader of register DPLL_CONFIG"]
pub type R = crate::R<u32, super::DPLL_CONFIG>;
#[doc = "Writer for register DPLL_CONFIG"]
pub type W = crate::W<u32, super::DPLL_CONFIG>;
#[doc = "Register DPLL_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPLL_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPLL_CORREL_CONFIG`"]
pub type DPLL_CORREL_CONFIG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DPLL_CORREL_CONFIG`"]
pub struct DPLL_CORREL_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL_CORREL_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If MXD_IF_OPTION is 0: Not used If CY_CORREL_EN is 1: \\[11:0\\] CY correl Access address compare mask for LSB 12 bits. Ideal value is 0xFFF \\[15:12\\] CY correl maximum number of allowed mismatched bits in access address. Ideal value is 0x0."]
    #[inline(always)]
    pub fn dpll_correl_config(&self) -> DPLL_CORREL_CONFIG_R {
        DPLL_CORREL_CONFIG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If MXD_IF_OPTION is 0: Not used If CY_CORREL_EN is 1: \\[11:0\\] CY correl Access address compare mask for LSB 12 bits. Ideal value is 0xFFF \\[15:12\\] CY correl maximum number of allowed mismatched bits in access address. Ideal value is 0x0."]
    #[inline(always)]
    pub fn dpll_correl_config(&mut self) -> DPLL_CORREL_CONFIG_W {
        DPLL_CORREL_CONFIG_W { w: self }
    }
}
