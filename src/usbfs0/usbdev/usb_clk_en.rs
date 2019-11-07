#[doc = "Reader of register USB_CLK_EN"]
pub type R = crate::R<u32, super::USB_CLK_EN>;
#[doc = "Writer for register USB_CLK_EN"]
pub type W = crate::W<u32, super::USB_CLK_EN>;
#[doc = "Register USB_CLK_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_CLK_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR_CLK_EN`"]
pub type CSR_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSR_CLK_EN`"]
pub struct CSR_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_CLK_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    pub fn csr_clk_en(&self) -> CSR_CLK_EN_R {
        CSR_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    pub fn csr_clk_en(&mut self) -> CSR_CLK_EN_W {
        CSR_CLK_EN_W { w: self }
    }
}
